use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogEntry {
    pub text: String,
    pub class: String,
    pub is_html: bool,
    pub category: String,
    pub description: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct Article {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Clone, Default)]
pub struct BbsPost {
    pub id: usize,
    pub name: String,
    pub content: String,
}