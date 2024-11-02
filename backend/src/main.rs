mod constants;
mod database;
mod models;
mod scrapers;

use anyhow::Result;
use dotenv::dotenv;
use headless_chrome::Browser;
use std::env;

use crate::database::operations::{bulk_save_players, bulk_save_rankings, bulk_save_stats};
use crate::database::pool::init_db;
use crate::scrapers::{rankings_scraper::RankingsScraper, stats_scraper::StatsScraper};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    init_db().await?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    // let scoring_var = env::var("SCORING").unwrap_or_else(|_| "half".to_string());
    // let scoring = match scoring_var.as_str() {
    //     "half" | "standard" | "ppr" => scoring_var,
    //     _ => anyhow::bail!("Invalid SCORING value. Must be 'half', 'standard', or 'ppr'."),
    // };

    let rankings_scraper = RankingsScraper::new(&tab);
    let stats_scraper = StatsScraper::new();

    let (players, rankings) = rankings_scraper.scrape().await?;
    let stats = stats_scraper.scrape().await?;

    bulk_save_players(&players).await?;
    bulk_save_rankings(&rankings).await?;
    bulk_save_stats(&stats).await?;

    Ok(())
}
