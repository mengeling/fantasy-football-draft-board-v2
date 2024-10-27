mod constants;
mod database;
mod models;
mod scrapers;
mod utils;

use anyhow::Result;
use dotenv::dotenv;
use headless_chrome::Browser;
use std::env;

use crate::database::operations::save_player;
use crate::database::pool::init_db;
use crate::models::player::Player;
use crate::scrapers::{rankings_scraper::RankingsScraper, stats_scraper::StatsScraper};
use crate::utils::helpers::combine_player_data;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    init_db().await?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let scoring_var = env::var("SCORING").unwrap_or_else(|_| "half".to_string());
    let scoring = match scoring_var.as_str() {
        "half" | "standard" | "ppr" => scoring_var,
        _ => anyhow::bail!("Invalid SCORING value. Must be 'half', 'standard', or 'ppr'."),
    };

    let rankings_scraper = RankingsScraper::new(&tab, &scoring);
    let stats_scraper = StatsScraper::new(&scoring);

    let (players, rankings) = rankings_scraper.scrape().await?;
    let stats = stats_scraper.scrape().await?;

    for player in players {
        save_player(&player).await?;
    }

    // let combined_players: Vec<Player> = combine_player_data(players, stats);
    // println!("Combined players: {:?}", combined_players);

    // for player in combined_players {
    //     save_player(&player).await?;
    // }

    Ok(())
}
