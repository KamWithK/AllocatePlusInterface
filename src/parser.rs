use chrono::naive::NaiveDateTime;
use serde_json::value::Value;

pub struct Unit {
    pub name: String,
    pub groups: Vec<Group>
}

pub struct Group {
    pub name: String,
    pub preferences: Vec<Preference>
}

pub struct Preference {
    pub days: Vec<NaiveDateTime>,
    pub popularity: f64
}

pub fn parse(raw_data: Value, semester: &str) -> Vec<Unit> {
    let mut units = Vec::new();

    for (unit, values) in raw_data.as_object().unwrap() {
        // Only get this semesters units
        if values.get("semester").unwrap() != semester {
            continue
        }

        // Create Unit
        units.push(
            Unit {
                name: unit.to_string(),
                groups: Vec::new()
            }
        );

        for (group, values) in values.get("groups").unwrap().as_object().unwrap() {
            // Create group
            units.last_mut().unwrap().groups.push(
                Group {
                    name: group.to_string(),
                    preferences: Vec::new()
                }
            );

            for (_, values) in values.get("activities").unwrap().as_object().unwrap() {
                // Avoid non-selectable units
                if values.get("selectable").unwrap() != "available" {
                    continue
                }

                // Parse dates and times
                let days = values.get("activitiesDays").unwrap().as_array().unwrap();
                let time = values.get("start_time").unwrap().as_str().unwrap();

                let to_date = |value: &Value| NaiveDateTime::parse_from_str(&format!("{}-{}", value.as_str().unwrap(), time), "%d/%m/%Y-%H:%M").unwrap();
                let days = days.iter().map(to_date).collect();

                // Create preference
                units.last_mut().unwrap().groups.last_mut().unwrap().preferences.push(
                    Preference {
                        days: days,
                        popularity: 0.
                    }
                );
            }
        }
    }
    
    units
}
