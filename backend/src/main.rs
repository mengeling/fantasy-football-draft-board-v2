mod constants;
mod database;
mod models;
mod scrapers;
mod utils;

use anyhow::Result;
use dotenv::dotenv;
use headless_chrome::Browser;

use crate::database::operations::save_player;
use crate::database::pool::init_db;
use crate::models::player::Player;
use crate::scrapers::{rankings_scraper::RankingsScraper, stats_scraper::StatsScraper, Scraper};
use crate::utils::helpers::combine_player_data;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    init_db().await?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");
    let stats_url = "https://www.fantasypros.com/nfl/stats/{}?scoring=HALF.php";

    let rankings_scraper = RankingsScraper::new(&tab, rankings_url);
    let stats_scraper = StatsScraper::new(stats_url.to_string());

    let rankings = rankings_scraper.scrape().await?;
    let stats = stats_scraper.scrape().await?;

    let combined_players: Vec<Player> = combine_player_data(rankings, stats);
    println!("Combined players: {:?}", combined_players);

    for player in combined_players {
        save_player(&player).await?;
    }

    Ok(())
}
