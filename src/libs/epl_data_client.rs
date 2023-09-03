use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamStanding {
    pub standing: usize,
    pub team_name: String,
    pub victory_point: u32,
    pub match_count: u32,
    pub won_count: u32,
    pub drawn_count: u32,
    pub lost_count: u32,
    pub goal_point: i32,
    pub lost_point: i32,
    pub point_difference: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandingsResponse {
    pub standing: Vec<TeamStanding>,
    pub updated_at: String,
}

pub async fn get_standings() -> Result<StandingsResponse, Error> {
    let url = "https://epl-discord-bot.kalvin.workers.dev/standing";
    let response = reqwest::get(url).await?.json::<StandingsResponse>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_standings() {
        let standings = get_standings().await.unwrap();
        print!("{:?}", standings)
    }
}
