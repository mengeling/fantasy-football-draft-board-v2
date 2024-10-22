use async_trait::async_trait;
use anyhow::Result;
use crate::models::player::Player;

pub mod rankings_scraper;
pub mod stats_scraper;

#[async_trait]
pub trait Scraper {
    async fn scrape(&self) -> Result<Vec<Player>>;
}
