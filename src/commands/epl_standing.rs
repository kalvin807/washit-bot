use comfy_table::{Cell, Table};
use poise::serenity_prelude::*;
use serde_json::Value;
use crate::libs::epl_data_client::StandingsResponse;

use crate::{Context, Error};

/// Get EPL standings
#[poise::command(slash_command)]
pub async fn epl_standing(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let standings = crate::libs::epl_data_client::get_standings().await;
    match standings {
        Ok(StandingsResponse { standings, updated_at }) => {
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
                let team = standing["team"].as_object().unwrap();
                let stats = standing["stats"].as_array().unwrap();
                let mut row = Vec::new();

                row.push(Cell::new(standing["rank"].as_i64().unwrap()));
                row.push(Cell::new(team["shortName"].as_str().unwrap()));

                for stat in stats {
                    let value = stat["value"].as_i64().unwrap();
                    row.push(Cell::new(value));
                }

                table.add_row(row);
            }

            ctx.send(|m| {
                m.embed(|e| {
                    e.title("Premier League Standings")
                        .description(format!("```\n{}\n```", table))
                        .footer(|f| f.text(format!("最後更新: {}", updated_at)))
                })
            })
            .await?;
        }
        Err(e) => {
            ctx.send(|m| {
                m.content(format!("Error: {}", e))
                    .ephemeral(true)
            })
            .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_standings() {
        let json_str = r#"[
            {
                "rank": 1,
                "team": {
                    "shortName": "ARS"
                },
                "stats": [
                    {"value": 20},
                    {"value": 15},
                    {"value": 3},
                    {"value": 2},
                    {"value": 43},
                    {"value": 20},
                    {"value": 23},
                    {"value": 48}
                ]
            }
        ]"#;

        let standings: Value = serde_json::from_str(json_str).unwrap();
        let standings = standings.as_array().unwrap();

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
            let team = standing["team"].as_object().unwrap();
            let stats = standing["stats"].as_array().unwrap();
            let mut row = Vec::new();

            row.push(Cell::new(standing["rank"].as_i64().unwrap()));
            row.push(Cell::new(team["shortName"].as_str().unwrap()));

            for stat in stats {
                let value = stat["value"].as_i64().unwrap();
                row.push(Cell::new(value));
            }

            table.add_row(row);
        }

        let table_str = format!("{}", table);
        assert!(table_str.contains("ARS"));
        assert!(table_str.contains("48"));
    }
}
