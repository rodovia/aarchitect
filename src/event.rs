use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct MessageEvent {
    pub content: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub operation: String,
    pub event: Option<String>,

    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Message {
    content: String,
    created_at: i64,
    author: String
}

impl Message {
    pub fn new(content: String, created_at: i64, author: String) -> Self {
        Self {
            content,
            created_at,
            author
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_created_at(&self) -> &i64 {
        &self.created_at
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }
}
