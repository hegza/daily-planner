use std::fmt;

use super::Activity;
use crate::Time;

/// A time box with a set activity and possibly a time slot.
#[derive(Clone, Debug)]
pub struct TimeBox {
    pub time: Option<TimeSlotKind>,
    pub activity: Activity,
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
            TimeSlotKind::Span(start, end) => f.write_str(&format!("{}--{}", start, end)),
        }
    }
}
