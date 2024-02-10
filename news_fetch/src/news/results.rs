use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Results {
    article_id: Option<String>,
    title: Option<String>,
    link: Option<String>,
    keywords: Option<Vec<String>>,
    creator: Option<Vec<String>>,
    video_url: Option<String>,
    description: Option<String>,
    content: Option<String>,
    pub_date: Option<String>,
    image_url: Option<String>,
    source_id: Option<String>,
    source_priority: Option<u32>,
    country: Option<Vec<String>>,
    category: Option<Vec<String>>,
    language: Option<String>,
    ai_tag: Option<String>,
    ai_region: Option<String>,
    sentiment: Option<String>,
    sentiement_stats: Option<String>,
    // ai_tag: Option<Vec<String>>,
    // ai_region: Option<Vec<String>>,
    // sentiment: Option<String>,
    // sentiement_stats: Option<SentimentStats>,
    next_page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SentimentStats {
    positive: f32,
    neutral: f32,
    negative: f32,
}
