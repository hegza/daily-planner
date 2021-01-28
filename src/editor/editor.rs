use std::io::{Stdout, Write};

use super::{Error, Result};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, style, Colorize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use crate::{activity::timebox::TimeSlotKind, schedule::Schedule, Clock, Time};

#[derive(Debug)]
pub struct Editor {
    stdout: Stdout,
    pub mode: Mode,
    pub schedule: Schedule,
}

#[derive(Clone, Debug)]
pub enum Mode {
    // Move cursor, use general commands
    Cursor,
    // Write content
    Insert,
    // Adjust time
    Time,
}

impl Editor {
    pub fn with_stdout(stdout: Stdout, schedule: Schedule) -> Editor {
        Editor {
            stdout,
            schedule,
            mode: Mode::Cursor,
        }
    }

    /// Main entry point
    pub fn run(&mut self) -> Result<()> {
        self.render()?;

        // Detect keys until exit
        self.loop_input()
    }

    /// Main re-draw function
    fn render(&mut self) -> Result<()> {
        // Rename binding, we all know what stdout is
        let stdout = &mut self.stdout;

        let line_count = self.schedule.0.len();
        for (line_y, time_box) in self.schedule.0.iter().enumerate() {
            let t_str = match &time_box.time {
                Some(t) => format!("{}", t),
                None => "     ".to_owned(),
            };
            let content = format!("{:<12} {}", t_str, time_box.activity);
            let styled = style(content);
            stdout
                .queue(style::PrintStyledContent(styled))?
                .queue(cursor::MoveToNextLine(1))?;
        }
        stdout.flush()?;
        if let Some(last_timed_item) = self
            .schedule
            .0
            .iter()
            .rev()
            .find_map(|time_box| time_box.time.clone())
        {
            let first_timed_item = self
                .schedule
                .0
                .iter()
                .find_map(|time_box| time_box.time.clone())
                .unwrap();
            let first_time = match &first_timed_item {
                TimeSlotKind::Time(t) => t,
                TimeSlotKind::Span(start, _) => start,
            };
            let last_time = match &last_timed_item {
                TimeSlotKind::Time(t) => t,
                TimeSlotKind::Span(_, end) => end,
            };
            let time_left: Time = Clock::difference(last_time, first_time).into();
            stdout
                .queue(style::Print(format!(
                    "{} left unscheduled / sleep",
                    time_left
                )))?
                .queue(cursor::MoveToNextLine(1))?;
        }
        stdout
            .queue(style::Print("ctrl+q to exit"))?
            .queue(cursor::MoveToNextLine(1))?;
        stdout.flush()?;

        Ok(())
    }

    /// Main input processing loop
    fn loop_input(&mut self) -> Result<()> {
        // Rename binding, we all know what stdout is
        let stdout = &mut self.stdout;

        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: _,
                }) => {
                    stdout.queue(cursor::MoveUp(1))?.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: _,
                }) => {
                    stdout.queue(cursor::MoveDown(1))?.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: _,
                }) => {
                    stdout.queue(cursor::MoveRight(1))?.flush()?;
                }
                _ => (),
            }
        }

        Ok(())
    }
}
