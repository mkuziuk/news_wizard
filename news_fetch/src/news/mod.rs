pub mod data;
pub mod results;

pub use data::NewsData;
pub use results::Results;

use anyhow::Result;
use reqwest;
use serde_json;

pub async fn get_news_data(url: &str) -> Result<NewsData> {
    let data = reqwest::get(url).await?.text().await?;
    let data: NewsData = serde_json::from_str(&data)?;
    Ok(data)
}
