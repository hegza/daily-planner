use crate::{activity::TimeBox, editor::cursor::ContentCursor, Time};

/// Main data structure
#[derive(Clone, Debug)]
pub struct Schedule {
    pub timeboxes: Vec<TimeBox>,
    pub wake_up: Time,
}

impl Schedule {
    pub fn mut_line(&mut self, cursor: &ContentCursor) -> &mut TimeBox {
        let line = cursor.map_to_line(self);
        &mut self.timeboxes[line]
    }
}
