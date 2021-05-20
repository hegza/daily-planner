use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    cell::{Ref, RefCell},
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// Captures and maintains textual input and a character cursor from key events passed to it. Can maintain it's own buffer or borrow an external one.
#[derive(Debug)]
pub struct TextCapture {
    text: Rc<RefCell<String>>,
    cursor: usize,
}

impl TextCapture {
    /// Creates an empty text capture with its own buffer.
    pub fn owned() -> TextCapture {
        TextCapture {
            text: Rc::new(RefCell::new(String::new())),
            cursor: 0,
        }
    }
    /// Creates a text capture into a reference. Subsequent calls to TextCapture change the given text content.
    pub fn capture(text_ptr: &Rc<RefCell<String>>, char_cursor: usize) -> TextCapture {
        TextCapture {
            text: text_ptr.clone(),
            cursor: char_cursor,
        }
    }

    /// Changes the captured text based on key event. Returns resultant cursor movement, positive = right, negative = left.
    pub fn input(&mut self, key: &KeyEvent) -> i32 {
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
                            return 0;
                        }
                    };
                }

                // Make into capital letter if shift is pressed
                if modifiers.intersects(KeyModifiers::SHIFT) {
                    c = c.to_uppercase().next().unwrap();
                };

                // Insert the character into captured text at cursor position
                self.text.borrow_mut().insert(self.cursor, c.into());
                // Return cursor movement: +1
                1
            }
            // Remove the character to the left of cursor, then move cursor left
            KeyCode::Backspace => {
                if self.cursor_left() {
                    let remove = self.cursor;
                    self.text.borrow_mut().remove(remove);
                    -1
                } else {
                    0
                }
            }
            KeyCode::Enter => 0,
            // Move cursor to start of content
            KeyCode::Home => {
                let left = self.cursor;
                self.cursor -= left;
                -(left as i32)
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

                right as i32
            }
            KeyCode::PageUp => 0,
            KeyCode::PageDown => 0,
            // Move 4 or less to the right
            KeyCode::Tab => {
                let right = 0.max(self.text.borrow().len() - self.cursor + 4);
                self.cursor += right;
                right as i32
            }
            // Move 4 or less to the left
            KeyCode::BackTab => {
                let left = 0.min(self.cursor as i32 - 4);
                self.cursor -= left as usize;
                left
            }
            // Remove the character right of cursor
            KeyCode::Delete => {
                let line_len = self.text.borrow().len();
                if self.cursor != line_len {
                    let remove = self.cursor;
                    self.text.borrow_mut().remove(remove);
                }
                0
            }
            KeyCode::Insert => 0,
            KeyCode::F(_) => 0,
            KeyCode::Left => {
                if self.cursor_left() {
                    -1
                } else {
                    0
                }
            }
            KeyCode::Right => {
                if self.cursor_right() {
                    1
                } else {
                    0
                }
            }
            KeyCode::Up => 0,
            KeyCode::Down => 0,
            KeyCode::Null => 0,
            KeyCode::Esc => 0,
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

    pub fn text(&self) -> Ref<String> {
        self.text.borrow()
    }
}
