use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub by: String,
    pub id: u32,
    #[serde(default)]
    pub kids: Vec<u32>,
    #[serde(default)]
    pub parent: u32,
    #[serde(default)]
    pub text: String,
    pub time: u32,
    pub r#type: String
}

impl Post {
    pub fn link (&self) -> String {
        return format!("https://news.ycombinator.com/item?id={}", self.id);
    }

    pub fn context_link (&self, root: u32) -> String {
        return format!("https://news.ycombinator.com/item?id={}#{}", root, self.id);
    } 
}