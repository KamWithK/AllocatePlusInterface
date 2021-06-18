use serde::ser::Serialize;
use serde_json::value::Value;
use std::time::Duration;
use thirtyfour::prelude::*;
use thirtyfour::support::sleep;
use md5;

pub struct Browse {
    pub driver: WebDriver
}

impl Browse {
    pub async fn build_browser<T: Serialize>(caps: &T) -> Browse {
        Browse {
            driver: WebDriver::new("http://localhost:4444", caps).await.unwrap()
        }
    }

    pub async fn login(&self, username: &str, password: &str, auth_key: &str) -> WebDriverResult<()> {
        self.driver.get("https://my-timetable.monash.edu/odd/student").await?;

        // Fill in username and password
        self.driver.find_element(By::Id("okta-signin-username")).await?.send_keys(username).await?;
        self.driver.find_element(By::Id("okta-signin-password")).await?.send_keys(password).await?;
        self.driver.find_element(By::Id("okta-signin-submit")).await?.click().await?;

        // Fill in Google authenticator key
        if auth_key != "" {
            self.driver.query(By::Css("input[type=tel]")).first().await?.send_keys(auth_key).await?;
            self.driver.query(By::Css("input[type=submit]")).first().await?.click().await?;
        }

        Ok(())
    }

    pub async fn preload_all(&self) -> WebDriverResult<Value> {
        if self.driver.query(By::Id("enrol-header")).exists().await? == false {
            Err(thirtyfour::error::WebDriverError::NotFound("".to_string(), "".to_string()))
        } else {
            let student_enrolments = self.driver.execute_script(&"return data.student.student_enrolment").await?;

            for (unit, values) in student_enrolments.value().as_object().unwrap() {
                for group in values.get("groups").unwrap().as_object().unwrap().keys() {
                    let hash = md5::compute(format!("{}{}", unit, group));
                    self.driver.execute_script(&format!("showGroup('{:x}')", hash)).await?;
                    sleep(Duration::from_millis(100)).await;
                }
            }

            Ok(self.driver.execute_script(&"return data.student.student_enrolment").await?.value().to_owned())
        }
    }
}
