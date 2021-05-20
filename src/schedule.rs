use chrono::NaiveTime;

use crate::{dom::TimeBox, editing::cursor::ContentCursor, Time};

/// Main data structure
#[derive(Clone, Debug)]
pub struct Schedule {
    pub timeboxes: Vec<TimeBox>,
    pub wake_up: Time,
    pub sunrise: Option<NaiveTime>,
    pub sunset: Option<NaiveTime>,
}

impl Schedule {
    pub fn mut_line_at_cursor(&mut self, cursor: &ContentCursor) -> &mut TimeBox {
        let line = cursor.map_to_line();
        &mut self.timeboxes[line]
    }

    pub fn mut_line(&mut self, idx: usize) -> Option<&mut TimeBox> {
        self.timeboxes.get_mut(idx)
    }

    pub fn line(&self, idx: usize) -> Option<&TimeBox> {
        self.timeboxes.get(idx)
    }
}
