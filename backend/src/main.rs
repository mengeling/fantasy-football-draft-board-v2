mod constants;
mod database;
mod models;
mod scrapers;

use anyhow::Result;
use headless_chrome::Browser;

use crate::database::operations::{bulk_save_players, bulk_save_rankings, bulk_save_stats};
use crate::database::pool::init_db;
use crate::scrapers::{
    player_scraper::PlayerScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_db().await?;
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;

    let players = PlayerScraper::process_tasks(player_tasks).await?;

    let stats_scraper = StatsScraper::new();
    let stats = stats_scraper.scrape().await?;

    bulk_save_players(&players).await?;
    bulk_save_rankings(&rankings).await?;
    bulk_save_stats(&stats).await?;

    Ok(())
}
