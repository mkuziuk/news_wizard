use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Url {
    base_url: String,
    api_key: String,
    pub q: Option<String>,
    pub qln_title: Option<String>,
    pub qln_meta: Option<String>,
    pub timeframe: Option<String>,
    pub country: Option<String>,
    pub category: Option<String>,
    pub language: Option<String>,
    pub tag: Option<String>,
    pub sentiment: Option<String>,
    pub region: Option<String>,
    pub domain: Option<String>,
    pub domain_url: Option<String>,
    pub exclude_domain: Option<String>,
    pub priority_domain: Option<String>,
    pub timezone: Option<String>,
    pub full_content: Option<bool>,
    pub image: Option<bool>,
    pub video: Option<bool>,
    pub size: Option<u8>,
    pub page: Option<String>,
}

impl Url {
    pub fn new(base_url: String, api_key: String) -> Url {
        Url {
            base_url,
            api_key,
            ..Default::default()
        }
    }
}

pub fn get_url(url: Url) -> String {
    let mut url_str: String = url.base_url + "?apiKey=" + &url.api_key;

    if let Some(q) = url.q {
        url_str.push_str("&q=");
        url_str.push_str(&q);
    }
    if let Some(qln_title) = url.qln_title {
        url_str.push_str("&qlnTitle=");
        url_str.push_str(&qln_title);
    }
    if let Some(qln_meta) = url.qln_meta {
        url_str.push_str("&qlnMeta=");
        url_str.push_str(&qln_meta);
    }
    if let Some(timeframe) = url.timeframe {
        url_str.push_str("&timeframe=");
        url_str.push_str(&timeframe);
    }
    if let Some(country) = url.country {
        url_str.push_str("&country=");
        url_str.push_str(&country);
    }
    if let Some(category) = url.category {
        url_str.push_str("&category=");
        url_str.push_str(&category);
    }
    if let Some(language) = url.language {
        url_str.push_str("&language=");
        url_str.push_str(&language);
    }
    if let Some(tag) = url.tag {
        url_str.push_str("&tag=");
        url_str.push_str(&tag);
    }
    if let Some(sentiment) = url.sentiment {
        url_str.push_str("&sentiment=");
        url_str.push_str(&sentiment);
    }
    if let Some(region) = url.region {
        url_str.push_str("&region=");
        url_str.push_str(&region);
    }
    if let Some(domain) = url.domain {
        url_str.push_str("&domain=");
        url_str.push_str(&domain);
    }
    if let Some(domain_url) = url.domain_url {
        url_str.push_str("&domainurl=");
        url_str.push_str(&domain_url);
    }
    if let Some(exclude_domain) = url.exclude_domain {
        url_str.push_str("&excludedomain=");
        url_str.push_str(&exclude_domain);
    }
    if let Some(priority_domain) = url.priority_domain {
        url_str.push_str("&prioritydomain=");
        url_str.push_str(&priority_domain);
    }
    if let Some(timezone) = url.timezone {
        url_str.push_str("&timezone=");
        url_str.push_str(&timezone);
    }
    if let Some(full_content) = url.full_content {
        url_str.push_str("&full_content=");
        let full_content_num = if full_content { 1 } else { 0 };
        url_str.push_str(full_content_num.to_string().as_str());
    }
    if let Some(image) = url.image {
        url_str.push_str("&image=");
        let image_num = if image { 1 } else { 0 };
        url_str.push_str(image_num.to_string().as_str());
    }
    if let Some(video) = url.video {
        url_str.push_str("&video=");
        let video_num = if video { 1 } else { 0 };
        url_str.push_str(video_num.to_string().as_str());
    }
    if let Some(size) = url.size {
        url_str.push_str("&size=");
        url_str.push_str(size.to_string().as_str());
    }
    if let Some(page) = url.page {
        url_str.push_str("&page=");
        url_str.push_str(&page);
    }

    url_str
}
