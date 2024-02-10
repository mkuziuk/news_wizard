use anyhow::Result;
use meilisearch_sdk::{client::*, search::*};

use dto::NewsArticle;

pub struct MeiliRepository {
    client: Client,
}

impl MeiliRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn add_articles(&self, new_articles: Vec<NewsArticle>) -> Result<()> {
        let index = self.client.index("news_articles");
        index.add_documents(&[new_articles], Some("id")).await?;

        Ok(())
    }

    pub async fn get_articles(&self, query: &str) -> Result<SearchResults<NewsArticle>> {
        let index = self.client.index("news_articles");
        let news_articles: SearchResults<NewsArticle> =
            SearchQuery::new(&index).with_query(query).execute().await?;

        Ok(news_articles)
    }

    pub async fn delete_arcticle(&self, id: &str) -> Result<()> {
        let index = self.client.index("news_articles");
        index.delete_document(id).await?;

        Ok(())
    }

    pub async fn update_article(&self, new_article: NewsArticle) -> Result<()> {
        let index = self.client.index("news_articles");
        index.add_or_update(&[new_article], Some("id")).await?;

        Ok(())
    }
}
