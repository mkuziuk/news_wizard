use anyhow::Result;
use meilisearch_sdk::{client::*, search::*};

use dto::NewsArticle;
use uuid::Uuid;

pub struct MeiliRepository {
    client: Client,
}

impl MeiliRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn add_or_update_article(&self, new_article: NewsArticle) -> Result<()> {
        let index = self.client.index("news_articles");
        index.add_or_update(&[new_article], Some("id")).await?;

        Ok(())
    }

    pub async fn add_or_replace_articles(&self, new_articles: Vec<NewsArticle>) -> Result<()> {
        let index = self.client.index("news_articles");
        index.add_or_replace(&new_articles, Some("id")).await?;

        Ok(())
    }

    pub async fn get_articles(&self, query: &str) -> Result<Vec<NewsArticle>> {
        let index = self.client.index("news_articles");
        let news_articles = SearchQuery::new(&index)
            .with_query(query)
            .execute::<NewsArticle>()
            .await?
            .hits
            .into_iter()
            .map(|res| res.result)
            .collect();

        Ok(news_articles)
    }

    pub async fn delete_article(&self, id: &Uuid) -> Result<()> {
        let index = self.client.index("news_articles");
        index.delete_document(id).await?;

        Ok(())
    }

    pub async fn create_dump(&self) -> Result<()> {
        self.client.create_dump().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use dto::{ExternalArticleId, Language, NewsArticle};
    use meilisearch_sdk::client::Client;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_or_update_article() {
        let client = Client::new("http://localhost:7700", Some("super_cool_key"));
        let meili_repository = MeiliRepository::new(client);

        let new_article = NewsArticle {
            id: Uuid::new_v4(),
            api_id: ExternalArticleId::Empty,
            title: String::from("Updated News!"),
            link: String::from("https://example.com/updated-article"),
            source_id: String::from("source987"),
            keywords: vec![String::from("update"), String::from("revision")],
            authors: vec![String::from("Jane Smith")],
            summary: String::from("Summary of the updated article..."),
            publication_date: DateTime::parse_from_rfc3339("2022-01-01T00:00:00+00:00").unwrap(),
            content: String::from("Full content of the updated article..."),
            country: vec![String::from("US")],
            category: vec![String::from("Technology")],
            language: Language(String::from("en")),
            tags: Vec::new(),
            translations: HashMap::new(),
        };

        let result = meili_repository.add_or_update_article(new_article).await;
        assert!(result.is_ok());

        // Verify that the article was added or updated in the index
        let query = "Updated News!";
        let search_results = meili_repository.get_articles(query).await;
        assert!(search_results.is_ok());
        let articles = search_results.unwrap();

        println!("Articles: {:?}", articles);

        // let delete_arcticle = meili_repository.delete_article(&articles[0].id).await;
        // assert!(delete_arcticle.is_ok());

        // Additional checks can be done to verify the properties of the added or updated article
    }

    #[tokio::test]
    async fn test_add_articles() {
        let client = Client::new("http://localhost:7700", Some("super_cool_key"));
        let meili_repository = MeiliRepository::new(client);

        let articles = vec![
            NewsArticle {
                id: Uuid::new_v4(),
                api_id: ExternalArticleId::NewsDataIo("12asas3".to_string()),
                title: String::from("Breaking News!"),
                link: String::from("https://example.com/article"),
                source_id: String::from("source123"),
                keywords: vec![String::from("breaking"), String::from("urgent")],
                authors: vec![String::from("John Doe")],
                summary: String::from("Summary of the article..."),
                publication_date: DateTime::parse_from_rfc3339("2022-01-01T00:00:00+00:00")
                    .unwrap(),
                content: String::from("Full content of the article..."),
                country: vec![String::from("US")],
                category: vec![String::from("Politics")],
                language: Language(String::from("en")),
                tags: Vec::new(),
                translations: HashMap::new(),
            },
            NewsArticle {
                id: Uuid::new_v4(),
                api_id: ExternalArticleId::NewsDataIo("12as1wwas3".to_string()),
                title: String::from("Breaking News 22222!"),
                link: String::from("https://example.com/article2222"),
                source_id: String::from("source321"),
                keywords: vec![String::from("breaking222"), String::from("urgent2222")],
                authors: vec![String::from("John Doe")],
                summary: String::from("Summary of the article..."),
                publication_date: DateTime::parse_from_rfc3339("2022-01-01T00:00:00+00:00")
                    .unwrap(),
                content: String::from("Full content of the article..."),
                country: vec![String::from("US")],
                category: vec![String::from("Politics")],
                language: Language(String::from("en")),
                tags: Vec::new(),
                translations: HashMap::new(),
            }, // Add more NewsArticle instances as needed
        ];

        let result = meili_repository.add_or_replace_articles(articles).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_articles() {
        let client = Client::new("http://localhost:7700", Some("super_cool_key"));
        let meili_repository = MeiliRepository::new(client);

        let articles = meili_repository.get_articles("").await;

        println!("Articles: {:#?}", articles);

        assert!(articles.is_ok());
    }
}
