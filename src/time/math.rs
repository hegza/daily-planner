use std::ops::{Add, AddAssign, Sub, SubAssign};

use chrono::{NaiveTime, Timelike};

use super::{Clock, Duration, Time};

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

impl<'a, 'b> Sub<&'b Time> for &'a Time {
    type Output = Duration;

    /// Calculates the difference between two times, assuming differences less
    /// than 24 hours
    fn sub(self, rhs: &'b Time) -> Self::Output {
        let self_min = self.hour as i16 * 60 + self.min as i16;
        let rhs_min = rhs.hour as i16 * 60 + rhs.min as i16;

        let mut diff_min = self_min - rhs_min;

        // The end time was on the next day's side
        if diff_min < 0 {
            diff_min = (24 * 60 - rhs_min) + self_min;
        }

        let hours = diff_min / 60;
        let mins = diff_min - hours * 60;
        Duration::hm(hours as i8, mins as i8)
    }
}

impl<'a, 'b> Add<&'b Duration> for &'a Time {
    type Output = Time;

    fn add(self, rhs: &'b Duration) -> Self::Output {
        let nt = NaiveTime::from_hms(self.hour.into(), self.min.into(), 0) + rhs.0;

        Time {
            hour: nt.hour() as u8,
            min: nt.minute() as u8,
        }
    }
}

impl<'a, 'b> Sub<&'b Duration> for &'a Time {
    type Output = Time;

    fn sub(self, rhs: &'b Duration) -> Self::Output {
        let nt = NaiveTime::from_hms(self.hour.into(), self.min.into(), 0) - rhs.0;

        Time {
            hour: nt.hour() as u8,
            min: nt.minute() as u8,
        }
    }
}

impl AddAssign<&Duration> for Time {
    fn add_assign(&mut self, rhs: &Duration) {
        *self = &*self + rhs;
    }
}

impl SubAssign<&Duration> for Time {
    fn sub_assign(&mut self, rhs: &Duration) {
        *self = &*self - rhs;
    }
}

impl<'a, 'b> Add<&'b Duration> for &'a Duration {
    type Output = Duration;

    fn add(self, rhs: &'b Duration) -> Self::Output {
        Duration(self.0 + rhs.0)
    }
}

impl<'a, 'b> Sub<&'b Duration> for &'a Duration {
    type Output = Duration;

    fn sub(self, rhs: &'b Duration) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

impl AddAssign<&Duration> for Duration {
    fn add_assign(&mut self, rhs: &Duration) {
        *self = &*self + rhs;
    }
}

impl SubAssign<&Duration> for Duration {
    fn sub_assign(&mut self, rhs: &Duration) {
        *self = &*self - rhs;
    }
}
