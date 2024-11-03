mod constants;
mod database;
mod models;
mod scrapers;

use anyhow::Result;
use chrono::Local;
use env_logger;
use headless_chrome::Browser;
use log::{error, info};

use crate::database::operations::{bulk_save_players, bulk_save_rankings, bulk_save_stats};
use crate::database::pool::init_db;
use crate::scrapers::{rankings_scraper::RankingsScraper, stats_scraper::StatsScraper};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting scraper job at {}", Local::now());

    match run_scraper().await {
        Ok(_) => {
            info!("Scraper job completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Scraper job failed: {}", e);
            Err(e)
        }
    }
}

async fn run_scraper() -> Result<()> {
    init_db().await?;
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (players, rankings) = rankings_scraper.scrape().await?;

    let stats_scraper = StatsScraper::new();
    let stats = stats_scraper.scrape().await?;

    bulk_save_players(&players).await?;
    bulk_save_rankings(&rankings).await?;
    bulk_save_stats(&stats).await?;

    Ok(())
}
