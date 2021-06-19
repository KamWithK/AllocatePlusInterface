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

impl Unit {
    pub fn parse_unit(unit: &str, values: Value) -> Unit {
        // Get groups
        let parse_groups = |(group, values): (&String, &Value)| Group::parse_group(group, values.to_owned());
        let groups: Vec<Group> = values.get("groups").unwrap().as_object().unwrap().iter().map(parse_groups).collect();

        // Create unit
        Unit {
            name: unit.to_string(),
            groups
        }
    }
}

impl Group {
    pub fn parse_group(group: &str, values: Value) -> Group {
        // Get preferences
        let filter_preferences = |(_, values): &(&String, &Value)| values.get("selectable").unwrap() != "available";
        let parse_preferences = |(_, values): (_, &Value)| Preference::parse_preference(values.to_owned());
        let preferences: Vec<Preference> = values.get("activities").unwrap().as_object().unwrap().iter().filter(filter_preferences).map(parse_preferences).collect();

        // Create group
        Group {
            name: group.to_string(),
            preferences
        }
    }
}

impl Preference {
    pub fn parse_preference(values: Value) -> Preference {
        // Parse dates and times
        let days = values.get("activitiesDays").unwrap().as_array().unwrap();
        let time = values.get("start_time").unwrap().as_str().unwrap();
    
        let to_date = |value: &Value| NaiveDateTime::parse_from_str(&format!("{}-{}", value.as_str().unwrap(), time), "%d/%m/%Y-%H:%M").unwrap();
        let days = days.iter().map(to_date).collect();
    
        // Create preference
        Preference {
            days: days,
            popularity: 0.
        }
    }
}

pub fn parse(raw_data: Value, semester: &str) -> Vec<Unit> {
    let filter_units = |(_, values): &(&String, &Value)| values.get("semester").unwrap() == semester;
    let parse_units = |(group, values): (&String, &Value)| Unit::parse_unit(group, values.to_owned());
    let units: Vec<Unit> = raw_data.as_object().unwrap().iter().filter(filter_units).map(parse_units).collect();
    
    units
}
