use redis::Commands;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::RedisClient;

#[command]
pub async fn write(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    let key = args.single::<String>()?;
    let message = args.single::<String>()?;
    let mut conn = client.get_connection().unwrap();

    async {
        match conn.set::<String, String, String>(key, message) {
            Ok(_) => msg.reply(&ctx.http, "OK").await,
            Err(_) => msg.reply(&ctx.http, "ERR").await,
        }
        .expect("failed to send message");
    }
    .await;

    Ok(())
}

#[command]
pub async fn read(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let client = data.get::<RedisClient>().unwrap();
    let key = args.single::<String>()?;
    let mut conn = client.get_connection().unwrap();

    async {
        match conn.get::<String, String>(key) {
            Ok(result) => msg.reply(&ctx.http, format!("Result: {:?}", result)).await,
            Err(e) => msg.reply(&ctx.http, format!("Error: {:?}", e)).await,
        }
        .expect("failed to send message");
    }
    .await;

    Ok(())
}
