pub mod news_data;
pub mod news_service;
use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let link = "https://www.nytimes.com/2024/02/07/us/politics/congress-ukraine-israel-aid.html";

    // let selectors = vec![
    //     "h1", "h2", "h3", "h4", "h5", "h6", "p", "a", "ul", "ol", "li", "table", "td", "th", "tr",
    //     "div", "span",
    // ];

    let ny_times_title = match news_service::get_selector_text(link, "h1").await {
        Ok(text) => text,
        Err(e) => {
            print!("{:?}", e);
            Vec::<String>::new()
        }
    };

    let ny_times_content = match news_service::get_selector_text(link, "p").await {
        Ok(text) => text,
        Err(e) => {
            print!("{:?}", e);
            Vec::<String>::new()
        }
    };

    println!("{:#?}", ny_times_title);
    println!("{:#?}", ny_times_content);
}
