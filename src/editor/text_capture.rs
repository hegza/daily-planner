use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    cell::{Ref, RefCell},
    fmt,
    io::{self, Stdout, Write},
    rc::Rc,
};

use super::render::Draw;

/// Captures and maintains text input and a character cursor from key events passed to it. Can maintain it's own buffer or borrow an external one.
#[derive(Debug)]
pub struct TextCapture {
    text: Rc<RefCell<String>>,
    cursor: usize,
    input: InputSource,
}

#[derive(Debug)]
pub enum InputSource {
    // Captured stdio
    Stdio(Option<(io::Stdin, io::Stdout)>),
    // Returns (needs_redraw, cursor_movement{-1, 0, 1})
    Key(InputFn),
}

pub struct InputFn(Box<dyn Fn(&KeyEvent) -> (bool, i32)>);

impl fmt::Debug for InputFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("InputFn(KeyEvent) -> (bool, i32)")
    }
}

impl TextCapture {
    /// Creates an empty text capture with its own buffer.
    pub fn owned(input: InputSource) -> TextCapture {
        TextCapture {
            text: Rc::new(RefCell::new(String::new())),
            cursor: 0,
            input,
        }
    }
    /// Creates a text capture into a reference. Subsequent calls to TextCapture change the given text content.
    pub fn capture(
        text_ptr: Rc<RefCell<String>>,
        char_cursor: usize,
        input: InputSource,
    ) -> TextCapture {
        TextCapture {
            text: text_ptr,
            cursor: char_cursor,
            input,
        }
    }

    /// Changes the captured text based on key event. Returns redraw and resultant cursor movement, positive = right, negative = left.
    pub fn input(&mut self, key: &KeyEvent) -> (bool, i32) {
        let KeyEvent { code, modifiers } = key;

        match code {
            // Insert char and move cursor right
            KeyCode::Char(c) => {
                let mut c = *c;

                // HACK: work around hard to manage multi-codepoint unicode characters
                if c.len_utf8() != 1 {
                    c = match c {
                        'ä' => 'a',
                        'ö' => 'o',
                        // Return for a no-op on unknown multi-codepoint character
                        _ => {
                            return (false, 0);
                        }
                    };
                }

                // Make into capital letter if shift is pressed
                if modifiers.intersects(KeyModifiers::SHIFT) {
                    c = c.to_uppercase().next().expect("programmer logic error");
                };

                // Insert the character into captured text at cursor position
                self.text.borrow_mut().insert(self.cursor, c);
                // Return cursor movement: +1
                (true, 1)
            }
            // Remove the character to the left of cursor, then move cursor left
            KeyCode::Backspace => {
                if self.cursor_left() {
                    let remove = self.cursor;
                    self.text.borrow_mut().remove(remove);
                    (true, -1)
                } else {
                    (true, 0)
                }
            }
            KeyCode::Enter => (false, 0),
            // Move cursor to start of content
            KeyCode::Home => {
                let left = self.cursor;
                self.cursor -= left;
                (true, -(left as i32))
            }
            // Move cursor to end of content
            KeyCode::End => {
                let right = self
                    .text
                    .borrow()
                    .len()
                    .checked_sub(self.cursor)
                    .expect("off-by-one for end-key");
                self.cursor += right;

                (true, right as i32)
            }
            KeyCode::PageUp => (false, 0),
            KeyCode::PageDown => (false, 0),
            // Move 4 or less to the right
            KeyCode::Tab => {
                let right = 0.max(self.text.borrow().len() - self.cursor + 4);
                self.cursor += right;
                (true, right as i32)
            }
            // Move 4 or less to the left
            KeyCode::BackTab => {
                let left = 0.min(self.cursor as i32 - 4);
                self.cursor -= left as usize;
                (true, left)
            }
            // Remove the character right of cursor
            KeyCode::Delete => {
                let line_len = self.text.borrow().len();
                if self.cursor != line_len {
                    let remove = self.cursor;
                    self.text.borrow_mut().remove(remove);
                }
                (false, 0)
            }
            KeyCode::Insert => (false, 0),
            KeyCode::F(_) => (false, 0),
            KeyCode::Left => {
                if self.cursor_left() {
                    (true, -1)
                } else {
                    (false, 0)
                }
            }
            KeyCode::Right => {
                if self.cursor_right() {
                    (true, 1)
                } else {
                    (false, 0)
                }
            }
            KeyCode::Up => (false, 0),
            KeyCode::Down => (false, 0),
            KeyCode::Null => (false, 0),
            KeyCode::Esc => (false, 0),
        }
    }

    fn cursor_left(&mut self) -> bool {
        // If no characters left of cursor, return
        if self.cursor == 0 {
            false
        } else {
            // Move cursor left
            self.cursor -= 1;
            true
        }
    }

    fn cursor_right(&mut self) -> bool {
        if self.cursor >= self.text.borrow().len() {
            false
        } else {
            // Move cursor right
            self.cursor += 1;
            true
        }
    }
    // Resets the whole text
    pub fn set_text(&mut self, text: String) {
        *self.text.borrow_mut() = text;
    }
    pub fn text(&self) -> Ref<String> {
        self.text.borrow()
    }
    pub fn cursor(&self) -> u16 {
        self.cursor as u16
    }
}

impl Draw for TextCapture {
    fn draw(&self, stdout: &mut Stdout) -> crossterm::Result<()> {
        stdout.write_all(self.text().as_bytes())?;

        stdout.flush()?;
        Ok(())
    }
}
