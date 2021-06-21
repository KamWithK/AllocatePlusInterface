mod browser;
mod parser;
mod scheduler;

use std::path::PathBuf;
use thirtyfour::prelude::{DesiredCapabilities, WebDriverResult};
use browser::Browse;
use parser::{Group, Activity, parse};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut chrome_driver = DesiredCapabilities::chrome();
    let browser_profile_dir = PathBuf::from("browser_profile").canonicalize().unwrap();
    chrome_driver.add_chrome_arg(&format!("user-data-dir={}", browser_profile_dir.to_str().unwrap().trim_start_matches("\\\\?\\")))?;
    chrome_driver.set_headless()?;

    let browser = Browse::build_browser(&chrome_driver).await;

    let (username, password, auth_key) = ("", "", "");
    let session = browser.login(&username, &password, &auth_key).await?;

    let raw_data = browser.get_data(session).await?;
    browser.driver.quit().await?;

    let units = parse(raw_data, "S2-01");
    let groups: Vec<&Group> = units.iter().flat_map(|unit| &unit.groups).collect();
    let activities: Vec<&Activity> = groups.iter().flat_map(|group| &group.activities).collect();

    Activity::get_collisions(activities.as_slice(), 15, 6, 11);

    Ok(())
}
