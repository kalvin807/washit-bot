use crate::utils::redis_client::RedisClient;
use lazy_static::lazy_static;
use redis::{Commands, Connection};
use regex::Regex;
use serenity::{model::prelude::Message, prelude::Context};
use tracing::debug;

lazy_static! {
    static ref URL_RE: Regex =
        Regex::new(r"(http(s)?://)?(www.)?([a-zA-Z0-9])+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,5}(:[0-9]{1,5})?(/[^\s]*)?").unwrap();
}

const TTL_SECONDS: usize = 60 * 60 * 24; // 1 day

fn extract_urls(url: String) -> Option<Vec<String>> {
    if !URL_RE.is_match(&url) {
        return None;
    }

    let mut urls: Vec<String> = Vec::new();
    URL_RE.find_iter(&url).for_each(|matches| {
        urls.push(matches.as_str().to_string());
    });

    Some(urls)
}

fn store_message(conn: &mut Connection, key: String, message: &Message) {
    let value = format! {"https://discord.com/channels/{}/{}/{}", message.guild_id.unwrap(), message.channel_id, message.id};
    conn.set_ex::<String, String, String>(key, value, TTL_SECONDS)
        .expect("msg store failed");
}

fn url_exists(conn: &mut Connection, url: &str) -> bool {
    conn.get::<&str, String>(url).is_ok()
}

pub async fn ming_handler(ctx: &serenity::Context, new_message: &serenity::Message) -> Result<(), crate::Error> {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    let mut conn = client.get_connection().unwrap();
    let content = new_message.content.clone();

    let mut is_ming = false;
    if let Some(urls) = extract_urls(content) {
        for u in urls {
            debug!("URL found: {}", u);

            if url_exists(&mut conn, &u) {
                debug!("URL found in redis: {}", u);
                is_ming = true;
            }

            store_message(&mut conn, u, new_message);
        }
    }

    if is_ming {
        if let Err(e) = new_message.react(&ctx.http, '💩').await {
            // Handle the error, e.g., log an error message
            tracing::error!("Failed to react to message: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_urls_0_url() {
        let testcase = "Hello world".to_string();

        let result = extract_urls(testcase);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_urls_1_url() {
        let testcase = "Hello world example.com".to_string();

        let result = extract_urls(testcase);
        assert_eq!(result.unwrap(), vec!["example.com"]);
    }

    #[test]
    fn test_extract_urls_2_url() {
        let testcase = "Hello world example.com example2.com".to_string();

        let result = extract_urls(testcase);
        assert_eq!(result.unwrap(), vec!["example.com", "example2.com"]);
    }
}
    Ok(())
