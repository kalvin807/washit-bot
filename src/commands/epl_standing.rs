use crate::libs::epl_data_client::get_standings;
use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    command.defer(&ctx).await.unwrap();
    let standings = get_standings().await.unwrap();
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("EPL Standing");
                embed.fields(
                    standings
                        .standing
                        .iter()
                        .map(|standing| {
                            (
                                standing.team_name.clone(),
                                format!(
                                    "排名: {} \
                                    積分: {} \
                                    上陣: {} \
                                    勝: {} \
                                    和: {} \
                                    負: {} \
                                    進球: {} \
                                    失球: {} \
                                    差: {}",
                                    standing.standing,
                                    standing.victory_point,
                                    standing.match_count,
                                    standing.won_count,
                                    standing.drawn_count,
                                    standing.lost_count,
                                    standing.goal_point,
                                    standing.lost_point,
                                    standing.point_difference,
                                ),
                                false,
                            )
                        })
                        .collect::<Vec<(String, String, bool)>>(),
                );
                embed
            })
        })
        .await
    {
        println!("Cannot edit response: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("epl_standing")
        .description("Fetch EPL standing")
}
