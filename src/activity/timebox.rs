use crate::time::Duration;

use super::Activity;
use super::TimeSlotKind;

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

impl TimeBox {
    pub fn adjust_absolute(&mut self, adjust_duration: &Duration, adjust_start: bool) {
        if let Some(time) = &mut self.time {
            time.adjust_absolute(adjust_duration, adjust_start)
        }
    }
}
