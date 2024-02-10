use super::results::Results;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NewsData {
    status: Option<String>,
    total_results: Option<u32>,
    results: Option<Vec<Results>>,
}
