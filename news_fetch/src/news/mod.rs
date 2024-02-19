pub mod article;
pub mod data;

pub use article::Article;
pub use data::NewsData;

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest;
use serde_json;
use uuid::Uuid;

use dto::{Language, NewsArticle};

/// Get [NewsData] from the given url.
pub async fn get_news_data(url: &str) -> Result<NewsData> {
    let data = reqwest::get(url).await?.text().await?;
    let data: NewsData = serde_json::from_str(&data)?;
    Ok(data)
}

pub fn news_data_to_news_articles(news_data: NewsData) -> Result<Vec<NewsArticle>> {
    let mut news_articles: Vec<NewsArticle> = Vec::new();

    let articles = match news_data.get_results() {
        None => return Err(anyhow::anyhow!("No articles found")),
        Some(articles) => articles,
    };

    for article in articles {
        let news_article = NewsArticle {
            id: Uuid::new_v4(),
            newsdata_id: match article.article_id {
                None => "".to_string(),
                Some(id) => id,
            },
            title: match article.title {
                None => "".to_string(),
                Some(title) => title,
            },
            link: match article.link {
                None => "".to_string(),
                Some(link) => link,
            },
            source_id: match article.source_id {
                None => "".to_string(),
                Some(source_id) => source_id,
            },
            keywords: match article.keywords {
                None => Vec::new(),
                Some(keywords) => keywords,
            },
            authors: match article.creator {
                None => Vec::new(),
                Some(authors) => authors,
            },
            summary: match article.description {
                None => "".to_string(),
                Some(summary) => summary,
            },
            publication_date: match article.pub_date {
                None => DateTime::from(Utc::now()),
                Some(pub_date) => DateTime::parse_from_str(&pub_date, "%Y-%m-%d %H:%M:%S")?,
            },
            content: match article.content {
                None => "".to_string(),
                Some(content) => content,
            },
            country: match article.country {
                None => vec!["".to_string()],
                Some(country) => country,
            },
            category: match article.category {
                None => vec!["".to_string()],
                Some(category) => category,
            },
            language: match article.language {
                None => Language("".to_string()),
                Some(language) => Language(language),
            },
            translations: HashMap::new(),
        };

        news_articles.push(news_article);
    }

    Ok(news_articles)
}
