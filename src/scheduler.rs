use std::ops::Add;
use std::convert::TryInto;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration};
use std::clone::Clone;
use std::marker::Copy;

#[derive(Copy, Clone)]
pub struct Timeblock {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime
}

pub struct Day {
    pub times: Vec<Timeblock>
}

pub struct Week {
    pub days: Vec<Day>
}

impl Timeblock {
    pub fn from_duration(start: NaiveDateTime, duration: Duration) -> Timeblock {
        Timeblock {
            start,
            end: start.add(duration)
        }
    }

    // Return true when there is a collision
    pub fn check_collision(&self, other: &Timeblock) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn check_collisions(&self, others: &[Timeblock]) -> Vec<bool> {
        others.iter().map(|other| self.check_collision(other)).collect()
    }

    pub fn check_nested_collisions(first: &[Timeblock], second: &[Timeblock]) -> Vec<Vec<bool>> {
        first.iter().map(|timeblock| timeblock.check_collisions(second)).collect()
    }
}

impl Day {
    pub fn from_frequency(frequency: usize, start: u32, end: u32, day: u32) -> Day {
        let seconds_to_datetime = |seconds| Timeblock::from_duration(
            NaiveDateTime::new(
                NaiveDate::from_ymd(1, 1, day),
                NaiveTime::from_num_seconds_from_midnight(seconds, 0)
            ),
            Duration::seconds(frequency.try_into().unwrap())
        );
        let times = (start..end).step_by(frequency).map(seconds_to_datetime).collect();

        Day {
            times
        }
    }

    pub fn num_collisions(&self, timeblocks:  &[Timeblock]) -> Vec<i64> {
        let to_bool = |accumulated: i64, collides: &bool| accumulated.add(i64::from(*collides));
        let sum_days_collisions = |collisions: &Vec<bool>| collisions.iter().fold(0, to_bool);

        Timeblock::check_nested_collisions(&self.times, timeblocks).iter().map(sum_days_collisions).collect()
    }
}

impl Week {
    pub fn from_frequency(frequency: usize, start: u32, end: u32) -> Week {
        let get_day = |day| Day::from_frequency(frequency, start, end, day);
        let days = (1..5).map(get_day).collect();

        Week {
            days
        }
    }

    pub fn num_collisions(&self, timeblocks: &[Timeblock]) -> Vec<Vec<i64>> {
        let days = |day: &Day| day.num_collisions(timeblocks);

        self.days.iter().map(days).collect()
    }
}
