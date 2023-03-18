use crate::utils::openai::*;
use serenity::{
    model::{prelude::Message, user::User},
    prelude::Context,
};

const BOT_ID: u64 = 1042057406525485096;

fn is_tagging_me_only(mentions: &[User]) -> bool {
    mentions
        .iter()
        .all(|mention| *mention.id.as_u64() == BOT_ID)
}

fn extract_prev_content(new_message: &Message) -> String {
    if new_message.referenced_message.is_some() {
        return new_message
            .referenced_message
            .as_ref()
            .unwrap()
            .content
            .clone();
    }
    // Return empty string if no previous message
    String::new()
}

pub async fn chat_handler(ctx: Context, new_message: Message) {
    // Check if the message is tagging the bot
    if !new_message.mentions.is_empty() && is_tagging_me_only(&new_message.mentions) {
        let content = new_message.content.clone();
        // Ask chat gpt
        let prev_content = extract_prev_content(&new_message);

        let response = ask_chat_gpt(content, prev_content).await;

        // Send the response
        new_message
            .reply_mention(&ctx, response)
            .await
            .expect("failed to send message");
    }
}
