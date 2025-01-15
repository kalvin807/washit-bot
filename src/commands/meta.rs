use crate::{Context, Error};

/// Ping the bot
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping() {
        let ctx = Context::test();
        let result = ping(ctx).await;
        assert!(result.is_ok());
    }
}
