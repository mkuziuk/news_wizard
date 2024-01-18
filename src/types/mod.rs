use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

enum ExternalId {
    Telegram(u64),
    Discord(u64),
}

struct User {
    id: Uuid,
    external_ids: Vec<ExternalId>,
    email: Option<String>,
    interests: Vec<String>,
}

struct NewsDataArticleId(String);
struct NewsDataSourceId(String);

struct Language(String);

struct NewsArticle {
    id: Uuid,
    newsdata_id: NewsDataArticleId,
    title: String,
    link: String,
    source_id: NewsDataSourceId,
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

struct TranslatedArticle {
    article_id: Uuid,
    language: Language,
    title: String,
    summary: String,
    content: String,
}
