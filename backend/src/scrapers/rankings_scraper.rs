use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::models::players::{PlayerIdentity, PlayerTask, Position, Team};
use crate::models::rankings::{Rankings, RankingsBase, ScoringSettings};

pub struct RankingsScraper<'a> {
    tab: &'a Tab,
}

impl<'a> RankingsScraper<'a> {
    pub fn new(tab: &'a Tab) -> Self {
        Self { tab }
    }

    fn get_urls() -> std::collections::HashMap<ScoringSettings, &'static str> {
        std::collections::HashMap::from([
            (
                ScoringSettings::Standard,
                "https://www.fantasypros.com/nfl/rankings/consensus-cheatsheets.php",
            ),
            (
                ScoringSettings::Half,
                "https://www.fantasypros.com/nfl/rankings/half-point-ppr-cheatsheets.php",
            ),
            (
                ScoringSettings::PPR,
                "https://www.fantasypros.com/nfl/rankings/ppr-cheatsheets.php",
            ),
        ])
    }

    pub async fn scrape(&self) -> Result<(Vec<Rankings>, Vec<PlayerTask>)> {
        let mut ranking_tables = Vec::new();

        for (scoring_settings, url) in Self::get_urls() {
            println!("Scraping {}...", url);
            self.tab.navigate_to(url)?;
            self.tab.wait_until_navigated()?;
            // self.tab.wait_for_element("table#ranking-table")?;

            // Click the "View" dropdown button to change it to the "Ranks" view to get detailed rankings
            let view_dropdown_button = self
                .tab
                .wait_for_element(".select-advanced--view .select-advanced__button")?;
            // println!("Found dropdown button, clicking...");

            // Debug: Check button state before clicking
            // match view_dropdown_button.get_content() {
            //     Ok(html) => println!("Button HTML before click: {}", html),
            //     Err(e) => println!("Error getting button HTML: {:?}", e),
            // }

            // Click the dropdown button using JavaScript
            // println!("Clicking dropdown button with JavaScript...");
            match view_dropdown_button.call_js_fn("function() { this.click(); }", vec![], false) {
                Ok(_) => println!("JavaScript click completed"),
                Err(e) => {
                    println!("JavaScript click failed: {:?}", e);
                    return Err(anyhow::anyhow!("Failed to click dropdown button: {:?}", e));
                }
            }

            // Wait longer for the dropdown to open
            // println!("Waiting for dropdown to open...");
            // std::thread::sleep(std::time::Duration::from_millis(500));

            // Debug: Check if the button got the --open class
            // match view_dropdown_button.get_content() {
            //     Ok(html) => {
            //         println!("Button HTML after click: {}", html);
            //         if html.contains("select-advanced__button--open") {
            //             println!("Button has --open class (dropdown should be open)");
            //         } else {
            //             println!("Button does NOT have --open class (dropdown is closed)");
            //         }
            //     }
            //     Err(e) => println!("Error getting button HTML after click: {:?}", e),
            // }

            // // Check if dropdown is open by looking for the --open class on the button
            // match view_dropdown_button.get_content() {
            //     Ok(html) => {
            //         if html.contains("select-advanced__button--open") {
            //             println!("Dropdown is open (button has --open class)");
            //         } else {
            //             println!("Dropdown is NOT open (button missing --open class)");
            //             return Err(anyhow::anyhow!(
            //                 "Failed to open dropdown - button missing --open class"
            //             ));
            //         }
            //     }
            //     Err(e) => return Err(anyhow::anyhow!("Error checking button state: {:?}", e)),
            // }

            // Find the dropdown options using the correct selectors
            let option_buttons = self.tab.find_elements(
                ".select-advanced--view .select-advanced__item .select-advanced-content--button",
            )?;
            println!("Found {} dropdown options", option_buttons.len());

            // Try to find and click the "Ranks" option
            let mut found_ranks_option = false;
            for (i, option_button) in option_buttons.iter().enumerate() {
                // Debug: Print the HTML of each option button
                // match option_button.get_content() {
                //     Ok(html) => println!("Option {} HTML: {}", i, html),
                //     Err(e) => println!("Error getting option {} HTML: {:?}", i, e),
                // }

                // Get the text from the .select-advanced-content__text div
                match option_button.find_element(".select-advanced-content__text") {
                    Ok(text_element) => {
                        // Debug: Print the HTML of the text element
                        // match text_element.get_content() {
                        //     Ok(html) => println!("Option {} text element HTML: {}", i, html),
                        //     Err(e) => println!(
                        //         "Error getting text element HTML for option {}: {:?}",
                        //         i, e
                        //     ),
                        // }

                        match text_element.get_inner_text() {
                            Ok(text) => {
                                // println!(
                                //     "Option {} raw text: '{}' (length: {})",
                                //     i,
                                //     text,
                                //     text.len()
                                // );
                                // let trimmed_text = text.trim();
                                // println!(
                                //     "Option {} trimmed text: '{}' (length: {})",
                                //     i,
                                //     trimmed_text,
                                //     trimmed_text.len()
                                // );

                                // // Also check for any non-printable characters
                                // let bytes: Vec<u8> = text.bytes().collect();
                                // println!("Option {} bytes: {:?}", i, bytes);

                                if text.trim() == "Ranks" {
                                    println!("Found Ranks option, clicking...");
                                    option_button.click()?;
                                    found_ranks_option = true;
                                    break;
                                }
                            }
                            Err(e) => {
                                // eprintln!("Error getting text for option {}: {:?}", i, e);
                            }
                        }
                    }
                    Err(e) => {
                        // eprintln!("Error finding text element for option {}: {:?}", i, e);
                    }
                }
            }

            if !found_ranks_option {
                return Err(anyhow::anyhow!("Could not find 'Ranks' option in dropdown"));
            }

            // Wait for table to render with detailed rankings and then scroll to the bottom of it to load all players
            self.tab.wait_for_element("table#ranking-table")?;
            let ranking_table_last_row = self
                .tab
                .wait_for_element("tbody tr.player-row:last-child")?;
            ranking_table_last_row.scroll_into_view()?;

            let ranking_table = self.tab.wait_for_element("table#ranking-table")?;
            ranking_tables.push((ranking_table.get_content()?, scoring_settings));
        }
        self.tab.close(true)?;

        let mut seen_players = std::collections::HashSet::new();
        let mut all_rankings = Vec::new();
        let mut all_player_tasks = Vec::new();

        for (ranking_table, scoring_settings) in ranking_tables {
            let (rankings, player_tasks) = self
                .parse_ranking_table(&ranking_table, &mut seen_players, scoring_settings)
                .await?;
            all_rankings.extend(rankings);
            all_player_tasks.extend(player_tasks);
        }

        Ok((all_rankings, all_player_tasks))
    }

    async fn parse_ranking_table(
        &self,
        table_html: &str,
        seen_players: &mut std::collections::HashSet<i32>,
        scoring_settings: ScoringSettings,
    ) -> Result<(Vec<Rankings>, Vec<PlayerTask>)> {
        let document = Html::parse_document(table_html);
        let row_selector = Selector::parse("tbody tr.player-row").unwrap();
        let cell_selector = Selector::parse("td").unwrap();
        let ranking_regex = Regex::new(r"(\D+)(\d+)").unwrap();

        let mut rankings = Vec::new();
        let mut player_tasks = Vec::new();

        for row in document.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();

            // Debug: Print cell contents
            println!("=== Row Debug ===");
            for (i, cell) in cells.iter().enumerate() {
                let cell_text = cell.text().collect::<String>().trim().to_string();
                println!("Cell {}: '{}'", i, cell_text);
            }
            println!("================");

            let overall_ranking = parse_cell_as_number::<i32>(&cells[0], "Overall ranking");
            let player_identity = get_player_identity(&cells[2]);
            let (position, position_ranking) = get_position_ranking(&cells[3], &ranking_regex);
            let best_ranking = parse_cell_as_number::<i32>(&cells[4], "Best ranking");
            let worst_ranking = parse_cell_as_number::<i32>(&cells[5], "Worst ranking");
            let average_ranking = parse_cell_as_number::<f32>(&cells[6], "Average ranking");
            let standard_deviation_ranking =
                parse_cell_as_number::<f32>(&cells[7], "Standard deviation ranking");

            rankings.push(Rankings {
                player_id: player_identity.id,
                scoring_settings: scoring_settings.clone(),
                base: RankingsBase {
                    overall: overall_ranking,
                    position: position_ranking,
                    best: best_ranking,
                    worst: worst_ranking,
                    average: average_ranking,
                    standard_deviation: standard_deviation_ranking,
                },
            });

            if !seen_players.contains(&player_identity.id) {
                seen_players.insert(player_identity.id);
                player_tasks.push(PlayerTask {
                    identity: player_identity,
                    position: position.clone(),
                });
            }
        }

        Ok((rankings, player_tasks))
    }
}

fn parse_cell_as_number<T: FromStr>(
    cell: &scraper::element_ref::ElementRef,
    field_name: &str,
) -> T {
    cell.text()
        .collect::<String>()
        .parse()
        .unwrap_or_else(|_| panic!("{} should always be present", field_name))
}

fn get_player_identity(player_cell: &scraper::element_ref::ElementRef) -> PlayerIdentity {
    let player_id = player_cell
        .select(&Selector::parse("div").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("data-player")
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Player ID should always be present");
    let team = Team::from_str(
        player_cell
            .select(&Selector::parse("span").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim_matches(&['(', ')'][..]),
    )
    .unwrap();
    let player_url_element = player_cell
        .select(&Selector::parse("a").unwrap())
        .next()
        .unwrap();
    let bio_url = player_url_element
        .value()
        .attr("href")
        .unwrap_or("")
        .replace("/players/", "/schedule/")
        .to_string();
    let name = player_url_element.text().collect::<String>();

    PlayerIdentity {
        id: player_id,
        bio_url,
        name,
        team,
    }
}

fn get_position_ranking(
    position_cell: &scraper::element_ref::ElementRef,
    re: &Regex,
) -> (Position, i32) {
    let position_text = position_cell.text().collect::<String>();
    let caps = re
        .captures(&position_text)
        .expect("Position and ranking should always be present");
    (
        Position::from_str(&caps[1]).unwrap(),
        caps[2]
            .parse::<i32>()
            .expect("Position ranking should always be present"),
    )
}
