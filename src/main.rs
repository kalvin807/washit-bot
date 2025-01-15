mod commands;
mod handlers;
mod libs;
mod utils;

use std::env;
use std::sync::Arc;

use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use crate::commands::{
    epl_standing::epl_standing,
    math::multiply,
    meta::ping,
    rw::{read, write},
};
use redis::Client as RedisClient;
use tracing::{error, info};

// Type aliases for convenience
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    redis_client: Arc<RedisClient>,
}

/// Show help menu
#[poise::command(slash_command, prefix_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Type ~help command for more info on a command.",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let redis_url = env::var("REDIS_DSL").expect("REDIS_DSL must be set");
    let redis_client = Arc::new(
        RedisClient::open(redis_url).expect("Failed to connect to Redis")
    );

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES 
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                help(),
                commands::imagine::imagine(),
                epl_standing(),
                multiply(),
                ping(),
                read(),
                write(),
                // TODO: Add other commands here
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            // Event handlers
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    if let poise::Event::Message { new_message } = event {
                        handlers::chat::chat_handler(ctx, new_message).await?;
                        handlers::ming::ming_handler(ctx, new_message).await?;
                    }
                    if let poise::Event::Ready { data_about_bot } = event {
                        info!("Connected as {}", data_about_bot.user.name);
                    }
                    Ok(())
                })
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    info!("Executing command {}", ctx.command().name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!("Executed command {}", ctx.command().name);
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    error!("Error in command: {:?}", error);
                    if let Err(e) = poise::builtins::on_error(error).await {
                        error!("Error while handling error: {}", e);
                    }
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    redis_client: redis_client.clone(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.unwrap().start().await {
        error!("Client error: {:?}", why);
    }
}
