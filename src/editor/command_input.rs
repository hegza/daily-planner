use crossterm::{
    cursor,
    event::{Event, KeyCode},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use super::{command::Command, text_capture::TextCapture};

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
                    // Enter breaks out of command input
                    if k.code == KeyCode::Enter {
                        break;
                    }

                    let cursor_move = self.cur_input.input(&k);

                    use std::cmp::Ordering;
                    match cursor_move.cmp(&0) {
                        Ordering::Greater => stdout
                            .queue(cursor::MoveRight(cursor_move as u16))
                            .unwrap()
                            .flush()
                            .unwrap(),
                        Ordering::Equal => {}
                        Ordering::Less => stdout
                            .queue(cursor::MoveLeft((-cursor_move) as u16))
                            .unwrap()
                            .flush()
                            .unwrap(),
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

fn input_into_command(_text: &str) -> Option<Command> {
    None
}
