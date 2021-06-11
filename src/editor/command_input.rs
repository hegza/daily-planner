use crossterm::{
    cursor,
    event::{Event, KeyCode},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use super::{
    command::Command,
    text_capture::{self, TextCapture},
};

/// A modal command input that captures stdin and cursor while it's active.
#[derive(Debug)]
pub struct CommandInput {
    cur_input: TextCapture,
}

impl Default for CommandInput {
    fn default() -> Self {
        CommandInput {
            cur_input: TextCapture::owned(text_capture::InputSource::Stdio(None)),
        }
    }
}

impl CommandInput {
    /// Captures stdout and blocks while updating the CommandInput contents based on input.
    pub fn capture(&mut self, stdout: &mut Stdout) -> crossterm::Result<Option<Command>> {
        loop {
            let ev = crossterm::event::read()?;
            let redraw = match ev {
                Event::Key(k) => {
                    // Enter breaks out of command input
                    if k.code == KeyCode::Enter {
                        break;
                    }

                    let (redraw, cursor_move) = self.cur_input.input(&k);

                    use std::cmp::Ordering;
                    match cursor_move.cmp(&0) {
                        Ordering::Greater => stdout
                            .queue(cursor::MoveRight(cursor_move as u16))?
                            .flush()?,
                        Ordering::Equal => {}
                        Ordering::Less => stdout
                            .queue(cursor::MoveLeft((-cursor_move) as u16))?
                            .flush()?,
                    }

                    redraw
                }
                Event::Mouse(_) => false,
                Event::Resize(_, _) => false,
            };

            if redraw {}
        }

        Ok(input_into_command(&self.cur_input.text()))
    }
}

fn input_into_command(_text: &str) -> Option<Command> {
    None
}
