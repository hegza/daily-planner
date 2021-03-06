use std::{borrow::Borrow, cell::RefCell, io::Write};

use crossterm::{
    cursor,
    style::{self, style},
    QueueableCommand,
};

use crate::editor::{
    error::ResourceOwnershipError,
    state::{StatusBar, TimeMode},
    Mode,
};

use super::Render;

impl Render for StatusBar {
    fn render(&self, stdout: &mut std::io::Stdout) -> crate::editor::Result<()> {
        let rc_mode = self
            .mode
            .upgrade()
            .ok_or_else(|| ResourceOwnershipError("'mode' has been dropped".to_string()))?;
        let cell_mode: &RefCell<Mode> = rc_mode.borrow();
        let mode: &Mode = &cell_mode.borrow();
        let mode_str = match mode {
            Mode::Cursor => "",
            Mode::Insert => "-- INSERT --",
            Mode::Time => {
                let time_mode = self.time_mode.upgrade().ok_or_else(|| {
                    ResourceOwnershipError("'time_mode' has been dropped".to_string())
                })?;
                let time_mode: &RefCell<TimeMode> = &time_mode.borrow();
                let time_mode: &TimeMode = &time_mode.borrow();
                match time_mode {
                    TimeMode::Relative => "-- ADJUST TIME (relative) --",
                    TimeMode::Absolute => "-- ADJUST TIME (absolute) --",
                }
            }
            Mode::GoTo => "goto +",
            Mode::Delete => "delete +",
        };

        let content = mode_str.to_string();

        let styled = style(&content);
        stdout
            .queue(style::PrintStyledContent(styled))?
            .queue(cursor::MoveToNextLine(1))?;
        stdout.flush()?;

        Ok(())
    }
}
