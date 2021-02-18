use std::fmt;

use super::Activity;
use crate::{time::Duration, Time};

/// A time box with a set activity and possibly a time slot.
#[derive(Clone, Debug)]
pub struct TimeBox {
    pub time: Option<TimeSlotKind>,
    pub activity: Activity,
    pub done: bool,
    pub adjust_policy: AdjustPolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdjustPolicy {
    Normal,
    /// This time does not move unless moved as the primary item
    Fixed,
}

#[derive(Clone, Debug)]
pub enum TimeSlotKind {
    Time(Time),
    Span(Time, Time),
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

impl Default for TimeBox {
    fn default() -> Self {
        TimeBox {
            time: None,
            activity: Activity::default(),
            done: false,
            adjust_policy: AdjustPolicy::Normal,
        }
    }
}
