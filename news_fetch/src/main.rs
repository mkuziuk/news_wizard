use meilisearch_sdk::client::Client;
use std::env;

use dto;
use meilisearch_api::MeiliRepository;

pub mod news;
pub mod url;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let url = "https://newsdata.io/api/1/news";
    let api_key = "pub_35440fe584a4c4c1e0428c4c5454dfedf14bc";

    let mut url = url::Url::new(url.to_string(), api_key.to_string());
    url.size = Some(1);
    let url = url::get_url(url);

    let news = news::get_news_data(&url).await.unwrap();

    println!("{:#?}", news);

    let client = Client::new("http://localhost:7700", Some("super_cool_key"));
    let meili_repository = MeiliRepository::new(client);

    // let results = match news::news_data_to_news_articles(news) {
    //     Ok(results) => results,
    //     Err(e) => {
    //         println!("{:#?}", e);
    //     }
    // };
}

#[cfg(test)]
mod tests {
    use super::url::{self, Url};

    /// Test get_url function.
    #[test]
    fn test_get_url() {
        let base_url = "https://newsdata.io/api/1/news".to_string();
        let api_key = "test_key".to_string();
        let mut url = Url::new(base_url, api_key);

        url.q = Some("test_q".to_string());
        url.country = Some("test_country".to_string());
        url.language = Some("test_language".to_string());
        url.category = Some("test_category".to_string());

        let expected_url = "https://newsdata.io/api/1/news?apiKey=test_key&q=test_q&country=test_country&category=test_category&language=test_language";
        assert_eq!(url::get_url(url), expected_url);
    }
}
