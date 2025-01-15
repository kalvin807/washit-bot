use comfy_table::{Cell, Table};
use poise::serenity_prelude::*;
use crate::libs::epl_data_client::{StandingsResponse, TeamStanding};

use crate::{Context, Error};

/// Get EPL standings
#[poise::command(slash_command)]
pub async fn epl_standing(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let standings = crate::libs::epl_data_client::get_standings().await;
    match standings {
        Ok(StandingsResponse { standing: standings, updated_at }) => {
            let mut table = Table::new();
            table.set_header(vec![
                Cell::new("#"),
                Cell::new("Team"),
                Cell::new("P"),
                Cell::new("W"),
                Cell::new("D"),
                Cell::new("L"),
                Cell::new("GF"),
                Cell::new("GA"),
                Cell::new("GD"),
                Cell::new("Pts"),
            ]);

            for standing in standings {
                let mut row = Vec::new();

                row.push(Cell::new(standing.standing));
                row.push(Cell::new(&standing.team_name));
                row.push(Cell::new(standing.match_count));
                row.push(Cell::new(standing.won_count));
                row.push(Cell::new(standing.drawn_count));
                row.push(Cell::new(standing.lost_count));
                row.push(Cell::new(standing.goal_point));
                row.push(Cell::new(standing.goal_point));
                row.push(Cell::new(standing.point_difference));
                row.push(Cell::new(standing.victory_point));
                table.add_row(row);
            }

            let embed = CreateEmbed::default()
                .title("Premier League Standings")
                .description(format!("```\n{}\n```", table))
                .footer(CreateEmbedFooter::new(format!("最後更新: {}", updated_at)));

            let reply = poise::CreateReply::default()
                .embed(embed);
            ctx.send(reply).await?;
        }
        Err(e) => {
            let reply = poise::CreateReply::default()
                .content(format!("Error: {}", e))
                .ephemeral(true);
            ctx.send(reply).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_standings() {
        let standings = vec![
            TeamStanding {
                standing: 1,
                team_name: "ARS".to_string(),
                victory_point: 48,
                match_count: 20,
                won_count: 15,
                drawn_count: 3,
                lost_count: 2,
                goal_point: 43,
                point_difference: 23,
                lost_point: 20,
            }
        ];

        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("#"),
            Cell::new("Team"),
            Cell::new("P"),
            Cell::new("W"),
            Cell::new("D"),
            Cell::new("L"),
            Cell::new("GF"),
            Cell::new("GA"),
            Cell::new("GD"),
            Cell::new("Pts"),
        ]);

        for standing in standings {
            let mut row = Vec::new();

            row.push(Cell::new(standing.standing));
            row.push(Cell::new(&standing.team_name));
            row.push(Cell::new(standing.match_count));
            row.push(Cell::new(standing.won_count));
            row.push(Cell::new(standing.drawn_count));
            row.push(Cell::new(standing.lost_count));
            row.push(Cell::new(standing.goal_point));
            row.push(Cell::new(standing.goal_point));
            row.push(Cell::new(standing.point_difference));
            row.push(Cell::new(standing.victory_point));

            table.add_row(row);
        }

        let table_str = format!("{}", table);
        assert!(table_str.contains("ARS"));
        assert!(table_str.contains("48"));
    }
}
