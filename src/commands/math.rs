use crate::{Context, Error};

/// Multiply two numbers
#[poise::command(slash_command, prefix_command)]
pub async fn multiply(
    ctx: Context<'_>,
    #[description = "First number"] num1: f64,
    #[description = "Second number"] num2: f64,
) -> Result<(), Error> {
    let result = num1 * num2;
    ctx.say(format!("The result is: {}", result)).await?;
    Ok(())
}
