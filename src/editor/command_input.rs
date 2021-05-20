use crossterm::{cursor, event::Event, QueueableCommand};
use std::{
    cell::RefCell,
    io::{Stdout, Write},
    rc::Rc,
};

use super::{command::Command, cursor::ContentCursor, text_capture::TextCapture};

/// A modal command input that captures stdin and cursor while it's active.
#[derive(Debug)]
pub struct CommandInput {
    cur_input: TextCapture,
}

impl Default for CommandInput {
    fn default() -> Self {
        CommandInput {
            cur_input: TextCapture::owned(),
        }
    }
}

impl CommandInput {
    pub fn capture(&mut self, stdout: &mut Stdout) -> Option<Command> {
        loop {
            let ev = crossterm::event::read().unwrap();
            let redraw = match ev {
                Event::Key(k) => {
                    let cursor_move = self.cur_input.input(&k);

                    if cursor_move > 0 {
                        stdout
                            .queue(cursor::MoveRight(cursor_move as u16))
                            .unwrap()
                            .flush()
                            .unwrap()
                    } else if cursor_move < 0 {
                        stdout
                            .queue(cursor::MoveLeft((-cursor_move) as u16))
                            .unwrap()
                            .flush()
                            .unwrap()
                    }
                    true
                }
                Event::Mouse(_) => false,
                Event::Resize(_, _) => false,
            };

            if redraw {}
        }

        input_into_command(&self.cur_input.text())
    }
}

fn input_into_command(text: &str) -> Option<Command> {
    None
}
