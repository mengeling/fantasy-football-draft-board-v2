use anyhow::Result;
use headless_chrome::Browser;

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

    // Parse the HTML using the scraper crate
    let document = scraper::Html::parse_document(table_html);
    let selector = scraper::Selector::parse("tbody tr.player-row").unwrap();

    // Find all matching elements
    let player_rows: Vec<_> = document.select(&selector).collect();

    // Check if we have player rows and print the last one
    if let Some(last_row) = player_rows.last() {
        println!("Last player row HTML: {}", last_row.html());
    } else {
        println!("No player rows found.");
    }

    // Pause the program to allow viewing of console output
    use std::io::{self, Write};
    print!("Press Enter to exit...");
    io::stdout().flush()?; // Ensure the message is printed before waiting for input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
