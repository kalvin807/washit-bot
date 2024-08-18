use crate::libs::epl_data_client::{get_standings, TeamStanding};
use comfy_table::Table;
use comfy_table::{presets::ASCII_HORIZONTAL_ONLY, ContentArrangement};
use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub fn format_standings(standings: &[TeamStanding]) -> String {
    let mut table = Table::new();
    table
        .load_preset(ASCII_HORIZONTAL_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(52)
        .set_header(vec![
            "#", "隊", "分", "場", "勝", "平", "負", "進", "失", "淨",
        ]);

    for standing in standings.iter() {
        table.add_row(vec![
            standing.standing.to_string(),
            standing.team_name.clone(),
            standing.victory_point.to_string(),
            standing.match_count.to_string(),
            standing.won_count.to_string(),
            standing.drawn_count.to_string(),
            standing.lost_count.to_string(),
            standing.goal_point.to_string(),
            standing.lost_point.to_string(),
            standing.point_difference.to_string(),
        ]);
    }

    table.to_string()
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    command.defer(&ctx).await.unwrap();
    let standings_response = get_standings().await.unwrap();

    let formatted_standings = format_standings(&standings_response.standing);
    let embed = create_standings_embed(&formatted_standings, &standings_response.updated_at);

    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
    {
        println!("Cannot edit response: {}", why);
    }
}

fn create_standings_embed(standings: &str, updated_at: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("積分榜")
        .description(format!("```\n{}\n```", standings))
        .footer(|f| f.text(format!("最後更新: {}", updated_at)))
        .color(0x3498db);
    embed
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("epl_standing").description("英超積分榜")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_standings() {
        let standings = vec![
            TeamStanding {
                standing: 1,
                team_name: "曼城".to_string(),
                victory_point: 89,
                match_count: 38,
                won_count: 29,
                drawn_count: 2,
                lost_count: 7,
                goal_point: 94,
                lost_point: 33,
                point_difference: 61,
            },
            TeamStanding {
                standing: 2,
                team_name: "阿森納".to_string(),
                victory_point: 84,
                match_count: 38,
                won_count: 26,
                drawn_count: 6,
                lost_count: 6,
                goal_point: 88,
                lost_point: 43,
                point_difference: 45,
            },
        ];

        let formatted = format_standings(&standings);

        assert!(formatted.contains("曼城"));
        assert!(formatted.contains("阿森納"));
    }
}
