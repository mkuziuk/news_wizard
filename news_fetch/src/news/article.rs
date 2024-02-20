use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Article {
    pub article_id: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub creator: Option<Vec<String>>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub pub_date: Option<String>,
    pub image_url: Option<String>,
    pub source_id: Option<String>,
    pub source_priority: Option<u32>,
    pub country: Option<Vec<String>>,
    pub category: Option<Vec<String>>,
    pub language: Option<String>,
    pub ai_tag: Option<String>,
    pub ai_region: Option<String>,
    pub sentiment: Option<String>,
    pub sentiement_stats: Option<String>,
    // ai_tag: Option<Vec<String>>,
    // ai_region: Option<Vec<String>>,
    // sentiment: Option<String>,
    // sentiement_stats: Option<SentimentStats>,
    pub next_page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SentimentStats {
    positive: f32,
    neutral: f32,
    negative: f32,
}
