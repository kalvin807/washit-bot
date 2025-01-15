use crate::{Context, Error};
use redis::Commands;

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
    #[description = "URL to read"] url: String,
) -> Result<(), Error> {
    let data = ctx.data();
    let client = &data.redis_client;
    let mut conn = client.get_connection()?;

    match conn.get::<&str, String>(&url) {
        Ok(_) => {
            ctx.say(format!("Found {}", url)).await?;
        }
        Err(e) => {
            ctx.say(format!("Error: {}", e)).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_read() {
        let ctx = Context::test();
        
        // Test write
        let write_result = write(ctx.clone(), "http://example.com".to_string()).await;
        assert!(write_result.is_ok());

        // Test read
        let read_result = read(ctx, "http://example.com".to_string()).await;
        assert!(read_result.is_ok());
    }
}
