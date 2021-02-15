use std::io::Stdout;

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
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let KeyEvent { code, modifiers } = key;

        let pos = cursor.map_to_content(&self);

        let edit_text = &mut self.timeboxes[pos.1].activity.summary;
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
                cursor.move_right(&self, stdout)?;
                true
            }
            // Remove the character to the left of cursor, then move cursor left
            KeyCode::Backspace => {
                if char_idx != 0 {
                    let remove = char_idx - 1;
                    edit_text.remove(remove);

                    // Move cursor left
                    cursor.move_left(&self, stdout)?;

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
    pub fn insert_time_box(&mut self, idx: usize) -> Result<bool> {
        // "Insert below"
        self.timeboxes.insert(idx, TimeBox::default());

        Ok(true)
    }
}
