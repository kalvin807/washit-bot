use crate::RedisClient;
use lazy_static::lazy_static;
use redis::{Commands, Connection};
use regex::Regex;
use serenity::{model::prelude::Message, prelude::Context};
use tracing::debug;

lazy_static! {
    static ref URL_RE: Regex =
        Regex::new(r"(http(s)?://)?(www.)?([a-zA-Z0-9])+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,5}(:[0-9]{1,5})?(/[^\s]*)?").unwrap();
}

fn has_url(url: String) -> Option<String> {
    URL_RE
        .find(&url)
        .map(|matches| matches.as_str().to_string())
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
        Some(url) => {
            debug!("URL found: {}", url);

            // Try find the URL in the redis
            // He minged if found
            if is_exist(&mut conn, &url) {
                debug!("URL found in redis: {}", url);
                is_ming = true;
            }

            // Store to redis
            store_message(&mut conn, url, &new_message);
        }
    }

    if is_ming {
        new_message.react(&ctx.http, '💩').await.unwrap();
    }
}
