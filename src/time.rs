use chrono::{NaiveTime, Timelike};
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::template::template::TimeTemplate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    hour: u8,
    min: u8,
}

impl Time {
    pub fn hm(hour: u8, min: u8) -> Time {
        Time { hour, min }
    }

    pub fn round_to_quarter(mut self) -> Time {
        let which_quarter = ((self.min + 7) % 60) / 15;
        match which_quarter {
            0 => {
                if self.min > 45 {
                    self.hour += 1;
                }
                self.min = 0;
            }
            1 => {
                self.min = 15;
            }
            2 => {
                self.min = 30;
            }
            3 => {
                self.min = 45;
            }
            _ => unreachable!(),
        }
        self
    }
    pub fn round_to_half(mut self) -> Time {
        let which_half = ((self.min + 15) % 60) / 30;
        match which_half {
            0 => {
                if self.min > 30 {
                    self.hour += 1;
                }
                self.min = 0;
            }
            1 => self.min = 30,
            _ => unreachable!(),
        }
        self
    }
}

#[test]
fn time_rounding() {
    let t1 = Time::hm(0, 58).round_to_quarter();
    assert_eq!(t1, Time::hm(1, 0));

    let t2 = Time::hm(3, 2).round_to_quarter();
    assert_eq!(t2, Time::hm(3, 0));

    let t3 = Time::hm(2, 43).round_to_quarter();
    assert_eq!(t3, Time::hm(2, 45));
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

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        let hour = duration.0.num_hours();
        // `Time` can only represent times less than 24 hours
        assert!(
            hour <= 24,
            "{}: Time can only represent times less than 24 hours",
            hour
        );
        Time {
            hour: hour as u8,
            min: (duration.0.num_minutes() - 60 * hour) as u8,
        }
    }
}

impl From<Time> for Duration {
    fn from(time: Time) -> Self {
        Duration::hm(time.hour, time.min)
    }
}

/// e.g. 15:01
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.min)
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

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

pub struct Clock {
    hour: u8,
    min: u8,
}

impl Clock {
    pub fn difference(start: &Time, end: &Time) -> Clock {
        let start_minutes = start.hour as i64 * 60 + start.min as i64;
        let end_minutes = end.hour as i64 * 60 + end.min as i64;
        let diff_minutes = if start_minutes < end_minutes {
            end_minutes - start_minutes
        } else {
            start_minutes - end_minutes
        };

        let hour = (diff_minutes / 60) as u8;
        let min = (diff_minutes - (hour as i64) * 60) as u8;

        Clock { hour, min }
    }
}

impl From<Clock> for Time {
    fn from(c: Clock) -> Self {
        Time {
            hour: c.hour,
            min: c.min,
        }
    }
}
