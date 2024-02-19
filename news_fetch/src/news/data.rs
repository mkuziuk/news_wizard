use super::article::Article;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NewsData {
    status: Option<String>,
    total_results: Option<u32>,
    results: Option<Vec<Article>>,
}

impl NewsData {
    pub fn get_results(self) -> Option<Vec<Article>> {
        self.results
    }
}
