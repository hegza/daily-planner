use crate::{
    activity::{Activity, TimeBox, TimeSlotKind},
    schedule::Schedule,
    time::Duration,
    Time,
};

/// Represents a daily template of activities, loadable from a file.
#[derive(Clone, Debug)]
pub struct Template(pub Vec<TimeBoxTemplate>);

#[derive(Clone, Debug)]
pub struct TimeBoxTemplate {
    pub time: Option<TimeSlotTemplate>,
    pub activity: Activity,
}

#[derive(Clone, Debug)]
pub enum TimeSlotTemplate {
    /// e.g. %H:M or +0:15
    Time(TimeTemplate),
    /// e.g. %H:%M--%H:%M or +1:00--+00:30
    Span(TimeTemplate, TimeTemplate),
    // Not implemented, but this also covers the case for absolute time, e.g. 15:00
}

#[derive(Clone, Debug)]
pub enum TimeTemplate {
    /// e.g. %H:M
    TimeFormat,
    /// e.g. +0:15
    RelativeTime(Duration),
}

// <!-- Conversions to concrete types -->
pub struct TemplateMeta {
    pub wake_up: Time,
    pub span_len: Duration,
}

impl Template {
    pub fn schedule(&self, meta: TemplateMeta) -> Schedule {
        let mut time = meta.wake_up;

        let timeboxes = self
            .0
            .iter()
            .map(|time_box_template| time_box_template.time_box(&mut time, &meta.span_len))
            .collect();

        Schedule {
            timeboxes,
            wake_up: meta.wake_up,
        }
    }
}

impl TimeBoxTemplate {
    fn time_box(&self, cur_time: &mut Time, span_len: &Duration) -> TimeBox {
        let time = match &self.time {
            // No associated time
            None => None,
            Some(time_slot_kind) => Some(match time_slot_kind {
                TimeSlotTemplate::Time(time) => match time {
                    // %H:%M, use current time
                    TimeTemplate::TimeFormat => TimeSlotKind::Time(*cur_time),
                    // +0:15, advance by given duration then use that
                    TimeTemplate::RelativeTime(time) => {
                        // Advance time by given duration
                        *cur_time += time;
                        TimeSlotKind::Time(*cur_time)
                    }
                },
                TimeSlotTemplate::Span(start, end) => {
                    let start_time = match start {
                        // %H:%M--, use current time
                        TimeTemplate::TimeFormat => *cur_time,
                        // +0:15--, advance by given duration then use that
                        TimeTemplate::RelativeTime(time) => {
                            // Advance time by given duration
                            *cur_time += time;
                            *cur_time
                        }
                    };
                    let end_time = match end {
                        // --%H:%M, use default span length
                        TimeTemplate::TimeFormat => {
                            // Advance time by the default span length
                            *cur_time += span_len;
                            *cur_time
                        }
                        // --+1:00, use given duration as span length
                        TimeTemplate::RelativeTime(duration) => {
                            // Advance time by given length
                            *cur_time += duration;
                            *cur_time
                        }
                    };
                    TimeSlotKind::Span(start_time, end_time)
                }
            }),
        };

        let activity = self.activity.clone();

        TimeBox {
            time,
            activity,
            ..Default::default()
        }
    }
}
