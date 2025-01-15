use crate::{Context, Error};
use redis::Commands;

use crate::utils::redis_client::RedisClient;

/// Write a URL to the database
#[poise::command(slash_command, prefix_command)]
pub async fn write(
    ctx: Context<'_>,
    #[description = "URL to store"] url: String,
) -> Result<(), Error> {
    let data = ctx.data();
    let client = &data.redis_client;
    let mut conn = client.get_connection()?;
    
    match conn.set::<&str, &str, ()>(&url, "") {
        Ok(_) => {
            ctx.say(format!("Added {}", url)).await?;
        }
        Err(e) => {
            ctx.say(format!("Error: {}", e)).await?;
        }
    }
    
    Ok(())
}

/// Read a URL from the database
#[poise::command(slash_command, prefix_command)]
pub async fn read(
    ctx: Context<'_>,
    #[description = "URL to check"] url: String,
) -> Result<(), Error> {
    let data = ctx.data();
    let client = &data.redis_client;
    let mut conn = client.get_connection()?;
    
    match conn.get::<&str, String>(&url) {
        Ok(_) => {
            ctx.say(format!("Found {}", url)).await?;
        }
        Err(_) => {
            ctx.say(format!("Not found {}", url)).await?;
        }
    }
    
    Ok(())
}
