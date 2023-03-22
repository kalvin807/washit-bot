use crate::utils::{
    openai::{Message as ChatGPTMessage, *},
    redis_client::RedisClient,
};
use log::debug;
use redis::{Commands, Connection};
use serenity::{
    model::{
        id,
        prelude::{Message, MessageId},
        user::User,
    },
    prelude::Context,
};

const BOT_ID: u64 = 1044883401926250567;
const ROOT_PREFIX: &str = "root_";
const PTR_PREFIX: &str = "ptr_";

fn is_tagging_me_only(mentions: &[User]) -> bool {
    mentions
        .iter()
        .all(|mention| *mention.id.as_u64() == BOT_ID)
}

fn get_reference_id(msg: &Message) -> Option<MessageId> {
    if msg.referenced_message.is_some() {
        Some(msg.referenced_message.unwrap().id)
    } else {
        None
    }
}

fn get_root_msg(msg_id: &MessageId, conn: &Connection) -> Option<Box<[ChatGPTMessage]>> {
    let key = ROOT_PREFIX.to_string() + &msg_id.to_string();
    // Query redis
    let result = conn.get::<String, String>(key);
    if let Ok(serialized_value) = result {
        let result: [ChatGPTMessage] = serde_json::from_str(&serialized_value).unwrap();
        result
    }
    None
}

fn as_chat_gpt_msg(msg: &Message) -> ChatGPTMessage {
    ChatGPTMessage {
        role: if *msg.author.id.as_u64() == BOT_ID {
            Role::Assistant
        } else {
            Role::User
        }
        .as_string(),
        content: msg.content.clone(),
    }
}

pub async fn get_redis_connection(ctx: &Context) -> Connection {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    client.get_connection().unwrap()
}

pub async fn chat_handler(ctx: Context, new_message: Message) {
    let redis = get_redis_connection(&ctx).await;
    // Check if a message is tagging the bot only & ignore message without any mention
    if !new_message.mentions.is_empty() && is_tagging_me_only(&new_message.mentions) {
        let chat = get_chat(&new_message);
        let mut chat_messages: Vec<ChatGPTMessage> = chat.iter().map(as_chat_gpt_msg).collect();
        // ChatGPT
        let response = ask_chat_gpt(&mut chat_messages).await;
        new_message
            .reply_ping(&ctx, response)
            .await
            .expect("failed to send message");
    }
}
