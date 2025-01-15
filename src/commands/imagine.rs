use crate::{Context, Error};
use crate::utils::openai::generate_images;
use poise::CreateReply;

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
            let reply = CreateReply::default()
                .content(format!("{}\n ```{}```", response, prompt));
            ctx.send(reply).await?;
        }
        Err(e) => {
            let reply = CreateReply::default()
                .content(format!("OpenAI: {}", e))
                .ephemeral(true);
            ctx.send(reply).await?;
        }
    }
    
    Ok(())
}
