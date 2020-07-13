use async_recursion::async_recursion;
use std::{thread, time};

use crate::post::Post;

pub struct KeywordWatch {
    keywords: Vec<String>,
    hook_url: String,
    checked: Vec<u32>
}

impl KeywordWatch {
    pub fn new (keywords: Vec<String>, hook_url: String) -> KeywordWatch {
        return KeywordWatch {
            keywords,
            hook_url,
            checked: vec![]
        }
    }

    #[async_recursion]
    async fn process_post (&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        match reqwest::get(&url).await {
            Ok(resp) => {
                match resp.json::<Post>().await {
                    Ok(post) => {
                        self.checked.push(post.id);
                        let contains = self.keywords.iter().find(|keyword| post.title.to_lowercase().contains(&keyword.to_lowercase()));
                        if contains.is_some() {
                            let found_keyword = contains.unwrap();
                            println!("Found keyword: {}, Post: {}", found_keyword, post.id);
                            self.send_to_slack(post).await?;
                        };
                    },
                    Err(e) => println!("JSON Parse error: {}", e),
                }
            }
            Err(e) => println!("Request error: {}", e),
        };
        Ok(())
    }

    async fn process_posts (&mut self, posts: Vec<u32>) -> Result<(), Box<dyn std::error::Error>> {
        for id in &posts[..40] {
            if !self.checked.contains(&id) {
                self.process_post(*id).await?;
            }
        }
        Ok(())
    }

    async fn send_to_slack(&self, post: Post) -> Result<(), Box<dyn std::error::Error>> {
        let post_link = format!("https://news.ycombinator.com/item?id={}", post.id);
        let post_title = post.title;
        let body = format!("{{
            \"blocks\": [
                {{
                    \"type\": \"section\",
                    \"text\": {{
                        \"type\": \"mrkdwn\",
                        \"text\": \"<{}|*{}*>\"
                    }}
                }}
            ]
        }}", post_link, post_title);
        let client = reqwest::Client::new();
        client.post(&self.hook_url)
            .body(body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn start (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Watching for keywords {} ...", self.keywords.join(", "));
        loop {
            let posts = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .await?
            .json::<Vec<u32>>()
            .await?;

            self.process_posts(posts).await?;
            
            let duration = time::Duration::from_secs(10);
            thread::sleep(duration);
        }
    }
}