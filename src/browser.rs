use serde::ser::Serialize;
use serde_json::value::Value;
use json_value_merge::Merge;
use thirtyfour::prelude::*;

pub struct Browse {
    pub driver: WebDriver
}

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

    fn get_request(session: &SessionInfo, field: &str, unit: &str, group: &str) -> String {
        let url = format!(
            "rest/student/{}/subject/{}/group/{}/{}/?ss={}",
            session.student_number, unit, group, field, session.session_ss
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
                    let activities_request = Browse::get_request(&session, "activities", unit, group);
                    let mut activities_result = self.driver.execute_script(&activities_request).await?.value().to_owned();

                    let popularities_request = Browse::get_request(&session, "popularities", unit, group);
                    let popularities_result = self.driver.execute_script(&popularities_request).await?.value().to_owned();

                    let filter_popularities = |(key, _): &(&String, &Value)| key.contains("activity");
                    let popularities: Vec<(&String, &Value)> = popularities_result.as_object().unwrap().iter().filter(filter_popularities).collect();

                    for ((_, values), (_, popularity)) in activities_result.as_object_mut().unwrap().iter_mut().zip(popularities) {
                        values["popularity"] = serde_json::Value::String(popularity.to_owned().get("popularity").unwrap().to_owned().to_string());
                    }

                    data.merge(activities_result);
                }
            }

            Ok(data)
        }
    }
}
