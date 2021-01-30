use std::{
    convert::TryFrom,
    io::{Stdout, Write},
};

use crate::editor::Result;
use crossterm::{cursor, QueueableCommand};

use crate::schedule::Schedule;

#[derive(Debug)]
pub struct ContentCursor {
    // Ghost position of the cursor
    hghost: u16,
    hpos: u16,
    vpos: u16,
    schedule_y: u16,
}

impl ContentCursor {
    pub fn init(
        schedule_h: u16,
        schedule_y: u16,
        stdout: &mut Stdout,
        schedule: &Schedule,
    ) -> ContentCursor {
        let (hpos, vpos) = schedule
            .map_content_to_screen(0, 0, schedule_y, schedule_h)
            .unwrap();

        let hghost = hpos;

        // Move the cursor to the start of the schedule
        Self::move_terminal_cursor(hpos, vpos + schedule_y, stdout).unwrap();

        ContentCursor {
            schedule_y,
            hghost,
            hpos,
            vpos,
        }
    }

    pub fn redraw(&self, stdout: &mut Stdout) -> Result<()> {
        Self::move_terminal_cursor(self.hpos, self.vpos, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_down(
        &mut self,
        stdout: &mut Stdout,
        schedule: &Schedule,
        schedule_y: u16,
        schedule_h: u16,
    ) -> Result<bool> {
        self.move_cursor_mapped((0, 1), schedule, schedule_y, schedule_h, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_up(
        &mut self,
        stdout: &mut Stdout,
        schedule: &Schedule,
        schedule_y: u16,
        schedule_h: u16,
    ) -> Result<bool> {
        self.move_cursor_mapped((0, -1), schedule, schedule_y, schedule_h, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_left(
        &mut self,
        stdout: &mut Stdout,
        schedule: &Schedule,
        schedule_y: u16,
        schedule_h: u16,
    ) -> Result<bool> {
        self.move_cursor_mapped((-1, 0), schedule, schedule_y, schedule_h, stdout)
    }

    /// Returns true if cursor was moved
    pub fn move_right(
        &mut self,
        stdout: &mut Stdout,
        schedule: &Schedule,
        schedule_y: u16,
        schedule_h: u16,
    ) -> Result<bool> {
        self.move_cursor_mapped((1, 0), schedule, schedule_y, schedule_h, stdout)
    }

    fn move_cursor_mapped(
        &mut self,
        delta: (i16, i16),
        schedule: &Schedule,
        schedule_y: u16,
        schedule_h: u16,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        // Get current physical position of the cursor on the terminal screen
        let cur_pos = cursor::position()?;

        // Figure out where it lands on the schedule
        let mapped_pos =
            match schedule.map_cursor_to_content(cur_pos.0, cur_pos.1, schedule_y, schedule_h) {
                Some(pos) => pos,
                // OOB
                None => {
                    // Check if we have a valid ghost on vertical move
                    if delta.1.abs() >= 0 && delta.0 == 0 {
                        match schedule.map_cursor_to_content(
                            self.hghost,
                            cur_pos.1,
                            schedule_y,
                            schedule_h,
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

        // Restore screen position by mapping the content to screen
        let n_cur_pos =
            match schedule.map_content_to_screen(n_mapped_x, n_mapped_y, schedule_y, schedule_h) {
                Some(pos) => pos,
                None => return Ok(false),
            };

        // Save ghost on horizontal move
        if delta.0.abs() >= 1 {
            self.hghost = n_cur_pos.1;
        }

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
