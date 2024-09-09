use anyhow::Result;
use futures::future::join_all;
use headless_chrome::{Browser, Tab};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

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
                "id",
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
                "id",
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
                "id",
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
                "id",
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
                "id",
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

const BIO_HEADERS: &[&str] = &["Height", "Weight", "Age", "College"];

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");
    let stats_url = "https://www.fantasypros.com/nfl/stats/{}?scoring=HALF.php";

    let rankings = scrape_rankings(&tab, &rankings_url).await?;
    println!("{:?}", rankings);
    let stats = scrape_stats(&stats_url).await?;
    println!("{:?}", stats);

    Ok(())
}

async fn scrape_rankings(tab: &Tab, rankings_url: &str) -> Result<Vec<Vec<String>>, anyhow::Error> {
    tab.navigate_to(rankings_url)?;
    tab.wait_until_navigated()?;
    tab.wait_for_element("table#ranking-table")?;

    // Scroll to the last player
    tab.evaluate(
        "let rows = document.querySelectorAll('tbody tr.player-row');
         let lastRow = rows[rows.length - 1];
         lastRow.scrollIntoView();",
        false,
    )?;

    // Now that the additional rows should be loaded, get the updated HTML
    let table_element = tab.wait_for_element("table#ranking-table")?;
    let table_html_debug =
        table_element.call_js_fn("function() { return this.outerHTML; }", vec![], false)?;

    // Fix: Assign the unwrap to a variable to extend its lifetime
    let table_html_value = table_html_debug.value.unwrap();
    let table_html = table_html_value.as_str().unwrap();

    // Parse the document
    let document = Html::parse_document(table_html);
    let row_selector = Selector::parse("tbody tr.player-row").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    // Regex for splitting position and ranking
    let re = Regex::new(r"(\D+)(\d+)").unwrap();

    // Data structure to hold the extracted player data
    let mut bio_futures = Vec::new();
    let mut rows = Vec::new();

    // Loop through each player-row
    for row in document.select(&row_selector) {
        let mut row_data = Vec::new();
        let tds: Vec<_> = row.select(&td_selector).collect();

        let mut bio_url = String::new();

        for (i, td) in tds.iter().enumerate() {
            match i {
                0 => {
                    // Get overall ranking
                    row_data.push(td.text().collect::<Vec<_>>().concat());
                }
                2 => {
                    // Get player_id, bio_url, name, and team
                    let div = td.select(&Selector::parse("div").unwrap()).next().unwrap();
                    let player_id = div.value().attr("data-player").unwrap_or("").to_string();
                    let a = td.select(&Selector::parse("a").unwrap()).next().unwrap();
                    bio_url = a.value().attr("href").unwrap_or("").to_string();
                    let name = a.text().collect::<Vec<_>>().concat();
                    let team = td
                        .select(&Selector::parse("span").unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim_matches(&['(', ')'][..])
                        .to_string();
                    row_data.extend(vec![player_id, bio_url.clone(), name, team]);
                }
                3 => {
                    // Split position and position ranking using regex
                    let text = td.text().collect::<Vec<_>>().concat();
                    if let Some(caps) = re.captures(&text) {
                        let position = caps.get(1).map_or("", |m| m.as_str()).to_string();
                        let ranking = caps.get(2).map_or("", |m| m.as_str()).to_string();
                        row_data.push(position);
                        row_data.push(ranking);
                    } else {
                        // Handle cases where the regex doesn't match
                        row_data.push(text);
                    }
                }
                4 => {
                    // Get bye week
                    row_data.push(td.text().collect::<Vec<_>>().concat());
                }
                _ => {}
            }
        }

        bio_futures.push(scrape_bio(bio_url.clone()));
        // row_data.extend(scrape_bio(&bio_url).await);
        // println!("{:?}", row_data);
        rows.push(row_data);
    }

    let bios = join_all(bio_futures).await;
    for (row, bio) in rows.iter_mut().zip(bios) {
        row.extend(bio);
    }

    Ok(rows)
}

async fn scrape_bio(bio_url: String) -> Vec<String> {
    // let client: Client = Client::new();
    let mut bio_data = vec![String::new(); BIO_HEADERS.len() + 1];

    let response = match CLIENT.get(bio_url).send().await {
        Ok(resp) => resp,
        Err(e) => return vec![format!("Error fetching bio: {}", e); BIO_HEADERS.len() + 1],
    };
    let body = match response.text().await {
        Ok(text) => text,
        Err(e) => return vec![format!("Error reading response: {}", e); BIO_HEADERS.len() + 1],
    };

    let html = Html::parse_document(&body);
    let picture_selector = Selector::parse("picture img").unwrap();
    let clearfix_selector = Selector::parse("div.clearfix").unwrap();
    let bio_detail_selector = Selector::parse("span.bio-detail").unwrap();

    if let Some(picture) = html.select(&picture_selector).next() {
        if let Some(img_url) = picture.value().attr("src") {
            bio_data[0] = img_url.to_string();

            if let Some(bio_div) = html.select(&clearfix_selector).next() {
                let bio_details: HashMap<_, _> = bio_div
                    .select(&bio_detail_selector)
                    .filter_map(|detail| {
                        let text = detail.text().collect::<String>();
                        let mut parts = text.split(": ");
                        Some((parts.next()?.to_string(), parts.next()?.to_string()))
                    })
                    .collect();

                for (i, header) in BIO_HEADERS.iter().enumerate() {
                    bio_data[i + 1] = bio_details
                        .get(header as &str)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                }
            }
        }
    }
    // println!("{:?}", bio_data);
    bio_data
}

async fn scrape_stats(url: &str) -> Result<Vec<Vec<String>>> {
    let mut dict_stats: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for (position, _position_stats) in STATS_HEADERS.iter() {
        let response = CLIENT.get(&url.replace("{}", position)).send().await?;
        let html = Html::parse_document(&response.text().await?);

        let table_selector = Selector::parse("table#data tbody").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        let mut rows = Vec::new();

        if let Some(table) = html.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let class_name = row.value().attr("class").unwrap_or("");
                let player_id = Regex::new(r"(\d+)")
                    .unwrap()
                    .captures(class_name)
                    .and_then(|cap| cap.get(1))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                let mut row_data = vec![player_id];
                for (i, td) in row.select(&cell_selector).enumerate() {
                    if i >= 2 && i < row.select(&cell_selector).count() - 1 {
                        row_data.push(td.text().collect::<String>());
                    }
                }
                rows.push(row_data);
            }
        }

        dict_stats.insert(position.to_string(), rows);
    }
    // println!("{:?}", dict_stats);
    create_stats_all(&dict_stats)
}

fn create_stats_all(dict_stats: &HashMap<String, Vec<Vec<String>>>) -> Result<Vec<Vec<String>>> {
    let mut all_stats: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut all_headers: Vec<String> = vec!["id".to_string()];

    // Collect all unique headers and initialize player stats
    for (position, stats) in dict_stats.iter() {
        let headers = STATS_HEADERS.get(position.as_str()).unwrap();
        for (i, header) in headers.iter().enumerate() {
            if i > 0 && !all_headers.contains(&header.to_string()) {
                all_headers.push(header.to_string());
            }
        }

        for row in stats {
            let player_id = &row[0];
            let player_stats = all_stats
                .entry(player_id.to_string())
                .or_insert_with(HashMap::new);
            for (i, value) in row.iter().enumerate() {
                if i < headers.len() {
                    player_stats.insert(headers[i].to_string(), value.to_string());
                }
            }
        }
    }

    // Create the final vector of vectors
    let mut result = Vec::new();
    result.push(all_headers.clone());

    for (player_id, stats) in all_stats {
        let mut row = vec![player_id];
        for header in all_headers.iter().skip(1) {
            row.push(
                stats
                    .get(header)
                    .cloned()
                    .unwrap_or_else(|| "0".to_string()),
            );
        }
        result.push(row);
    }

    Ok(result)
}
