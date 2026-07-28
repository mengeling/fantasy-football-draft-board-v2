use anyhow::Result;
use headless_chrome::Tab;
use regex::Regex;
use scraper::{Html, Selector};
use std::str::FromStr;
use std::time::{Duration, Instant};

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
            let table_html = self.scrape_ranking_table(url)?;
            ranking_tables.push((table_html, scoring_settings));
        }

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

    fn scrape_ranking_table(&self, url: &str) -> Result<String> {
        const MAX_ATTEMPTS: u32 = 3;
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 1..=MAX_ATTEMPTS {
            match self.try_scrape_ranking_table(url) {
                Ok(html) => return Ok(html),
                Err(e) => {
                    eprintln!(
                        "Rankings scrape attempt {}/{} for {} failed: {}",
                        attempt, MAX_ATTEMPTS, url, e
                    );
                    last_err = Some(e);
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("Failed to scrape {}", url)))
    }

    fn try_scrape_ranking_table(&self, url: &str) -> Result<String> {
        self.tab.navigate_to(url)?;
        self.tab.wait_until_navigated()?;
        self.dismiss_consent_banner();
        self.tab
            .wait_for_element(".select-advanced--view .select-advanced__button")?;
        self.select_ranks_view()?;
        self.wait_for_full_ranks_table(Duration::from_secs(30))?;

        let ranking_table = self.tab.wait_for_element("table#ranking-table")?;
        let html = ranking_table.get_content()?;

        let row_count = Html::parse_document(&html)
            .select(&Selector::parse("tbody tr.player-row").unwrap())
            .count();
        if row_count == 0 {
            return Err(anyhow::anyhow!("Ranking table captured with 0 player rows"));
        }

        Ok(html)
    }

    fn dismiss_consent_banner(&self) {
        // Best-effort: FantasyPros shows a OneTrust cookie banner on fresh
        // (cookie-less) sessions that can interfere with the view dropdown.
        // Prefer rejecting non-essential cookies; fall back to accept/close.
        let _ = self.tab.evaluate(
            r#"(function () {
                var b = document.querySelector(
                    '#onetrust-reject-all-handler, #onetrust-accept-btn-handler, .onetrust-close-btn-handler'
                );
                if (b) { b.click(); return true; }
                return false;
            })()"#,
            false,
        );
    }

    fn select_ranks_view(&self) -> Result<()> {
        // Open the "view" dropdown and pick "Ranks" in a single evaluated step.
        // Matching on textContent (not innerText) avoids a headless race where
        // freshly-opened options have no laid-out text yet.
        let result = self.tab.evaluate(
            r#"(function () {
                var btn = document.querySelector('.select-advanced--view .select-advanced__button');
                if (!btn) return 'no-button';
                btn.click();
                var opts = [].slice.call(document.querySelectorAll(
                    '.select-advanced--view .select-advanced__item .select-advanced-content--button'
                ));
                var ranks = opts.filter(function (o) { return o.textContent.trim() === 'Ranks'; })[0];
                if (!ranks) return 'no-ranks';
                ranks.click();
                return 'ok';
            })()"#,
            false,
        )?;

        match result.value.as_ref().and_then(|v| v.as_str()) {
            Some("ok") => Ok(()),
            other => Err(anyhow::anyhow!(
                "Could not select 'Ranks' view (dropdown state: {:?})",
                other
            )),
        }
    }

    fn wait_for_full_ranks_table(&self, timeout: Duration) -> Result<()> {
        // The table is only safe to capture once (a) the "Ranks" view is active
        // and its Best/Worst/Avg/StdDev columns have rendered as numbers (the
        // default "Overview" view puts non-numeric analytics ratings there), and
        // (b) every row has rendered. The rows populate progressively, so we
        // scroll to the bottom to nudge lazy loading and wait until the row
        // count stops changing before capturing.
        let check = r#"(function () {
            var label = document.querySelector('.select-advanced--view .select-advanced__button-text');
            var ranksActive = !!label && label.innerText.trim() === 'Ranks';
            window.scrollTo(0, document.body.scrollHeight);
            var last = document.querySelector('#ranking-table tbody tr.player-row:last-child');
            if (last) last.scrollIntoView();
            var rows = document.querySelectorAll('#ranking-table tbody tr.player-row');
            var worstNumeric = false;
            if (rows.length) {
                var cells = rows[0].querySelectorAll('td');
                worstNumeric = cells.length >= 8 && /^\d+$/.test((cells[5].innerText || '').trim());
            }
            return JSON.stringify({ ranksActive: ranksActive, worstNumeric: worstNumeric, count: rows.length });
        })()"#;

        let start = Instant::now();
        let mut last_count = 0usize;
        let mut stable_ticks = 0u32;

        loop {
            let parsed = self
                .tab
                .evaluate(check, false)?
                .value
                .and_then(|v| v.as_str().map(str::to_owned))
                .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok());

            if let Some(v) = parsed {
                let ranks_active = v["ranksActive"].as_bool().unwrap_or(false);
                let worst_numeric = v["worstNumeric"].as_bool().unwrap_or(false);
                let count = v["count"].as_u64().unwrap_or(0) as usize;

                if ranks_active && worst_numeric && count > 0 {
                    if count == last_count {
                        stable_ticks += 1;
                        // ~1.2s with an unchanged row count means rendering settled.
                        if stable_ticks >= 3 {
                            return Ok(());
                        }
                    } else {
                        stable_ticks = 0;
                        last_count = count;
                    }
                }
            }

            if start.elapsed() >= timeout {
                return Err(anyhow::anyhow!(
                    "'Ranks' view did not finish rendering within {:?} (last row count: {})",
                    timeout,
                    last_count
                ));
            }
            std::thread::sleep(Duration::from_millis(400));
        }
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
