use anyhow::Result;
use reqwest::get;
use scraper::{Html, Selector};
use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;

pub async fn get_selector_text(url: &str, selector: &str) -> Result<Vec<String>> {
    let html = get(url).await?.text().await?;

    // let mut file = File::create("response.html")?;
    // file.write_all(&html.as_bytes())?;

    let html_doc = Html::parse_document(&html);
    let selector = Selector::parse(selector).unwrap();

    let mut result = vec![];

    for element in html_doc.select(&selector) {
        let elements = element.text().collect::<Vec<_>>();
        result.push(elements.into_iter().collect::<String>());
    }

    Ok(result)
}
//sasasaasa
pub async fn get_selectors_text(
    url: &str,
    selectors: Vec<&str>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let html = get(url).await?.text().await?;
    let fragment = Html::parse_fragment(&html);

    let selectors: Vec<Selector> = selectors
        .into_iter()
        .map(|s| Selector::parse(s).unwrap())
        .collect();

    let mut result = vec![];

    for selector in selectors {
        for element in fragment.select(&selector) {
            let elements = element.text().collect::<Vec<_>>();
            result.push(elements.into_iter().collect::<String>());
        }
    }

    Ok(result)
}
