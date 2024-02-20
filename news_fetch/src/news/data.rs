use super::article::Article;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct NewsData {
    status: Option<String>,
    total_results: Option<u32>,
    results: Option<Vec<Article>>,
}

impl NewsData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_results(&self) -> Option<&Vec<Article>> {
        self.results.as_ref()
    }
}
