use anyhow::Result;
use headless_chrome::Browser;

use crate::database::connection::DB_POOL;
use crate::database::operations::{
    bulk_save_players, bulk_save_rankings, bulk_save_stats, delete_old_scraped_data,
    record_scraper_run,
};
use crate::scrapers::{
    player_scraper::PlayerScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

pub async fn run_scrapers() -> Result<()> {
    // Scrape all data
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;

    let players = PlayerScraper::process_tasks(player_tasks).await?;

    let stats_scraper = StatsScraper::new();
    let stats = stats_scraper.scrape().await?;

    let mut tx = DB_POOL.begin().await?;
    delete_old_scraped_data(&mut tx).await?;
    bulk_save_players(&players, &mut tx).await?;
    bulk_save_rankings(&rankings, &mut tx).await?;
    bulk_save_stats(&stats, &mut tx).await?;
    record_scraper_run(&mut tx).await?;
    tx.commit().await?;

    Ok(())
}
