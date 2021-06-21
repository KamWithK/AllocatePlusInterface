use chrono::{NaiveDateTime, Duration, Datelike};
use serde_json::value::Value;
use crate::scheduler::{Timeblock, Week};

pub const SECONDS_TO_MINUTES: usize = 60;
pub const SECONDS_TO_HOURS: u32 = 60 * 60;

pub struct Unit {
    pub name: String,
    pub groups: Vec<Group>
}

pub struct Group {
    pub name: String,
    pub activities: Vec<Activity>
}

pub struct Activity {
    pub days: Vec<NaiveDateTime>,
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
        let filter_activities = |(_, values): &(&String, &Value)| values.get("selectable").unwrap() != "available";
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
        // Parse dates and times
        let days = values.get("activitiesDays").unwrap().as_array().unwrap();
        let time = values.get("start_time").unwrap().as_str().unwrap();
    
        let to_date = |value: &Value| NaiveDateTime::parse_from_str(&format!("{}-{}", value.as_str().unwrap(), time), "%d/%m/%Y-%H:%M").unwrap();
        let days: Vec<NaiveDateTime> = days.iter().map(to_date).collect();
        let duration = Duration::seconds(values.get("duration").unwrap().as_i64().unwrap());

        // Ensure only day and time change
        let standard_start = days[0].with_year(1).unwrap().with_month(1).unwrap();
    
        // Create activity
        Activity {
            days: days,
            standard_timeblock: Timeblock::from_duration(standard_start, duration),
            popularity: values.get("popularity").unwrap().as_f64().unwrap()
        }
    }

    pub fn get_collisions(activities: &[&Activity], frequency: usize, start: u32, end: u32) -> Vec<Vec<i64>> {
        let week = Week::from_frequency(frequency * SECONDS_TO_MINUTES, start * SECONDS_TO_HOURS, end * SECONDS_TO_HOURS);
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
