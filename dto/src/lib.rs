use serde::{Deserialize, Serialize};

use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ExternalId {
    Telegram(u64),
    Discord(u64),
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
pub struct Language(pub String);

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    external_ids: Vec<ExternalId>,
    email: Option<String>,
    interests: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsArticle {
    pub id: Uuid,
    pub newsdata_id: String,
    pub title: String,
    pub link: String,
    pub source_id: String,
    pub keywords: Vec<String>,
    pub authors: Vec<String>,
    pub summary: String,
    pub publication_date: DateTime<FixedOffset>,
    pub content: String,
    pub country: Vec<String>,
    pub category: Vec<String>,
    pub language: Language,
    pub translations: HashMap<Language, TranslatedArticle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslatedArticle {
    id: Uuid,
    news_article_id: Uuid,
    language: String,
    title: String,
    summary: String,
    content: String,
}
