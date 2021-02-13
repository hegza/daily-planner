use std::{borrow::Borrow, cell::RefCell, io::Write};

use crossterm::{
    cursor,
    style::{self, style},
    QueueableCommand,
};

use crate::editor::editor::{Mode, StatusBar};

use super::Render;

impl Render for StatusBar {
    fn render(&self, stdout: &mut std::io::Stdout) -> crate::editor::Result<()> {
        let rc_mode = self.mode.upgrade().unwrap();
        let cell_mode: &RefCell<Mode> = rc_mode.borrow();
        let mode: &Mode = &cell_mode.borrow();
        let mode_str = match mode {
            Mode::Cursor => "",
            Mode::Insert => "-- INSERT --",
            Mode::Time => "-- ADJUST TIME --",
            Mode::GoTo => "goto +",
            Mode::Delete => "delete +",
        };

        let content = format!("{}", mode_str);

        let styled = style(&content);
        stdout
            .queue(style::PrintStyledContent(styled))?
            .queue(cursor::MoveToNextLine(1))?;
        stdout.flush()?;

        Ok(())
    }
}
