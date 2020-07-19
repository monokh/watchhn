use async_recursion::async_recursion;
use std::collections::hash_map::HashMap;
use std::{thread, time};

use crate::post::Post;

pub struct PostWatch {
    id: u32,
    hook_url: Option<String>,
    interval: u32,
    posts: HashMap<u32, Post>,
    initialised: bool,
}

impl PostWatch {
    pub fn new (id: u32, hook_url: Option<String>, interval: u32) -> PostWatch {
        return PostWatch {
            id,
            hook_url,
            interval,
            posts: HashMap::new(),
            initialised: false
        }
    }

    #[async_recursion]
    async fn process_post (&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        match reqwest::get(&url).await {
            Ok(resp) => {
                match resp.json::<Post>().await {
                    Ok(post) => {
                        let new = self.initialised && !self.posts.contains_key(&id);
                        self.posts.insert(post.id, post.clone());
                        if new {
                            println!("Found new comment: {}", post.context_link(self.id));
                            if self.hook_url.is_some() { self.send_to_slack(post.id).await? }
                        };
                        for id in post.kids {
                            self.process_post(id).await?;
                        }
                    },
                    Err(e) => println!("JSON Parse error: {}", e),
                }
            }
            Err(e) => println!("Request error: {}", e),
        };
        Ok(())
    }

    async fn send_to_slack(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let post = self.posts.get(&self.id).unwrap();
        let context = self.posts.get(&id).unwrap();
        let mut quote = context.text.clone();
        quote.truncate(100);
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
        }}", post.link(), post.title, quote, context.context_link(self.id));
        let client = reqwest::Client::new();
        client.post(self.hook_url.as_ref().unwrap())
            .body(body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn start (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Indexing {} ...", self.id);
        self.process_post(self.id).await?;
        self.initialised = true;
        println!("Indexing Done. Watching {} for changes...", self.id);
        loop {
            let duration = time::Duration::from_secs(self.interval as u64);
            thread::sleep(duration);
            self.process_post(self.id).await?;
        }
    }
}