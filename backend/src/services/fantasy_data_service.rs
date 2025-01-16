use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};

use crate::database::connection::get_db_connection;
use crate::database::operations::fantasy_data_operations::{
    bulk_save_players, bulk_save_rankings, bulk_save_stats, delete_old_data,
    record_fantasy_data_update,
};
use crate::scrapers::{
    player_scraper::PlayerScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

pub async fn update() -> Result<()> {
    let launch_options = LaunchOptions {
        port: Some(9500),
        headless: true,
        path: None,
        args: vec![
            "--no-sandbox",
            "--disable-setuid-sandbox",
            "--disable-dev-shm-usage",
            "--disable-gpu",
        ],
        idle_browser_timeout: std::time::Duration::from_secs(30),
        ..Default::default()
    };
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;

    let players = PlayerScraper::process_tasks(player_tasks).await?;

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
