use serde::ser::Serialize;
use serde_json::value::Value;
use json_value_merge::Merge;
use thirtyfour::prelude::*;

pub struct Browse {
    pub driver: WebDriver
}

#[derive(Clone)]
pub struct SessionInfo {
    pub student_number: String,
    pub session_ss: String
}

impl Browse {
    pub async fn build_browser<T: Serialize>(caps: &T) -> Browse {
        Browse {
            driver: WebDriver::new("http://localhost:4444", caps).await.unwrap()
        }
    }

    pub async fn login(&self, username: &str, password: &str, auth_key: &str) -> WebDriverResult<SessionInfo> {
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

        if self.driver.query(By::Id("enrol-header")).exists().await? == false {
            Err(thirtyfour::error::WebDriverError::RequestFailed("Failed to Login".to_owned()))
        } else {
            let student_number = self.driver.execute_script(&"return data.student.student_code").await?.value().to_string();
            let ss = self.driver.execute_script(&"return ss").await?.value().to_string();

            Ok(SessionInfo{
                student_number: student_number.trim_matches('"').to_string(),
                session_ss: ss.trim_matches('"').to_string()
            })
        }
    }

    fn get_request(session: SessionInfo, unit: &str, group: &str) -> String {
        let url = format!(
            "https://my-timetable.monash.edu/odd/rest/student/{}/subject/{}/group/{}/activities/?ss={}",
            session.student_number, unit, group, session.session_ss
        );
        format!("return await (await fetch('{}')).json()", url)
    }

    pub async fn get_data(&self, session: SessionInfo) -> WebDriverResult<Value> {
        if self.driver.query(By::Id("enrol-header")).exists().await? == false {
            Err(thirtyfour::error::WebDriverError::RequestFailed("Failed to Login".to_owned()))
        } else {
            // Get student enrolement details
            let student_enrolments = self.driver.execute_script(&"return data.student.student_enrolment").await?;

            // Empty json Value
            let mut data = Value::default();

            for (unit, values) in student_enrolments.value().as_object().unwrap() {
                for group in values.get("groups").unwrap().as_object().unwrap().keys() {
                    let request = &Browse::get_request(session.clone(), unit, group);
                    data.merge(self.driver.execute_script(request).await?.value().to_owned());
                }
            }

            Ok(data)
        }
    }
}
