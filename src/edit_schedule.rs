use std::{char::decode_utf16, io::Stdout, iter};

use crossterm::event::{KeyEvent, KeyModifiers};

use crate::{
    activity::TimeBox,
    editor::{cursor::ContentCursor, Result},
    schedule::Schedule,
};

impl Schedule {
    /// Returns true if something was changed
    pub fn edit_content(
        &mut self,
        key: &KeyEvent,
        cursor: &mut ContentCursor,
        self_y: u16,
        self_h: u16,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let KeyEvent { code, modifiers } = key;

        let pos = cursor.map_to_content(self, self_y, self_h);

        let edit_text = &mut self.0[pos.1].activity.summary;
        let char_idx = pos.0;

        use crossterm::event::KeyCode;
        let redraw = match code {
            // Insert char and move cursor right
            KeyCode::Char(c) => {
                // HACK: work around hard to manage multi-codepoint unicode characters
                let c = if c.len_utf8() != 1 {
                    match c {
                        'ä' => 'a',
                        'ö' => 'o',
                        // Return on unknown multi-codepoint character
                        _ => return Ok(false),
                    }
                } else {
                    *c
                };

                let c = if modifiers.intersects(KeyModifiers::SHIFT) {
                    c.to_uppercase().next().unwrap()
                } else {
                    c.clone()
                };

                edit_text.insert(char_idx, c.into());
                // Alternative implementation for insert
                /*
                let (start, end) = edit_text.split_at(char_idx);
                let n_text = format!("{}{}{}", start, c, end);
                *edit_text = n_text;
                */

                cursor.move_right(stdout, self, self_y, self_h)?;
                true
            }
            // Remove the character to the left of cursor, then move cursor left
            KeyCode::Backspace => {
                if char_idx != 0 {
                    let remove = char_idx - 1;
                    edit_text.remove(remove);

                    // Move cursor left
                    cursor.move_left(stdout, self, self_y, self_h)?;

                    true
                } else {
                    false
                }
            }
            KeyCode::Enter => false,
            KeyCode::Home => false,
            KeyCode::End => false,
            KeyCode::PageUp => false,
            KeyCode::PageDown => false,
            KeyCode::Tab => false,
            KeyCode::BackTab => false,
            KeyCode::Delete => false,
            KeyCode::Insert => false,
            KeyCode::F(_) => false,
            _ => false,
        };

        Ok(redraw)
    }

    /// Returns true if something was changed
    pub fn insert_time_box_below(
        &mut self,
        cursor: &mut ContentCursor,
        self_y: u16,
        self_h: u16,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let pos = cursor.map_to_content(self, self_y, self_h);
        let cursor_line = pos.1;

        // "Insert below"
        self.0.insert(cursor_line + 1, TimeBox::default());

        // Move one down and to the beginning
        cursor.move_to_content(0, pos.1 as u16 + 1, stdout, self, self_y, self_h)?;

        Ok(true)
    }

    /// Returns true if something was changed
    pub fn insert_time_box_above(
        &mut self,
        cursor: &mut ContentCursor,
        self_y: u16,
        self_h: u16,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let pos = cursor.map_to_content(self, self_y, self_h);
        let cursor_line = pos.1;

        // "Insert below"
        self.0.insert(cursor_line, TimeBox::default());

        // Move one up and to the beginning
        cursor.move_to_content(0, pos.1 as u16, stdout, self, self_y, self_h)?;

        Ok(true)
    }
}