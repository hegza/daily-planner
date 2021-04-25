use std::{cmp, fmt};

use crate::{schedule::Schedule, time::Duration, Time};

/// A time or a span of time.
#[derive(Clone, Debug, PartialEq)]
pub enum TimeSlotKind {
    Time(Time),
    Span(Time, Time),
}

impl TimeSlotKind {
    pub fn inherit_time(insert_at: usize, schedule: &Schedule) -> TimeSlotKind {
        // Take the first item with a time above (at - 1)
        for item in schedule.timeboxes[..insert_at].iter().rev() {
            if let Some(t) = &item.time {
                return t.clone();
            }
        }

        // If no times found, return wake up
        TimeSlotKind::Time(schedule.wake_up)
    }
    pub fn adjust_absolute(&mut self, adjust_duration: &Duration, adjust_start: bool) {
        match self {
            TimeSlotKind::Time(t) => t.adjust(&adjust_duration),
            TimeSlotKind::Span(start, end) => {
                if adjust_start {
                    start.adjust(&adjust_duration);
                };
                end.adjust(&adjust_duration);
            }
        };
    }
}

impl fmt::Display for TimeSlotKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeSlotKind::Time(t) => t.fmt(f),
            TimeSlotKind::Span(start, end) => {
                let len: Duration = Duration::from(end - start);
                f.write_str(&format!("{}--{} ({})", start, end, len))
            }
        }
    }
}

impl PartialOrd for TimeSlotKind {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let a = match self {
            TimeSlotKind::Time(t) => t,
            TimeSlotKind::Span(s, _) => s,
        };
        let b = match other {
            TimeSlotKind::Time(t) => t,
            TimeSlotKind::Span(s, _) => s,
        };
        a.partial_cmp(b)
    }
}
