use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ExternalId {
    Telegram(u64),
    Discord(u64),
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Language(String);

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    external_ids: Vec<ExternalId>,
    email: Option<String>,
    interests: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewsArticle {
    id: Uuid,
    newsdata_id: String,
    title: String,
    link: String,
    source_id: String,
    keywords: Vec<String>,
    author: String,
    summary: String,
    publication_date: DateTime<Utc>,
    content: String,
    country: String,
    category: String,
    language: Language,
    translations: HashMap<Language, TranslatedArticle>,
}

#[derive(Serialize, Deserialize)]
pub struct TranslatedArticle {
    id: Uuid,
    news_article_id: Uuid,
    language: String,
    title: String,
    summary: String,
    content: String,
}
