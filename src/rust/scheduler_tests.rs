use crate::scheduler::{Timeblock, Day, Week};
use chrono::{NaiveDateTime, Duration};

pub const SECONDS_TO_MINUTES: usize = 60;
pub const SECONDS_TO_HOURS: u32 = 60 * 60;

#[test]
fn basic_collision() {
    let example_time = NaiveDateTime::parse_from_str("1/1/1-11:00", "%d/%m/%Y-%H:%M").unwrap();
    let example_timeblock = Timeblock::from_duration(example_time, Duration::hours(3));

    let example_similar_time = NaiveDateTime::parse_from_str("1/1/1-12:00", "%d/%m/%Y-%H:%M").unwrap();
    let example_clashing_timeblock= Timeblock::from_duration(example_similar_time, Duration::hours(1));

    assert_eq!(example_timeblock.check_collision(&example_clashing_timeblock), true)
}

#[test]
fn basic_non_collision() {
    let example_time = NaiveDateTime::parse_from_str("1/1/1-10:00", "%d/%m/%Y-%H:%M").unwrap();
    let example_timeblock = Timeblock::from_duration(example_time, Duration::minutes(30));

    let example_similar_time = NaiveDateTime::parse_from_str("1/1/1-11:00", "%d/%m/%Y-%H:%M").unwrap();
    let example_non_clashing_timeblock= Timeblock::from_duration(example_similar_time, Duration::hours(1));

    assert_eq!(example_timeblock.check_collision(&example_non_clashing_timeblock), false)
}

#[test]
fn single_day() {
    let example_time = NaiveDateTime::parse_from_str("1/1/1-10:00", "%d/%m/%Y-%H:%M").unwrap();
    let example_timeblock = Timeblock::from_duration(example_time, Duration::minutes(60));

    let (frequency, start, end) = (60 * SECONDS_TO_MINUTES, 7 * SECONDS_TO_HOURS, 10 * SECONDS_TO_HOURS);

    let example_day = Day::from_frequency(frequency, start, end, 1);

    assert_eq!(example_timeblock.check_collisions(&example_day.times), vec![false, false, false])
}

#[test]
fn week() {
    let (frequency, start, end) = (60 * SECONDS_TO_MINUTES, 7 * SECONDS_TO_HOURS, 10 * SECONDS_TO_HOURS);

    let example_week = Week::from_frequency(frequency, start, end);
    let example_day = Day::from_frequency(frequency, start, end, 3);

    assert_eq!(example_week.num_collisions(&example_day.times), [[0, 0, 0], [0, 0, 0], [1, 1, 1], [0, 0, 0], [0, 0, 0]]);
}
