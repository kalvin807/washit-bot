use crate::utils::redis_client::RedisClient;
use lazy_static::lazy_static;
use log::debug;
use redis::{Commands, Connection};
use regex::Regex;
use serenity::{model::prelude::Message, prelude::Context};

lazy_static! {
    static ref URL_RE: Regex =
        Regex::new(r"(http(s)?://)?(www.)?([a-zA-Z0-9])+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,5}(:[0-9]{1,5})?(/[^\s]*)?").unwrap();
}

fn has_url(url: String) -> Option<Vec<String>> {
    // Since most of the message do not have url, use is_match to process them faster
    // Trade off is that we need to process the message twice if it has url
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
    // https://discord.com/channels/{guild}/{channel}/{message}
    let value = format! {"https://discord.com/channels/{}/{}/{}", message.guild_id.unwrap(), message.channel_id, message.id};
    let ttl = 60 * 60 * 24; // 1 day
    conn.set_ex::<String, String, String>(key, value, ttl)
        .expect("msg store failed");
}

fn is_exist(conn: &mut Connection, url: &str) -> bool {
    conn.get::<&str, String>(url).is_ok()
}

pub async fn ming_handler(ctx: Context, new_message: Message) {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    let mut conn = client.get_connection().unwrap();
    let content = new_message.content.clone();

    let mut is_ming = false;
    match has_url(content) {
        None => {}
        Some(urls) => {
            for u in urls {
                debug!("URL found: {}", u);

                // Try find the URL in the redis
                // He minged if found
                if is_exist(&mut conn, &u) {
                    debug!("URL found in redis: {}", u);
                    is_ming = true;
                }

                // Store to redis
                store_message(&mut conn, u, &new_message);
            }
        }
    }

    if is_ming {
        new_message.react(&ctx.http, '💩').await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_url_0_url() {
        let testcase = "Hello world".to_string();

        let result = has_url(testcase);
        assert!(result.is_none());
    }

    #[test]
    fn test_has_url_1_url() {
        let testcase = "Hello world example.com".to_string();

        let result = has_url(testcase);
        assert_eq!(result.unwrap(), vec!["example.com"]);
    }

    #[test]
    fn test_has_url_2_url() {
        let testcase = "Hello world example.com example2.com".to_string();

        let result = has_url(testcase);
        assert_eq!(result.unwrap(), vec!["example.com", "example2.com"]);
    }
}
