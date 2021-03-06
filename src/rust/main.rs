mod browser;
mod parser;
mod scheduler;

#[cfg(test)]
mod scheduler_tests;

#[macro_use] extern crate rocket;

use thirtyfour::{ChromeCapabilities, prelude::DesiredCapabilities};
use browser::Browse;
use parser::{Unit, Group, Activity, parse};
use serde::{Serialize, Deserialize};
use rocket::fs::{FileServer, relative};
use rocket::State;
use rocket::serde::json::Json;

pub const SECONDS_TO_MINUTES: usize = 60;
pub const SECONDS_TO_HOURS: u32 = 60 * 60;

#[derive(Serialize, Deserialize)]
struct LoginDetails {
    pub username: String,
    pub password: String,
    pub auth_key: String
}

#[post("/login", data="<login_details>")]
async fn login(login_details: Json<LoginDetails>, chrome_driver: &State<ChromeCapabilities>) -> Json<Vec<Unit>> {
    let browser = Browse::build_browser(chrome_driver.inner()).await;
    let session = browser.login(&login_details.username, &login_details.password, &login_details.auth_key).await.unwrap();

    let raw_data = browser.get_data(session).await.unwrap();
    browser.driver.quit().await.unwrap();
    
    let units = parse(raw_data, "S2-01");

    Json(units)
}

#[get("/collisions?<units>")]
async fn collisions(units: Json<Vec<Unit>>) -> Json<Vec<Vec<i64>>> {
    let groups: Vec<&Group> = units.iter().flat_map(|unit| &unit.groups).collect();
    let activities: Vec<&Activity> = groups.iter().flat_map(|group| &group.activities).collect();

    let (frequency, start, end) = (30 * SECONDS_TO_MINUTES, 7 * SECONDS_TO_HOURS, 22 * SECONDS_TO_HOURS);
    let activity_collisions = Activity::get_collisions(activities.as_slice(), frequency, start, end);

    Json(activity_collisions)
}

#[launch]
fn rocket() -> _ {
    let mut chrome_driver = DesiredCapabilities::chrome();
    chrome_driver.set_headless().unwrap();

    rocket::build()
        .mount("/", FileServer::from(relative!("public")))
        .mount("/api", routes![login, collisions])
        .manage(chrome_driver)
}
