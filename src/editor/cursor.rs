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
    /// The horizontal position of the cursor on terminal
    hpos: u16,
    /// The vertical position of the cursor on terminal
    vpos: u16,
    schedule_y: Rc<RefCell<u16>>,
    schedule_h: Rc<RefCell<u16>>,
}

impl ContentCursor {
    pub fn init(
        schedule_y: Rc<RefCell<u16>>,
        schedule_h: Rc<RefCell<u16>>,
        stdout: &mut Stdout,
        schedule: &Schedule,
    ) -> ContentCursor {
        let (hpos, vpos) = schedule
            .map_content_to_screen(0, 0, *schedule_y.borrow(), *schedule_h.borrow())
            .unwrap();

        let hghost = hpos;

        // Move the cursor to the start of the schedule
        Self::move_terminal_cursor(hpos, vpos + *schedule_y.borrow(), stdout).unwrap();

        ContentCursor {
            hghost,
            hpos,
            vpos,
            schedule_y,
            schedule_h,
        }
    }

    /// This method may panic, if called for an invalid content cursor
    pub fn map_to_content(&self, schedule: &Schedule) -> (usize, usize) {
        let y = *self.schedule_y.borrow();
        let h = *self.schedule_h.borrow();
        let (x, y) = schedule
            .map_cursor_to_content(self.hpos, self.vpos, y, h)
            .expect("failed to map cursor to content");
        (x as usize, y as usize)
    }

    /// This method may panic, if called for an invalid content cursor
    pub fn map_to_line(&self, schedule: &Schedule) -> usize {
        let y = *self.schedule_y.borrow();
        let h = *self.schedule_h.borrow();
        let y = schedule
            .map_cursor_to_line(self.vpos, y, h)
            .expect("failed to map cursor to content");
        y as usize
    }

    pub fn redraw(&mut self, stdout: &mut Stdout) -> Result<()> {
        Self::move_terminal_cursor(self.hpos, self.vpos, stdout)
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
        let cur_pos = cursor::position()?;

        // Figure out where it lands on the schedule
        let mapped_pos = match schedule.map_cursor_to_content(
            cur_pos.0,
            cur_pos.1,
            *self.schedule_y.borrow(),
            *self.schedule_h.borrow(),
        ) {
            Some(pos) => pos,
            // OOB
            None => {
                // Check if we have a valid ghost on vertical move
                if delta.1.abs() >= 0 && delta.0 == 0 {
                    match schedule.map_cursor_to_content(
                        self.hghost,
                        cur_pos.1,
                        *self.schedule_y.borrow(),
                        *self.schedule_h.borrow(),
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
        let n_mapped_x = if let Ok(x) = u16::try_from(mapped_pos.0 as i16 + delta.0) {
            x
        } else {
            return Ok(false);
        };
        let n_mapped_y = if let Ok(y) = u16::try_from(mapped_pos.1 as i16 + delta.1) {
            y
        } else {
            return Ok(false);
        };

        self.move_to_content(n_mapped_x, n_mapped_y, schedule, stdout)
    }

    /// Move to content column
    pub fn move_to_column(
        &mut self,
        idx: usize,
        schedule: &Schedule,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let cursor_line = self.map_to_line(schedule);

        self.move_to_content(idx as u16, cursor_line as u16, schedule, stdout)
    }

    pub fn move_to_content(
        &mut self,
        content_x: u16,
        content_y: u16,
        schedule: &Schedule,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        // Restore screen position by mapping the content to screen
        let n_cur_pos = match schedule.map_content_to_screen(
            content_x,
            content_y,
            *self.schedule_y.borrow(),
            *self.schedule_h.borrow(),
        ) {
            Some(pos) => pos,
            None => return Ok(false),
        };

        self.hpos = n_cur_pos.0;
        self.vpos = n_cur_pos.1;

        // Fail violently if the final move fails
        Self::move_terminal_cursor(n_cur_pos.0, n_cur_pos.1, stdout).expect("cursor desync");

        Ok(true)
    }

    fn move_terminal_cursor(screen_h: u16, screen_v: u16, stdout: &mut Stdout) -> Result<()> {
        Ok(stdout.queue(cursor::MoveTo(screen_h, screen_v))?.flush()?)
    }
}
