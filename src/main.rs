mod commands;
mod handlers;
mod utils;
use dotenvy::dotenv;
use log::{debug, error, info};
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;

use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::rw::*;
use crate::handlers::chat::*;
use crate::handlers::ming::*;
use crate::utils::redis_client::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        debug!("Message received: {:?}", new_message.id);
        chat_handler(ctx.clone(), new_message.clone()).await;
        ming_handler(ctx, new_message).await;
    }
}

#[group]
#[commands(multiply, ping, write, read)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
