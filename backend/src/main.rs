use anyhow::Result;
use headless_chrome::Browser;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

fn main() -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");

    tab.navigate_to(&rankings_url)?;
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
    let mut rows = Vec::new();

    // Loop through each player-row
    for row in document.select(&row_selector) {
        let mut row_data = Vec::new();
        let tds: Vec<_> = row.select(&td_selector).collect();

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
                    let bio_url = a.value().attr("href").unwrap_or("").to_string();
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
                    row_data.extend(vec![player_id, bio_url, name, team]);
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

        rows.push(row_data);
    }

    // Print the extracted data for debugging purposes
    for row in rows {
        println!("{:?}", row);
    }

    Ok(())
}

async fn scrape_bio(
    row_data: &mut Vec<String>,
    bio_headers: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let bio_url = &row_data[2];

    // Get HTML for the bio page
    let response = client.get(bio_url).send().await?;
    let body = response.text().await?;
    let html = Html::parse_document(&body);

    // Set up selectors
    let picture_selector = Selector::parse("picture img").unwrap();
    let clearfix_selector = Selector::parse("div.clearfix").unwrap();
    let bio_detail_selector = Selector::parse("span.bio-detail").unwrap();

    // Check for the picture element
    if let Some(picture) = html.select(&picture_selector).next() {
        if let Some(img_url) = picture.value().attr("src") {
            row_data.push(img_url.to_string());

            // Get bio details in the clearfix div
            if let Some(bio_div) = html.select(&clearfix_selector).next() {
                let mut bio_details_dict = HashMap::new();
                for detail in bio_div.select(&bio_detail_selector) {
                    let text = detail.text().collect::<Vec<_>>().concat();
                    let mut parts = text.split(": ");
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        bio_details_dict.insert(key.to_string(), value.to_string());
                    }
                }

                // Add bio details to row_data
                for header in bio_headers {
                    if let Some(value) = bio_details_dict.get(*header) {
                        row_data.push(value.clone());
                    } else {
                        row_data.push("None".to_string());
                    }
                }
            } else {
                // If bio details are missing, add "None" for each header
                row_data.extend(vec!["None".to_string(); bio_headers.len()]);
            }
        } else {
            // If no picture, add "None" for all bio fields
            row_data.extend(vec!["None".to_string(); bio_headers.len() + 1]);
        }
    } else {
        // No picture element, so add "None" for all bio fields
        row_data.extend(vec!["None".to_string(); bio_headers.len() + 1]);
    }

    Ok(())
}
