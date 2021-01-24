use chrono::{NaiveTime, Timelike};
use std::{
    convert::TryInto,
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::template::template::TimeTemplate;

#[derive(Clone, Copy, Debug)]
pub struct Time {
    hour: u8,
    min: u8,
}

impl Time {
    pub fn hm(hour: u8, min: u8) -> Time {
        Time { hour, min }
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        let hour = duration.0.num_hours();
        // `Time` can only represent times less than 24 hours
        assert!(hour <= 24);
        Time {
            hour: hour as u8,
            min: (duration.0.num_minutes() - 60 * hour) as u8,
        }
    }
}

/// e.g. 15:01
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.min)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Duration(chrono::Duration);

impl Duration {
    pub fn hours(hours: u8) -> Duration {
        Duration(chrono::Duration::hours(hours.into()))
    }
    pub fn hm(hours: u8, minutes: u8) -> Duration {
        Duration(chrono::Duration::minutes(
            hours as i64 * 60 + minutes as i64,
        ))
    }
}

// <!-- deserialize -->

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TimeTemplate::from_str(s) {
            Ok(tt) => match tt {
                TimeTemplate::TimeFormat => Err(format!(
                    "'{}': cannot time format template as a concrete time",
                    s
                )),
                TimeTemplate::RelativeTime(t) => Ok(t.into()),
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

// <!-- maths -->

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        let nt = NaiveTime::from_hms(self.hour.into(), self.min.into(), 0) + rhs.0;

        Time {
            hour: nt.hour() as u8,
            min: nt.minute() as u8,
        }
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        let nt = NaiveTime::from_hms(self.hour.into(), self.min.into(), 0) - rhs.0;

        Time {
            hour: nt.hour() as u8,
            min: nt.minute() as u8,
        }
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign<Duration> for Time {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}
