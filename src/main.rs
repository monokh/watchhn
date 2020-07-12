use tokio;
use serde::{Deserialize};
use async_recursion::async_recursion;
use std::collections::hash_map::HashMap;
use std::{thread, time};
use std::env;


#[derive(Deserialize, Debug, Clone)]
struct Post {
    #[serde(default)]
    title: String,
    #[serde(default)]
    by: String,
    id: u32,
    #[serde(default)]
    kids: Vec<u32>,
    #[serde(default)]
    parent: u32,
    #[serde(default)]
    text: String,
    time: u32,
    r#type: String
}

struct Watch {
    id: u32,
    hook_url: String,
    posts: HashMap<u32, Post>,
    new: Vec<u32>,
    initialised: bool,
}

impl Watch {
    fn new (id: u32, hook_url: String) -> Watch {
        return Watch {
            id,
            hook_url,
            posts: HashMap::new(),
            new: vec![],
            initialised: false
        }
    }

    #[async_recursion]
    async fn process_post (&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let resp: Post;
        if self.initialised && !self.posts.contains_key(&id)  {
            self.new.push(id)
        }
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        resp = reqwest::get(&url)
            .await?
            .json::<Post>()
            .await?;
        self.posts.insert(resp.id, resp.clone());
        for id in resp.kids {
            self.process_post(id).await?;
        }
        Ok(())
    }

    async fn send_to_slack(&self) -> Result<(), Box<dyn std::error::Error>> {
        for id in &self.new {
            let post_link = format!("https://news.ycombinator.com/item?id={}", self.id);
            let post_title = self.posts.get(&self.id).unwrap().title.clone();
            let context = self.posts.get(&id).unwrap();
            let mut quote = context.text.clone();
            quote.truncate(100);
            let context_link = format!("https://news.ycombinator.com/item?id={}", id);
            let body = format!("{{
                \"blocks\": [
                    {{
                        \"type\": \"section\",
                        \"text\": {{
                            \"type\": \"mrkdwn\",
                            \"text\": \"<{}|*{}*>\n>{}...\n<{}|Context>\"
                        }}
                    }}
                ]
            }}", post_link, post_title, quote, context_link);
            let client = reqwest::Client::new();
            client.post(&self.hook_url)
                .body(body)
                .send()
                .await?;
        }
        Ok(())
    }

    async fn start (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Indexing {} ...", self.id);
        self.process_post(self.id).await?;
        self.initialised = true;
        println!("Indexing Done. Watching {} for changes...", self.id);
        loop {
            let duration = time::Duration::from_secs(10);
            thread::sleep(duration);
            self.process_post(self.id).await?;
            if self.new.len() > 0 { println!("New: {:#?}", self.new) };
            self.send_to_slack().await?;
            self.new = vec![];
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let id: u32 = args.get(1).unwrap().parse().unwrap();
    let hook = args.get(2).unwrap();
    let mut watch = Watch::new(id, hook.to_string());
    watch.start().await?;
    Ok(())
}
