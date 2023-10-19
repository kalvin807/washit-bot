use crate::utils::{openai::*, redis_client::RedisClient};
use redis::{Commands, RedisResult};
use serenity::{
    model::{prelude::Message, user::User},
    prelude::Context,
};
use tracing::{debug, error};

pub const BOT_ID: u64 = 1042057406525485096;

fn handle_new_message(
    conn: &mut redis::Connection,
    message: &Message,
) -> RedisResult<Vec<Message>> {
    let key = format!("chat_{}", message.id);
    let value = serde_json::to_string::<Message>(message).unwrap();
    conn.lpush::<String, String, ()>(key.clone(), value)?;
    debug!("created new history: {}", key);

    let msg_key = format!("msg_{}", message.id);
    conn.set::<String, String, ()>(msg_key, key)?;
    Ok(vec![])
}

fn handle_reply_message(
    conn: &mut redis::Connection,
    message: &Message,
) -> RedisResult<Vec<Message>> {
    let referenced_message = message.referenced_message.as_ref().unwrap();
    let pointer_key = format!("msg_{}", referenced_message.id);
    let chat_history_key: String = match conn.get(&pointer_key) {
        Ok(value) => value,
        Err(_) => {
            return handle_new_message(conn, message);
        }
    };

    let history_raw = conn.lrange::<String, Vec<String>>(chat_history_key.clone(), 0, -1);
    let history: Vec<Message> = match history_raw {
        Ok(history_raw) => history_raw
            .iter()
            .map(|message| serde_json::from_str::<Message>(message).unwrap())
            .collect(),
        Err(_) => {
            return handle_new_message(conn, message);
        }
    };

    let value = serde_json::to_string::<Message>(message).unwrap();
    conn.rpush::<String, String, ()>(chat_history_key.clone(), value)?;
    debug!("continued at history: {}", chat_history_key);

    let msg_key = format!("msg_{}", message.id);
    conn.set::<String, String, ()>(msg_key, chat_history_key)?;

    // Return as a RedisResult
    Ok(history)
}

fn process_message(conn: &mut redis::Connection, message: Message) -> RedisResult<Vec<Message>> {
    match message.referenced_message {
        Some(_) => handle_reply_message(conn, &message),
        None => handle_new_message(conn, &message),
    }
}

fn is_tagging_me_only(mentions: &[User]) -> bool {
    mentions
        .iter()
        .all(|mention| *mention.id.as_u64() == BOT_ID)
}

async fn send_response_and_update_history(
    ctx: &Context,
    message: &Message,
    response: String,
    conn: &mut redis::Connection,
) {
    match message.reply(ctx, response).await {
        Ok(bot_response) => {
            handle_reply_message(conn, &bot_response).unwrap();
        }
        Err(e) => {
            error!("Failed to send message: {}", e);
            message.reply(ctx, format!("Discord: {}", e)).await.unwrap();
        }
    }
}

pub async fn chat_handler(ctx: &Context, new_message: &Message) {
    if !new_message.mentions.is_empty() && is_tagging_me_only(&new_message.mentions) {
        let data = ctx.data.write().await;
        let client = data.get::<RedisClient>().unwrap();
        let mut conn = client.get_connection().unwrap();

        let content = new_message.content.clone();
        let history = process_message(&mut conn, new_message.clone()).unwrap_or_default();
        let response = ask_chat_gpt(content, history).await;

        send_response_and_update_history(ctx, new_message, response, &mut conn).await;
    }
}
