use once_cell::sync::Lazy;
use redis::Commands;
use regex::Regex;
use poise::serenity_prelude::*;

use crate::utils::redis_client::RedisClient;

static URL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?://[^\s]+").unwrap());

fn is_valid_url(url: &str) -> bool {
    URL_RE.is_match(url)
}

fn extract_urls(url: &str) -> Vec<String> {
    let mut urls = Vec::new();
    URL_RE.find_iter(url).for_each(|matches| {
        urls.push(matches.as_str().to_string());
    });
    urls
}

fn url_exists(conn: &mut redis::Connection, url: &str) -> bool {
    conn.get::<&str, String>(url).is_ok()
}

pub async fn ming_handler(ctx: &Context, new_message: &Message) -> Result<(), crate::Error> {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    let mut conn = client.get_connection().unwrap();
    let content = new_message.content.clone();
    let urls = extract_urls(&content);

    for url in urls {
        if url_exists(&mut conn, &url) {
            new_message
                .reply(ctx, format!("MING! {}", url))
                .await
                .unwrap();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_urls_0_url() {
        let text = "Hello world";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_extract_urls_1_url() {
        let text = "Hello world https://example.com";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "https://example.com");
    }

    #[test]
    fn test_extract_urls_2_url() {
        let text = "Hello world https://example.com https://example2.com";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://example.com");
        assert_eq!(urls[1], "https://example2.com");
    }
}
