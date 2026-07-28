use anyhow::Result;
use headless_chrome::Browser;

use crate::database::connection::get_db_connection;
use crate::database::operations::fantasy_data_operations::{
    bulk_save_players, bulk_save_rankings, bulk_save_stats, delete_old_data,
    record_fantasy_data_update,
};
use crate::scrapers::{
    players_scraper::PlayersScraper, rankings_scraper::RankingsScraper, stats_scraper::StatsScraper,
};

pub async fn update() -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let rankings_scraper = RankingsScraper::new(&tab);
    let (rankings, player_tasks) = rankings_scraper.scrape().await?;
    eprintln!(
        "Scraped {} rankings across {} unique players",
        rankings.len(),
        player_tasks.len()
    );

    let stats_scraper = StatsScraper::new(&tab);
    let stats = stats_scraper.scrape().await?;
    eprintln!("Scraped {} player stat lines", stats.len());

    let players = PlayersScraper::process_tasks(player_tasks).await?;
    eprintln!("Scraped {} player profiles", players.len());

    if players.is_empty() || rankings.is_empty() || stats.is_empty() {
        return Err(anyhow::anyhow!(
            "Scrape produced incomplete data (players: {}, rankings: {}, stats: {}); \
             aborting refresh to preserve existing data",
            players.len(),
            rankings.len(),
            stats.len()
        ));
    }

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
