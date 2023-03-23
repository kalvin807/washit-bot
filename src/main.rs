mod commands;
mod handlers;
mod utils;

use std::env;

use dotenvy::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use tracing::{debug, error, info};

use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::rw::*;
use crate::handlers::chat::*;
use crate::handlers::ming::*;
use crate::utils::redis_client::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::imagine::register(command)
        })
        .await;

        info!("global slash command: {:#?}", guild_command)
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        debug!("Message received: {:?}", new_message.id);
        chat_handler(ctx.clone(), new_message.clone()).await;
        ming_handler(ctx, new_message).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            command.defer(&ctx).await.unwrap();
            let content = match command.data.name.as_str() {
                "imagine" => commands::imagine::run(&command.data.options).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| response.content(content))
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[group]
#[commands(multiply, ping, write, read)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let redis_url = env::var("REDIS_DSL").expect("REDIS_DSL must be set");
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<RedisClient>(redis_client);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
