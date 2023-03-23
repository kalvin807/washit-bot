use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::utils::openai::generate_images;

pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::String(prompt) = option {
        let response = generate_images(prompt).await.map(|urls| urls.join("\n"));
        match response {
            Ok(response) => response,
            Err(e) => format!("OpenAI: {}", e),
        }
    } else {
        "Please provide a valid prompt".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("imagine")
        .description("Ask ai to draw")
        .create_option(|option| {
            option
                .name("prompt")
                .description("The instruction")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
