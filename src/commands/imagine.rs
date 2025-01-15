use crate::utils::openai::generate_images;
use crate::{Context, Error};

/// Ask AI to draw an image based on your prompt
#[poise::command(slash_command)]
pub async fn imagine(
    ctx: Context<'_>,
    #[description = "The instruction"] prompt: String,
) -> Result<(), Error> {
    // Defer the response since image generation might take a while
    ctx.defer().await?;
    
    let response = generate_images(&prompt).await.map(|urls| urls.join("\n"));
    match response {
        Ok(response) => {
            ctx.send(|m: &mut poise::CreateReply<'_>| {
                m.content(format!("{}\n ```{}```", response, prompt))
            }).await?;
        }
        Err(e) => {
            ctx.send(|m: &mut poise::CreateReply<'_>| {
                m.content(format!("OpenAI: {}", e))
                 .ephemeral(true)
            }).await?;
        }
    }
    
    Ok(())
}
