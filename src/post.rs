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