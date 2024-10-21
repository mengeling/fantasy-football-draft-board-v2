use anyhow::Result;
use async_trait::async_trait;
use dotenv::dotenv;
use futures::future::join_all;
use headless_chrome::{Browser, Tab};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .expect("Failed to create HTTP client")
});

lazy_static! {
    static ref STATS_HEADERS: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        (
            "qb",
            vec![
                "pass_cmp",
                "pass_att",
                "pass_cmp_pct",
                "pass_yds",
                "pass_yds_per_att",
                "pass_td",
                "pass_int",
                "pass_sacks",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
        (
            "rb",
            vec![
                "rush_att",
                "rush_yds",
                "rush_yds_per_att",
                "rush_long",
                "rush_20",
                "rush_td",
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_td",
                "fumbles",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
        (
            "wr",
            vec![
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_long",
                "rec_20",
                "rec_td",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
        (
            "te",
            vec![
                "receptions",
                "rec_tgt",
                "rec_yds",
                "rec_yds_per_rec",
                "rec_long",
                "rec_20",
                "rec_td",
                "rush_att",
                "rush_yds",
                "rush_td",
                "fumbles",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
        (
            "k",
            vec![
                "field_goals",
                "fg_att",
                "fg_pct",
                "fg_long",
                "fg_1_19",
                "fg_20_29",
                "fg_30_39",
                "fg_40_49",
                "fg_50",
                "extra_points",
                "xp_att",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
        (
            "dst",
            vec![
                "sacks",
                "int",
                "fumbles_recovered",
                "fumbles_forced",
                "def_td",
                "safeties",
                "special_teams_td",
                "games",
                "fantasy_pts",
                "fantasy_pts_per_game",
            ]
        ),
    ]);
}

lazy_static! {
    static ref STATS_ALL_HEADERS: Vec<&'static str> = vec![
        "id",
        "pass_cmp",
        "pass_att",
        "pass_cmp_pct",
        "pass_yds",
        "pass_yds_per_att",
        "pass_td",
        "pass_int",
        "pass_sacks",
        "rush_att",
        "rush_yds",
        "rush_yds_per_att",
        "rush_long",
        "rush_20",
        "rush_td",
        "fumbles",
        "receptions",
        "rec_tgt",
        "rec_yds",
        "rec_yds_per_rec",
        "rec_long",
        "rec_20",
        "rec_td",
        "field_goals",
        "fg_att",
        "fg_pct",
        "fg_long",
        "fg_1_19",
        "fg_20_29",
        "fg_30_39",
        "fg_40_49",
        "fg_50",
        "extra_points",
        "xp_att",
        "sacks",
        "int",
        "fumbles_recovered",
        "fumbles_forced",
        "def_td",
        "safeties",
        "special_teams_td",
        "games",
        "fantasy_pts",
        "fantasy_pts_per_game",
    ];
}

// const BIO_HEADERS: &[&str] = &["Height", "Weight", "Age", "College"];

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: i32,
    name: String,
    team: String,
    position: String,
    ranking: PlayerRanking,
    bye_week: i32,
    bio: PlayerBio,
    stats: HashMap<String, f64>,
    bio_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerRanking {
    overall: i32,
    position: i32,
}

impl Default for PlayerRanking {
    fn default() -> Self {
        Self {
            overall: 0,
            position: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerBio {
    image_url: String,
    height: String,
    weight: String,
    age: String,
    college: String,
}

impl Default for PlayerBio {
    fn default() -> Self {
        Self {
            image_url: String::new(),
            height: String::new(),
            weight: String::new(),
            age: String::new(),
            college: String::new(),
        }
    }
}

struct PlayerData {
    id: i32,
    bio_url: String,
    name: String,
    team: String,
}
#[async_trait]
trait Scraper {
    async fn scrape(&self) -> Result<Vec<Player>>;
}

struct RankingsScraper<'a> {
    tab: &'a Tab,
    url: String,
}

struct StatsScraper {
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create tables if they don't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            team TEXT NOT NULL,
            position TEXT NOT NULL,
            overall_ranking INTEGER NOT NULL,
            position_ranking INTEGER NOT NULL,
            bye_week INTEGER NOT NULL,
            bio_url TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS player_bios (
            player_id INTEGER PRIMARY KEY REFERENCES players(id),
            image_url TEXT NOT NULL,
            height TEXT NOT NULL,
            weight TEXT NOT NULL,
            age INTEGER NOT NULL,
            college TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS player_stats (
            player_id INTEGER REFERENCES players(id),
            stat_name TEXT NOT NULL,
            stat_value DOUBLE PRECISION NOT NULL,
            PRIMARY KEY (player_id, stat_name)
        )",
    )
    .execute(&pool)
    .await?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");
    let stats_url = "https://www.fantasypros.com/nfl/stats/{}?scoring=HALF.php";

    let rankings_scraper = RankingsScraper {
        tab: &tab,
        url: rankings_url,
    };
    let stats_scraper = StatsScraper {
        url: stats_url.to_string(),
    };

    let rankings = rankings_scraper.scrape().await?;
    let stats = stats_scraper.scrape().await?;

    let combined_players: Vec<Player> = combine_player_data(rankings, stats);
    println!("Combined players: {:?}", combined_players);

    // Save players to the database
    for player in combined_players {
        save_player(&pool, &player).await?;
    }

    Ok(())
}

async fn save_player(pool: &sqlx::PgPool, player: &Player) -> Result<()> {
    // Insert player
    sqlx::query(
        "INSERT INTO players (id, name, team, position, overall_ranking, position_ranking, bye_week, bio_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (id) DO UPDATE SET
         name = EXCLUDED.name,
         team = EXCLUDED.team,
         position = EXCLUDED.position,
         overall_ranking = EXCLUDED.overall_ranking,
         position_ranking = EXCLUDED.position_ranking,
         bye_week = EXCLUDED.bye_week,
         bio_url = EXCLUDED.bio_url"
    )
    .bind(player.id)
    .bind(&player.name)
    .bind(&player.team)
    .bind(&player.position)
    .bind(player.ranking.overall)
    .bind(player.ranking.position)
    .bind(player.bye_week)
    .bind(&player.bio_url)
    .execute(pool)
    .await?;

    // Insert player bio
    sqlx::query(
        "INSERT INTO player_bios (player_id, image_url, height, weight, age, college)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (player_id) DO UPDATE SET
         image_url = EXCLUDED.image_url,
         height = EXCLUDED.height,
         weight = EXCLUDED.weight,
         age = EXCLUDED.age,
         college = EXCLUDED.college",
    )
    .bind(player.id)
    .bind(&player.bio.image_url)
    .bind(&player.bio.height)
    .bind(&player.bio.weight)
    .bind(player.bio.age.parse::<i32>().unwrap_or(0))
    .bind(&player.bio.college)
    .execute(pool)
    .await?;

    // Insert player stats
    for (stat_name, stat_value) in &player.stats {
        sqlx::query(
            "INSERT INTO player_stats (player_id, stat_name, stat_value)
             VALUES ($1, $2, $3)
             ON CONFLICT (player_id, stat_name) DO UPDATE SET
             stat_value = EXCLUDED.stat_value",
        )
        .bind(player.id)
        .bind(stat_name)
        .bind(stat_value)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[async_trait]
impl<'a> Scraper for RankingsScraper<'a> {
    async fn scrape(&self) -> Result<Vec<Player>> {
        self.tab.navigate_to(&self.url)?;
        self.tab.wait_until_navigated()?;
        self.tab.wait_for_element("table#ranking-table")?;

        // Scroll to the last player
        // self.tab.evaluate(
        //     "let rows = document.querySelectorAll('tbody tr.player-row');
        //         let lastRow = rows[rows.length - 1];
        //         lastRow.scrollIntoView();",
        //     false,
        // )?;

        let table_element = self.tab.wait_for_element("table#ranking-table")?;
        let table_html_debug =
            table_element.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;

        let table_html_value = table_html_debug.value.unwrap();
        let table_html = table_html_value.as_str().unwrap();

        let players = parse_rankings_html(&table_html)?;
        let bios = join_all(players.iter().map(|p| scrape_bio(p.bio_url.clone()))).await;
        Ok(players
            .into_iter()
            .zip(bios)
            .map(|(mut player, bio)| {
                player.bio = bio.unwrap_or_default();
                player
            })
            .collect())
    }
}

fn parse_rankings_html(table_html: &str) -> Result<Vec<Player>> {
    let document = Html::parse_document(table_html);
    let row_selector = Selector::parse("tbody tr.player-row").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let re = Regex::new(r"(\D+)(\d+)").unwrap();

    let mut players = Vec::new();

    for row in document.select(&row_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();

        let overall_ranking = tds[0]
            .text()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        let player_data = extract_player_data(&tds[2]);
        let (position, position_ranking) = extract_position_data(&tds[3], &re);
        let bye_week = tds[4]
            .text()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);

        players.push(Player {
            id: player_data.id,
            name: player_data.name,
            team: player_data.team,
            position,
            ranking: PlayerRanking {
                overall: overall_ranking,
                position: position_ranking.parse::<i32>().unwrap_or(0),
            },
            bye_week,
            bio: PlayerBio::default(),
            stats: HashMap::new(),
            bio_url: player_data.bio_url,
        });
    }

    Ok(players)
}

#[async_trait]
impl Scraper for StatsScraper {
    async fn scrape(&self) -> Result<Vec<Player>> {
        let mut players: Vec<Player> = Vec::new();

        for (position, headers) in STATS_HEADERS.iter() {
            let response = CLIENT.get(&self.url.replace("{}", position)).send().await?;
            let html = Html::parse_document(&response.text().await?);

            let table_selector = Selector::parse("table#data tbody").unwrap();
            let row_selector = Selector::parse("tr").unwrap();
            let cell_selector = Selector::parse("td").unwrap();

            if let Some(table) = html.select(&table_selector).next() {
                for row in table.select(&row_selector).take(1) {
                    let player_id = extract_player_id(&row);
                    let mut stats = HashMap::new();

                    for (i, td) in row.select(&cell_selector).enumerate().skip(2) {
                        if i < headers.len() + 2 {
                            let value = td.text().collect::<String>().parse::<f64>().unwrap_or(0.0);
                            stats.insert(headers[i - 2].to_string(), value);
                        }
                    }

                    if let Some(existing_player) = players.iter_mut().find(|p| p.id == player_id) {
                        // Update existing player's stats only if new value is greater
                        for (key, &value) in &stats {
                            existing_player
                                .stats
                                .entry(key.clone())
                                .and_modify(|e| *e = f64::max(*e, value))
                                .or_insert(value);
                        }
                    } else {
                        // Create a new player with all stats initialized to 0.0
                        let mut all_stats = HashMap::new();
                        for &header in STATS_ALL_HEADERS.iter() {
                            all_stats.insert(header.to_string(), 0.0);
                        }
                        // Update with the current position's stats
                        all_stats.extend(stats);

                        players.push(Player {
                            id: player_id,
                            name: String::new(),
                            team: String::new(),
                            position: position.to_string(),
                            ranking: PlayerRanking::default(),
                            bye_week: String::new().parse::<i32>().unwrap_or(0),
                            bio: PlayerBio::default(),
                            stats: all_stats,
                            bio_url: String::new(),
                        });
                    }
                }
            }
        }

        Ok(players)
    }
}

fn extract_player_id(row: &scraper::element_ref::ElementRef) -> i32 {
    let class_name = row.value().attr("class").unwrap_or("");
    Regex::new(r"(\d+)")
        .unwrap()
        .captures(class_name)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().parse::<i32>().unwrap_or(0))
        .unwrap_or(0)
}

fn extract_player_data(td: &scraper::element_ref::ElementRef) -> PlayerData {
    let div = td.select(&Selector::parse("div").unwrap()).next().unwrap();
    let a = td.select(&Selector::parse("a").unwrap()).next().unwrap();
    let span = td.select(&Selector::parse("span").unwrap()).next().unwrap();

    PlayerData {
        id: div
            .value()
            .attr("data-player")
            .unwrap_or("0")
            .parse::<i32>()
            .unwrap_or(0),
        bio_url: a.value().attr("href").unwrap_or("").to_string(),
        name: a.text().collect::<String>(),
        team: span
            .text()
            .collect::<String>()
            .trim_matches(&['(', ')'][..])
            .to_string(),
    }
}

fn extract_position_data(td: &scraper::element_ref::ElementRef, re: &Regex) -> (String, String) {
    let text = td.text().collect::<String>();
    if let Some(caps) = re.captures(&text) {
        (caps[1].to_string(), caps[2].to_string())
    } else {
        (text, String::new())
    }
}

async fn scrape_bio(bio_url: String) -> Result<PlayerBio> {
    let response = CLIENT.get(&bio_url).send().await?;
    let body = response.text().await?;
    let html = Html::parse_document(&body);

    let picture_selector = Selector::parse("picture img").unwrap();
    let clearfix_selector = Selector::parse("div.clearfix").unwrap();
    let bio_detail_selector = Selector::parse("span.bio-detail").unwrap();

    let mut bio = PlayerBio::default();

    if let Some(picture) = html.select(&picture_selector).next() {
        bio.image_url = picture.value().attr("src").unwrap_or("").to_string();
    }

    if let Some(bio_div) = html.select(&clearfix_selector).next() {
        let bio_details: HashMap<_, _> = bio_div
            .select(&bio_detail_selector)
            .filter_map(|detail| {
                let text = detail.text().collect::<String>();
                let mut parts = text.split(": ");
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();

        bio.height = bio_details.get("Height").cloned().unwrap_or_default();
        bio.weight = bio_details.get("Weight").cloned().unwrap_or_default();
        bio.age = bio_details.get("Age").cloned().unwrap_or_default();
        bio.college = bio_details.get("College").cloned().unwrap_or_default();
    }

    Ok(bio)
}

fn combine_player_data(rankings: Vec<Player>, stats: Vec<Player>) -> Vec<Player> {
    let mut combined = HashMap::new();

    for player in rankings {
        combined.insert(player.id, player);
    }

    for player in stats {
        if let Some(combined_player) = combined.get_mut(&player.id) {
            combined_player.stats = player.stats;
        }
    }

    combined.into_values().collect()
}
