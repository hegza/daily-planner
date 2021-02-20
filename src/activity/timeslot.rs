use std::fmt;

use crate::{schedule::Schedule, time::Duration, Time};

#[derive(Clone, Debug)]
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
