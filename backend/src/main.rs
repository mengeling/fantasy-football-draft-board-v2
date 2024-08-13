// use reqwest::blocking;
// use sxd_xpath::{Context, Factory, Value};
// use xmltree::Element;

// use std::sync::Arc;

use playwright::browser_type::BrowserType;
use playwright::browser_type::LaunchOptions;
use playwright::Playwright;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scoring_settings = "half-point-ppr";
    let rankings_url =
        format!("https://www.fantasypros.com/nfl/rankings/{scoring_settings}-cheatsheets.php");

    let playwright = Playwright::initialize().await?;
    let browser = playwright.chromium().launch_headless().await?;
    let page = browser.new_page().await?;
    page.goto(rankings_url).await?;
    let element = page
        .wait_for_selector("//*/table[@id='ranking-table']")
        .await?;
    let text = element.inner_html().await?;
    println!("Element text: {}", text);

    // Close the browser
    browser.close().await?;

    Ok(())

    // let response = blocking::get(rankings_url);
    // let html = response.unwrap().text().unwrap();
    // let factory = Factory::new();

    // let html_product_selector = parse("div.col").unwrap();
    // println!("{html}");
}
