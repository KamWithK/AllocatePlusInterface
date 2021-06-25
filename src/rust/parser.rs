use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration};
use serde::{Serialize, Deserialize};
use serde_json::value::Value;
use crate::scheduler::{Timeblock, Week};

#[derive(Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub groups: Vec<Group>
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub activities: Vec<Activity>
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub standard_timeblock: Timeblock,
    pub popularity: f64
}

impl Unit {
    pub fn parse_unit(unit: &str, values: Value) -> Unit {
        // Get groups
        let parse_groups = |(group, values): (&String, &Value)| Group::parse_group(group, values.to_owned());
        let groups = values.get("groups").unwrap().as_object().unwrap().iter().map(parse_groups).collect();

        // Create unit
        Unit {
            name: unit.to_string(),
            groups
        }
    }
}

impl Group {
    pub fn parse_group(group: &str, values: Value) -> Group {
        // Get activities
        let filter_activities = |(_, values): &(&String, &Value)| values.get("selectable").unwrap() == "available";
        let parse_activities = |(_, values): (_, &Value)| Activity::parse_activity(values.to_owned());
        let activities = values.get("activities").unwrap().as_object().unwrap().iter().filter(filter_activities).map(parse_activities).collect();

        // Create group
        Group {
            name: group.to_string(),
            activities
        }
    }
}

impl Activity {
    pub fn parse_activity(values: Value) -> Activity {
        // Get attributes
        let day = values.get("day_of_week").unwrap().as_str().unwrap().parse().unwrap();
        let week = values.get("start_time").unwrap().as_str().unwrap();
        let duration = values.get("duration").unwrap().as_str().unwrap().parse().unwrap();

        let popularity = values.get("popularity").unwrap().as_str().unwrap().parse().unwrap();

        let standard_timeblock = Timeblock::from_duration(
            NaiveDateTime::new(
                NaiveDate::from_weekday_of_month(1, 1, day, 1),
                NaiveTime::parse_from_str(week, &"%H:%M").unwrap()
            ),
            Duration::minutes(duration)
        );
    
        // Create activity
        Activity {
            standard_timeblock,
            popularity
        }
    }

    pub fn get_collisions(activities: &[&Activity], frequency: usize, start: u32, end: u32) -> Vec<Vec<i64>> {
        let week = Week::from_frequency(frequency, start, end);
        let activities: Vec<Timeblock> = activities.iter().map(|activity| activity.standard_timeblock).collect();

        week.num_collisions(activities.as_slice())
    }
}

pub fn parse(raw_data: Value, semester: &str) -> Vec<Unit> {
    let filter_units = |(_, values): &(&String, &Value)| values.get("semester").unwrap() == semester;
    let parse_units = |(group, values): (&String, &Value)| Unit::parse_unit(group, values.to_owned());
    let units = raw_data.as_object().unwrap().iter().filter(filter_units).map(parse_units).collect();
    
    units
}
