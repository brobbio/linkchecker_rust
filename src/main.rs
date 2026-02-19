use std::fs;

use futures::stream::{self, StreamExt};
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

fn extract_urls(text: &str) -> Vec<String> {
    let re = Regex::new(r"\[[^\]]*\]\(([^)]+)\)").unwrap();

    re.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}

async fn process_url(
    client: &Client,
    url: String,
) -> (String, Result<String, String>) {
    let response = client.get(&url).send().await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                return (
                    url,
                    Err(format!("HTTP {}", resp.status())),
                );
            }

            match resp.text().await {
                Ok(body) => {
                    let title = extract_title(&body);

                    match title {
                        Some(t) => (url, Ok(t)),
                        None => (url, Err("NO TITLE".into())),
                    }
                }
                Err(_) => (url, Err("BODY READ ERROR".into())),
            }
        }

        Err(e) => (url, Err(e.to_string())),
    }
}

fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").unwrap();

    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let input = fs::read_to_string("input.md")?;

    let urls = extract_urls(&input);

    println!("Found {} URLs", urls.len());

    let client = Client::new();

    let results = stream::iter(urls)
        .map(|url| {
            let client = client.clone();

            async move {
                process_url(&client, url).await
            }
        })
        .buffer_unordered(32)
        .collect::<Vec<_>>()
        .await;

    let mut output = String::new();

    for (url, result) in results {
        match result {
            Ok(title) => {
                output.push_str(&format!(
                    "[{}]({})\n",
                    title, url
                ));
            }

            Err(err) => {
                output.push_str(&format!(
                    "[{}]({})\n",
                    err, url
                ));
            }
        }
    }

    fs::write("output.md", output)?;

    println!("Done. Written to output.md");

    Ok(())
}