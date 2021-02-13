mod math;

use std::{fmt, str::FromStr};

use crate::template::template::TimeTemplate;

/// Represents naive time. May be used relatively in the span of 24 hours starting from wake-up, e.g. if wake up was at 10:00, 8:30 could be later than 10:00.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    hour: u8,
    min: u8,
}

impl Time {
    pub fn hm(hour: u8, min: u8) -> Time {
        Time { hour, min }
    }

    pub fn adjust(&mut self, duration: &Duration) {
        *self += duration;
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

/// Represents both positive and negative durations.
#[derive(Clone, Copy, Debug)]
pub struct Duration(chrono::Duration);

impl Duration {
    pub fn hours(hours: i8) -> Duration {
        Duration(chrono::Duration::hours(hours.into()))
    }
    pub fn hm(hours: i8, minutes: i8) -> Duration {
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
        Duration::hm(time.hour as i8, time.min as i8)
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

pub struct Clock {
    hour: u8,
    min: u8,
}

impl From<Clock> for Time {
    fn from(c: Clock) -> Self {
        Time {
            hour: c.hour,
            min: c.min,
        }
    }
}

impl From<Clock> for Duration {
    fn from(c: Clock) -> Self {
        Duration::hm(c.hour as i8, c.min as i8)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time: Time = (*self).into();
        let hour = time.hour;
        let min = time.min;
        if min != 0 && hour != 0 {
            write!(f, "{}h{}m", hour, min)
        } else if hour != 0 {
            write!(f, "{}h", hour)
        } else {
            write!(f, "{}m", min)
        }
    }
}
