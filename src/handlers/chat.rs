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

pub async fn chat_handler(ctx: Context, new_message: Message) {
    // Check if the message is tagging the bot
    if is_tagging_me_only(&new_message.mentions) {
        let content = new_message.content.clone();
        // Ask chat gpt
        let response = ask_chat_gpt(content).await;

        // Send the response
        new_message
            .reply_mention(&ctx, response)
            .await
            .expect("failed to send message");
    }
}
