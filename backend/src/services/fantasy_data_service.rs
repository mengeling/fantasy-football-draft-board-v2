use anyhow::Result;
use thirtyfour::ChromiumLikeCapabilities;
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::database::connection::get_db_connection;
use crate::database::operations::fantasy_data_operations::{
    bulk_save_players, bulk_save_rankings, bulk_save_stats, delete_old_data,
    record_fantasy_data_update,
};
use crate::scrapers::{
    players_scraper::PlayersScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

pub async fn update() -> Result<()> {
    // Use Selenium Chrome service if available, otherwise local chromedriver
    let chromedriver_url =
        std::env::var("CHROMEDRIVER_URL").unwrap_or_else(|_| "http://localhost:9515".to_string());

    let mut caps = DesiredCapabilities::chrome();
    // Selenium images are pre-configured, but add essential flags for stability
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--disable-gpu")?;

    let driver = WebDriver::new(&chromedriver_url, caps).await?;
    driver
        .set_page_load_timeout(std::time::Duration::from_secs(120))
        .await?;
    driver
        .set_implicit_wait_timeout(std::time::Duration::from_secs(10))
        .await?;

    let rankings_scraper = RankingsScraper::new(&driver);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;

    driver.quit().await?;

    let players = PlayersScraper::process_tasks(player_tasks).await?;

    let stats_scraper = StatsScraper::new();
    let stats = stats_scraper.scrape().await?;

    let conn = get_db_connection().await?;
    let mut tx = conn.begin().await?;
    delete_old_data(&mut tx).await?;
    bulk_save_players(&players, &mut tx).await?;
    bulk_save_rankings(&rankings, &mut tx).await?;
    bulk_save_stats(&stats, &mut tx).await?;
    record_fantasy_data_update(&mut tx).await?;
    tx.commit().await?;

    Ok(())
}
