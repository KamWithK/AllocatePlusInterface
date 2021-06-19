mod browser;
mod parser;

use std::path::PathBuf;
use thirtyfour::prelude::{DesiredCapabilities, WebDriverResult};
use browser::Browse;
use parser::parse;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut chrome_driver = DesiredCapabilities::chrome();
    let browser_profile_dir = PathBuf::from("browser_profile").canonicalize().unwrap();
    chrome_driver.add_chrome_arg(&format!("user-data-dir={}", browser_profile_dir.to_str().unwrap().trim_start_matches("\\\\?\\")))?;

    let browser = Browse::build_browser(&chrome_driver).await;

    let (username, password, auth_key) = ("", "", "");
    let session = browser.login(&username, &password, &auth_key).await?;

    let raw_data = browser.get_data(session).await?;
    browser.driver.quit().await?;
    parse(raw_data, "S2-01");


    Ok(())
}
