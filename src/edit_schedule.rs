use std::io::Stdout;

use crossterm::event::{KeyEvent, KeyModifiers};

use crate::{
    dom::{timebox::AdjustPolicy, TimeBox},
    editor::{cursor::ContentCursor, Result},
    schedule::Schedule,
    time::Duration,
};

impl Schedule {
    /// Changes the content based on the input key. Returns true if something was changed.
    pub fn edit_content(
        &mut self,
        key: &KeyEvent,
        cursor: &mut ContentCursor,
        stdout: &mut Stdout,
    ) -> Result<bool> {
        let KeyEvent { code, modifiers } = key;

        let pos = cursor.map_to_content(&self);

        let edit_text = &mut self.timeboxes[pos.line].activity.summary;
        let char_idx = pos.col;

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
                    c
                };

                edit_text.insert(char_idx, c);
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
            // Remove the character right of cursor
            KeyCode::Delete => {
                let line_len = edit_text.len();
                if char_idx != line_len {
                    let remove = char_idx;
                    edit_text.remove(remove);

                    true
                } else {
                    false
                }
            }
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

    /// Adjusts the item at primary index, and everything after it if they're
    /// not fixed
    pub fn adjust_times_relative(
        &mut self,
        primary_index: usize,
        adjust_duration: &Duration,
        time_cursor: usize,
    ) {
        // Always adjust the primary index
        self.adjust_time_absolute(primary_index, adjust_duration, time_cursor);

        // Adjust the others only if they are not fixed
        for idx in primary_index + 1..self.timeboxes.len() {
            let timebox = self.mut_line(idx).unwrap();
            if timebox.adjust_policy != AdjustPolicy::Fixed {
                timebox.adjust_absolute(adjust_duration, true);
            }
        }
    }

    /// Adjusts the item at given index. Reorders the item if necessary
    pub fn adjust_time_absolute(
        &mut self,
        idx: usize,
        adjust_duration: &Duration,
        time_cursor: usize,
    ) {
        let timebox = &mut self.timeboxes[idx];
        timebox.adjust_absolute(&adjust_duration, time_cursor == 0);
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        let temp = self.timeboxes[first].clone();
        self.timeboxes[first] = self.timeboxes[second].clone();
        self.timeboxes[second] = temp;
    }
}
