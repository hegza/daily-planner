use crate::schedule::Schedule;

use super::cursor::{MappedPos, TerminalPos};

impl TerminalPos {
    pub fn new<U16>(hpos: U16, vpos: U16) -> TerminalPos
    where
        U16: Into<u16>,
    {
        TerminalPos {
            hpos: hpos.into(),
            vpos: vpos.into(),
        }
    }

    /// Maps cursor to content (0, 0) being top-left of screen. Returns None on out-of-bounds.
    pub fn map_to_content(
        &self,
        render_y: u16,
        render_height: u16,
        schedule: &Schedule,
    ) -> Option<MappedPos> {
        let line_idx = match Self::map_to_line(self.vpos, render_y, render_height) {
            Some(idx) => idx,
            None => return None,
        };

        // Content == the summary of the activity
        let content_on_line = &schedule.timeboxes[line_idx].activity.summary;

        let char_idx = match (self.hpos as usize).checked_sub(schedule.time_col_width() + 1) {
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
    pub fn map_to_line(cursor_y: u16, render_y: u16, render_height: u16) -> Option<usize> {
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

impl MappedPos {
    pub fn new<USIZE>(char: USIZE, line: USIZE) -> MappedPos
    where
        USIZE: Into<usize>,
    {
        MappedPos {
            col: char.into(),
            line: line.into(),
        }
    }

    pub fn first() -> MappedPos {
        MappedPos { col: 0, line: 0 }
    }

    pub fn column(mut self, column: usize) -> MappedPos {
        self.col = column;
        self
    }
    pub fn next_column(mut self) -> MappedPos {
        self.col += 1;
        self
    }
    pub fn line(mut self, line: usize) -> MappedPos {
        self.line = line;
        self
    }
    pub fn next_line(mut self) -> MappedPos {
        self.line += 1;
        self
    }

    /// Maps content to cursor (0, 0) being the first character on the first item of the day at e.g. position (13, 0). Returns None on out-of-bounds.
    pub fn map_to_terminal(
        &self,
        render_y: u16,
        render_height: u16,
        schedule: &Schedule,
    ) -> Option<TerminalPos> {
        if self.line as u16 >= render_height {
            return None;
        }
        let out_y = self.line as u16 + render_y;

        let content_on_line = &schedule.timeboxes[self.line].activity.summary;
        if self.col >= content_on_line.chars().count() + 1 {
            return None;
        }

        let out_x = self.col as u16 + schedule.time_col_width() as u16 + 1;

        Some(TerminalPos::new(out_x, out_y))
    }
}

impl From<(u16, u16)> for TerminalPos {
    fn from(xy: (u16, u16)) -> Self {
        TerminalPos::new(xy.0, xy.1)
    }
}
