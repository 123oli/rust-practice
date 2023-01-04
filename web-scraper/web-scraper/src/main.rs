use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = Client::new()
        .get("https://www.rust-lang.org")
        .send().await?;

    let body = resp.text().await?;
    let document = Document::from(body.as_str());

    for node in document.find(Attr("id", "read-rust")) {
        for entry in node.find(Name("a")) {
            let title = entry.text();
            let url = entry.attr("href").unwrap();
            println!("This is: {} ({})", title, url);
        }
    }

    // println!("{:?}", document);


    Ok(())
}