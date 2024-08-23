// // use reqwest::blocking;
// // use sxd_xpath::{Context, Factory, Value};
// // use xmltree::Element;

// // use std::sync::Arc;

// // use playwright::browser_type::BrowserType;
// // use playwright::browser_type::LaunchOptions;
// use playwright::Playwright;
// // use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), playwright::Error> {
//     let playwright = Playwright::initialize().await?;
//     playwright.prepare()?;
//     let firefox = playwright.firefox();
//     let browser = firefox.launcher().headless(false).launch().await?;
//     let context = browser.context_builder().build().await?;
//     let page = context.new_page().await?;

//     let scoring_settings = "half-point-ppr";
//     let rankings_url =
//         format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");

//     page.goto_builder(&rankings_url).goto().await?;

//     // let playwright = Playwright::initialize().await?;
//     // let browser = playwright.chromium().launch_headless().await?;
//     // let page = browser.new_page().await?;
//     // page.goto(rankings_url).await?;
//     // let element = page
//     //     .wait_for_selector("//*/table[@id='ranking-table']")
//     //     .await?;
//     // let text = element.inner_html().await?;
//     // println!("Element text: {}", text);

//     // Close the browser
//     browser.close().await?;

//     Ok(())

//     // let response = blocking::get(rankings_url);
//     // let html = response.unwrap().text().unwrap();
//     // let factory = Factory::new();

//     // let html_product_selector = parse("div.col").unwrap();
//     // println!("{html}");
// }

// use playwright::Playwright;
// use std::path::Path;

// #[tokio::main]
// async fn main() -> Result<(), playwright::Error> {
//     // Enable debug logging
//     std::env::set_var("DEBUG", "pw:api");

//     let playwright = Playwright::initialize().await?;
//     let firefox = playwright.firefox();

//     let firefox_path = Path::new("/Applications/Firefox.app/Contents/MacOS/firefox");

//     let browser = firefox
//         .launcher()
//         .executable(firefox_path)
//         .headless(false)
//         .launch()
//         .await?;

//     let context = browser.context_builder().build().await?;
//     let page = context.new_page().await?;

//     let scoring_settings = "half-point-ppr";
//     let rankings_url =
//         format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");

//     page.goto_builder(&rankings_url).goto().await?;

//     browser.close().await?;
//     Ok(())
// }

// use reqwest::Error;
// use scraper::{Html, Selector};
// use std::error::Error as StdError;
// use std::fs::File;
// use std::io::Write;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn StdError>> {
//     let scoring_settings = "half-point-ppr";
//     let rankings_url =
//         format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");

//     // Make an HTTP GET request to the URL
//     let response = reqwest::get(&rankings_url).await?;
//     let body = response.text().await?;

//     // Write the HTML content to a file
//     let mut file = File::create("output.html")?;
//     file.write_all(body.as_bytes())?;

//     println!("HTML content written to output.html");

//     // Parse the HTML content
//     let document = Html::parse_document(&body);

//     // Define the CSS selector equivalent to the XPath
//     let selector = Selector::parse("table#ranking-table").unwrap();

//     // Select the element
//     if let Some(element) = document.select(&selector).next() {
//         // Print the outer HTML of the selected element
//         println!("{}", element.html());
//     } else {
//         println!("Element not found");
//     }

//     Ok(())
// }

use anyhow::Result;
use headless_chrome::Browser;

fn main() -> Result<()> {
    // Create a new headless browser
    let browser = Browser::default()?;

    // Open a new tab
    let tab = browser.new_tab()?;

    // Navigate to the URL
    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");
    tab.navigate_to(&rankings_url)?;
    tab.wait_until_navigated()?;

    // Wait for the table to load by waiting for the selector
    let table_element = tab.wait_for_element("table#ranking-table")?;

    // Use JavaScript to find and log the outer HTML of all `player-row` elements
    let player_rows_js = r#"
        var rows = document.querySelectorAll('tbody tr.player-row');
        return Array.from(rows).map(row => row.outerHTML);
    "#;

    let player_rows = table_element.call_js_fn(player_rows_js, vec![], false)?;

    if let Some(player_rows_value) = player_rows.value {
        let rows: Vec<String> = serde_json::from_value(player_rows_value)?;
        println!("Found {} player rows.", rows.len());
        for (i, row_html) in rows.iter().enumerate() {
            println!("Player row {}: {}", i + 1, row_html);
        }
    } else {
        println!("No player rows found or unable to retrieve them.");
    }

    // Pause the program to allow viewing of console output
    use std::io::{self, Write};
    print!("Press Enter to exit...");
    io::stdout().flush()?; // Ensure the message is printed before waiting for input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
