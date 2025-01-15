use poise::serenity_prelude::*;
use redis::Commands;

use crate::utils::redis_client::RedisClient;

const BOT_ID: u64 = 1042797425524183040;

fn is_tagging_me_only(mentions: &[User]) -> bool {
    mentions.len() == 1 && mentions.iter().all(|mention| mention.id.get() == BOT_ID)
}

pub async fn chat_handler(ctx: &Context, new_message: &Message) -> Result<(), crate::Error> {
    if !new_message.mentions.is_empty() && is_tagging_me_only(&new_message.mentions) {
        let data = ctx.data.write().await;
        let client = data.get::<RedisClient>().unwrap();
        let mut conn = client.get_connection().unwrap();
        let content = new_message.content.clone();
        let urls = content.split_whitespace().filter(|s| s.starts_with("http")).collect::<Vec<_>>();

        if !urls.is_empty() {
            for url in urls {
                if conn.get::<&str, String>(url).is_ok() {
                    new_message
                        .reply(ctx, format!("MING! {}", url))
                        .await
                        .unwrap();
                }
            }
        }
    }
    Ok(())
}
