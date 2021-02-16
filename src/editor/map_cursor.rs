use super::cursor::{MappedPos, TerminalPos};
use crate::schedule::Schedule;

impl Schedule {
    /// Maps content to cursor (0, 0) being the first character on the first item of the day at e.g. position (13, 0). Returns None on out-of-bounds.
    pub fn map_content_to_screen(
        &self,
        pos: &MappedPos,
        render_y: u16,
        render_height: u16,
    ) -> Option<TerminalPos> {
        if pos.line as u16 >= render_height {
            return None;
        }
        let out_y = pos.line as u16 + render_y;

        let content_on_line = &self.timeboxes[pos.line].activity.summary;
        if pos.col >= content_on_line.chars().count() + 1 {
            return None;
        }

        let out_x = pos.col as u16 + self.time_col_width() as u16 + 1;

        Some(TerminalPos::new(out_x, out_y))
    }

    /// Maps cursor to content (0, 0) being top-left of screen. Returns None on out-of-bounds.
    pub fn map_cursor_to_content(
        &self,
        cursor: &TerminalPos,
        render_y: u16,
        render_height: u16,
    ) -> Option<MappedPos> {
        let line_idx = match self.map_cursor_to_line(cursor.vpos, render_y, render_height) {
            Some(idx) => idx,
            None => return None,
        };

        // Content == the summary of the activity
        let content_on_line = &self.timeboxes[line_idx].activity.summary;

        let char_idx = match (cursor.hpos as usize).checked_sub(self.time_col_width() + 1) {
            Some(char_idx) => {
                if char_idx >= content_on_line.chars().count() + 2 {
                    // Out-of-bounds, content is leftwards
                    return None;
                }

                char_idx
            }
            // Out-of-bounds, content is rightwards
            None => return None,
        };

        Some(MappedPos::new(char_idx as u16, line_idx as u16))
    }

    /// Maps cursor to line 0 being the topmost line, None on OOB.
    pub fn map_cursor_to_line(
        &self,
        cursor_y: u16,
        render_y: u16,
        render_height: u16,
    ) -> Option<usize> {
        let line_idx = match cursor_y.checked_sub(render_y) {
            Some(idx) => {
                if idx >= render_height {
                    // Out-of-bounds, content is above
                    return None;
                }
                idx as usize
            }
            // Out-of-bounds, content is below
            None => return None,
        };

        Some(line_idx)
    }
}
