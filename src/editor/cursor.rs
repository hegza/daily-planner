use std::{
    cell::RefCell,
    convert::TryFrom,
    io::{Stdout, Write},
    rc::Rc,
};

use crate::editor::Result;
use crossterm::{cursor, QueueableCommand};

use crate::schedule::Schedule;

#[derive(Debug)]
pub struct ContentCursor {
    // Ghost position of the cursor
    hghost: u16,
    /// The position of the cursor on terminal
    pub pos: TerminalPos,
    schedule_y: Rc<RefCell<u16>>,
    schedule_h: Rc<RefCell<u16>>,
}

/// Cursor position in absolute terminal coordinates
#[derive(Debug, Clone, Copy)]
pub struct TerminalPos {
    pub hpos: u16,
    pub vpos: u16,
}

/// Cursor position mapped to editor content
#[derive(Debug, Clone)]
pub struct MappedPos {
    pub col: usize,
    pub line: usize,
}

impl ContentCursor {
    pub fn create_at_top_left(
        schedule_y: Rc<RefCell<u16>>,
        schedule_h: Rc<RefCell<u16>>,
        stdout: &mut Stdout,
        schedule: &Schedule,
    ) -> ContentCursor {
        let pos = MappedPos::first()
            .map_to_terminal(*schedule_y.borrow(), *schedule_h.borrow(), schedule)
            .unwrap();

        Self::create_at(pos, schedule_y, schedule_h, stdout)
    }
    pub fn create_at(
        pos: TerminalPos,
        schedule_y: Rc<RefCell<u16>>,
        schedule_h: Rc<RefCell<u16>>,
        stdout: &mut Stdout,
    ) -> ContentCursor {
        let hghost = pos.hpos;

        // Move the cursor to the start of the schedule
        Self::move_terminal_cursor(pos.hpos, pos.vpos + *schedule_y.borrow(), stdout).unwrap();

        ContentCursor {
            hghost,
            pos,
            schedule_y,
            schedule_h,
        }
    }

    /// This method may panic, if called for an invalid content cursor
    pub fn map_to_content(&self, schedule: &Schedule) -> MappedPos {
        let y = *self.schedule_y.borrow();
        let h = *self.schedule_h.borrow();
        self.pos
            .map_to_content(y, h, schedule)
            .expect("failed to map cursor to content")
    }

    /// This method may panic, if called for an invalid content cursor
    pub fn map_to_line(&self) -> usize {
        let y = *self.schedule_y.borrow();
        let h = *self.schedule_h.borrow();

        TerminalPos::map_to_line(self.pos.vpos, y, h).expect("failed to map cursor to content")
    }

    pub fn redraw(&mut self, stdout: &mut Stdout) -> Result<()> {
        Self::move_terminal_cursor(self.pos.hpos, self.pos.vpos, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_down(&mut self, schedule: &Schedule, stdout: &mut Stdout) -> Result<bool> {
        self.move_cursor_mapped((0, 1), schedule, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_up(&mut self, schedule: &Schedule, stdout: &mut Stdout) -> Result<bool> {
        self.move_cursor_mapped((0, -1), schedule, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_left(&mut self, schedule: &Schedule, stdout: &mut Stdout) -> Result<bool> {
        self.move_cursor_mapped((-1, 0), schedule, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_right(&mut self, schedule: &Schedule, stdout: &mut Stdout) -> Result<bool> {
        self.move_cursor_mapped((1, 0), schedule, stdout)
    }

    fn move_cursor_mapped(
        &mut self,
        delta: (i16, i16),
        schedule: &Schedule,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        // Get current physical position of the cursor on the terminal screen
        let cur_pos: TerminalPos = cursor::position()?.into();

        // Figure out where it lands on the schedule
        let mapped_pos = match cur_pos.map_to_content(
            *self.schedule_y.borrow(),
            *self.schedule_h.borrow(),
            schedule,
        ) {
            Some(pos) => pos,
            // OOB
            None => {
                // Check if we have a valid ghost on vertical move
                if delta.1.abs() >= 0 && delta.0 == 0 {
                    match TerminalPos::new(self.hghost, cur_pos.vpos).map_to_content(
                        *self.schedule_y.borrow(),
                        *self.schedule_h.borrow(),
                        schedule,
                    ) {
                        Some(pos) => pos,
                        None => return Ok(false),
                    }
                } else {
                    return Ok(false);
                }
            }
        };

        // Move the cursor using the delta
        let n_mapped_x = if let Ok(x) = u16::try_from(mapped_pos.col as i16 + delta.0) {
            x
        } else {
            return Ok(false);
        };
        let n_mapped_y = if let Ok(y) = u16::try_from(mapped_pos.line as i16 + delta.1) {
            y
        } else {
            return Ok(false);
        };

        self.move_to_content(&MappedPos::new(n_mapped_x, n_mapped_y), schedule, stdout)
    }

    /// Move to content column
    pub fn move_to_column(
        &mut self,
        idx: usize,
        schedule: &Schedule,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let cursor_line = self.map_to_line();

        self.move_to_content(&MappedPos::new(idx, cursor_line), schedule, stdout)
    }

    pub fn move_to_content(
        &mut self,
        mapped_pos: &MappedPos,
        schedule: &Schedule,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        // Restore screen position by mapping the content to screen
        let n_cur_pos = match mapped_pos.map_to_terminal(
            *self.schedule_y.borrow(),
            *self.schedule_h.borrow(),
            schedule,
        ) {
            Some(pos) => pos,
            None => return Ok(false),
        };
        self.pos = n_cur_pos;

        // Fail violently if the final move fails
        Self::move_terminal_cursor(n_cur_pos.hpos, n_cur_pos.vpos, stdout).expect("cursor desync");

        Ok(true)
    }

    pub fn clamp_to_content(&mut self, schedule: &Schedule) {
        let y_min = *self.schedule_y.borrow();
        if self.pos.vpos < y_min {
            self.pos.vpos = y_min;
        }

        let y_max = y_min + (*self.schedule_h.borrow()).min(schedule.timeboxes.len() as u16 - 1);
        if self.pos.vpos > y_max {
            self.pos.vpos = y_max;
        }

        let x_min = schedule.time_col_width() as u16 + 1;
        if self.pos.hpos < x_min {
            self.pos.hpos = x_min;
        }

        let content_line = self.map_to_line();
        let x_max = x_min + schedule.timeboxes[content_line].activity.summary.len() as u16;
        if self.pos.hpos > x_max {
            self.pos.hpos = x_max;
        }
    }

    /// Makes the API call to crossterm, moving the cursor to target terminal position.
    fn move_terminal_cursor(screen_h: u16, screen_v: u16, stdout: &mut Stdout) -> Result<()> {
        Ok(stdout.queue(cursor::MoveTo(screen_h, screen_v))?.flush()?)
    }
}
