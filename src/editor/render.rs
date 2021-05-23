mod schedule;
mod status_bar;

use crossterm::{cursor, QueueableCommand};
use std::io::{Stdout, Write};

use super::Result;

// TODO: merge to below
pub trait Render {
    fn render(&self, stdout: &mut Stdout) -> Result<()>;
}

// TODO: move/merge to render (name: draw > render)
pub trait Draw {
    fn draw_at(&self, x: u16, y: u16, mut stdout: Stdout) -> crossterm::Result<()> {
        // Store terminal cursor position to allow moving it temporarily for draw
        let cursor_origin = cursor::position()?;

        // Move terminal cursor to draw location
        stdout.queue(cursor::MoveTo(x, y))?;

        self.draw(&mut stdout)?;

        // Move terminal cursor back to where it started
        stdout.queue(cursor::MoveTo(cursor_origin.0, cursor_origin.1))?;

        stdout.flush()?;
        Ok(())
    }

    fn draw(&self, stdout: &mut Stdout) -> crossterm::Result<()>;
}

impl<'a> Draw for &'a str {
    fn draw(&self, stdout: &mut Stdout) -> crossterm::Result<()> {
        stdout.write_all(self.as_bytes())?;
        Ok(())
    }
}
