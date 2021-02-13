use std::{
    cell::RefCell,
    io::{Stdout, Write},
    rc::{Rc, Weak},
};

use super::{
    command::{self, Command},
    cursor::ContentCursor,
    render::Render,
    Result,
};
use crossterm::{
    cursor,
    event::{read, Event, KeyEvent},
    style, terminal, ExecutableCommand, QueueableCommand,
};

use crate::{activity::timebox::TimeSlotKind, schedule::Schedule, Clock, Time};

#[derive(Debug)]
pub struct Editor {
    stdout: Stdout,
    cursor: Option<ContentCursor>,
    /// The y-position of the cursor was when the schedule started to render
    schedule_y: Option<u16>,
    /// The height of the schedule when rendered, based on cursor y-position when the schedule stopped rendering
    schedule_h: Option<u16>,
    pub mode: Rc<RefCell<Mode>>,
    pub schedule: Schedule,
    status_bar: StatusBar,
    quit: bool,
}

#[derive(Clone, Debug, PartialEq)]
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
        let mode = Rc::new(RefCell::new(Mode::Cursor));

        Editor {
            stdout,
            schedule,
            schedule_y: None,
            schedule_h: None,
            cursor: None,
            status_bar: StatusBar {
                mode: Rc::downgrade(&mode),
            },
            mode,
            quit: false,
        }
    }

    /// Main entry point
    pub fn run(&mut self) -> Result<()> {
        self.render()?;

        // Create cursor at top-left
        self.cursor = Some(ContentCursor::init(
            self.schedule_h
                .expect("schedule position on screen must be known"),
            self.schedule_y
                .expect("schedule position on screen must be known"),
            &mut self.stdout,
            &self.schedule,
        ));

        // Detect keys until exit
        self.loop_input()
    }

    /// Main re-draw function
    fn render(&mut self) -> Result<()> {
        // Rename binding, we all know what stdout is
        let stdout = &mut self.stdout;

        // Clear screen and move cursor to top-left
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        // Render schedule while measuring it's height
        self.schedule_y = Some(cursor::position()?.1);
        self.schedule.render(stdout)?;
        self.schedule_h = Some(cursor::position()?.1 - self.schedule_y.unwrap());

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
            let time_left = first_time - last_time;
            stdout
                .queue(style::Print(format!(
                    "{} left unscheduled / sleep, wake-up at {}",
                    time_left,
                    last_time + &time_left
                )))?
                .queue(cursor::MoveToNextLine(1))?;
        }

        stdout
            .queue(style::Print("ctrl+q to exit"))?
            .queue(cursor::MoveToNextLine(1))?;

        self.status_bar.render(stdout)?;

        stdout.flush()?;

        if let Some(cursor) = self.cursor.as_ref() {
            cursor.redraw(stdout)?;
        }

        Ok(())
    }

    /// Main input processing loop
    fn loop_input(&mut self) -> Result<()> {
        loop {
            let ev = read()?;
            let redraw = match ev {
                Event::Key(key_ev) => {
                    let editor_command = Command::map(key_ev.clone(), self);

                    let redraw = if let Some(cmd) = editor_command {
                        let redraw = self.act(&cmd)?;
                        if self.quit {
                            break;
                        }
                        redraw
                    }
                    // No command was found for this key
                    else {
                        // Insert mode: make edits to the schedule data-structure
                        if *self.mode.borrow() == Mode::Insert {
                            let redraw = self.schedule.edit_content(
                                &key_ev,
                                self.cursor
                                    .as_mut()
                                    .expect("must have cursor when editing schedule"),
                                self.schedule_y.expect("schedule must have been rendered"),
                                self.schedule_h.expect("schedule must have been rendered"),
                                &mut self.stdout,
                            )?;
                            redraw
                        } else {
                            // redraw
                            false
                        }
                    };

                    redraw
                }
                Event::Mouse(_) => {
                    // redraw
                    false
                }
                Event::Resize(_, _) => {
                    eprintln!("resize not implemented");
                    // redraw
                    false
                }
            };
            if redraw {
                self.render()?;
            }
        }

        Ok(())
    }

    /// Returns "need full redraw"
    fn act(&mut self, cmd: &Command) -> Result<bool> {
        let redraw = match cmd {
            Command::Quit => {
                self.quit = true;
                false
            }
            Command::MoveCursor(dir) => {
                let cursor = self.cursor.as_mut().unwrap();
                match dir {
                    super::command::Dir::Up => {
                        cursor.move_up(
                            &mut self.stdout,
                            &self.schedule,
                            self.schedule_y.unwrap(),
                            self.schedule_h.unwrap(),
                        )?;
                    }
                    super::command::Dir::Down => {
                        cursor.move_down(
                            &mut self.stdout,
                            &self.schedule,
                            self.schedule_y.unwrap(),
                            self.schedule_h.unwrap(),
                        )?;
                    }
                    super::command::Dir::Left => {
                        cursor.move_left(
                            &mut self.stdout,
                            &self.schedule,
                            self.schedule_y.unwrap(),
                            self.schedule_h.unwrap(),
                        )?;
                    }
                    super::command::Dir::Right => {
                        cursor.move_right(
                            &mut self.stdout,
                            &self.schedule,
                            self.schedule_y.unwrap(),
                            self.schedule_h.unwrap(),
                        )?;
                    }
                };
                // Redraw
                false
            }
            Command::InsertMode => {
                *self.mode.borrow_mut() = Mode::Insert;
                // Redraw
                true
            }
            Command::CursorMode => {
                *self.mode.borrow_mut() = Mode::Cursor;
                // Redraw
                true
            }
            Command::TimeMode => {
                *self.mode.borrow_mut() = Mode::Time;
                // Redraw
                true
            }
            Command::InsertTimeBoxBelow => {
                // Insert time box below
                self.schedule.insert_time_box_below(
                    self.cursor
                        .as_mut()
                        .expect("must have cursor when editing schedule"),
                    self.schedule_y.expect("schedule must have been rendered"),
                    self.schedule_h.expect("schedule must have been rendered"),
                    &mut self.stdout,
                )?;

                // Redraw
                true
            }
            Command::InsertTimeBoxAbove => {
                // Insert time box below
                self.schedule.insert_time_box_above(
                    self.cursor
                        .as_mut()
                        .expect("must have cursor when editing schedule"),
                    self.schedule_y.expect("schedule must have been rendered"),
                    self.schedule_h.expect("schedule must have been rendered"),
                    &mut self.stdout,
                )?;

                // Redraw
                true
            }
            Command::Multi(commands) => {
                let mut redraw = false;
                for cmd in commands.iter() {
                    if self.act(cmd)? {
                        redraw = true;
                    }
                }
                redraw
            }
            Command::GoToColumn(col_kind) => {
                let cursor = self.cursor.as_mut().unwrap();
                let sched_y = self.schedule_y.unwrap();
                let sched_h = self.schedule_h.unwrap();
                let cursor_pos = cursor.map_to_content(&self.schedule, sched_y, sched_h);
                match col_kind {
                    command::ColumnKind::Index(idx) => cursor.move_to_content(
                        *idx as u16,
                        cursor_pos.1 as u16,
                        &mut self.stdout,
                        &self.schedule,
                        sched_y,
                        sched_h,
                    )?,
                    command::ColumnKind::Last => cursor.move_to_content(
                        self.schedule.0[cursor_pos.1 as usize]
                            .activity
                            .summary
                            .len() as u16,
                        cursor_pos.1 as u16,
                        &mut self.stdout,
                        &self.schedule,
                        sched_y,
                        sched_h,
                    )?,
                }
            }
        };
        Ok(redraw)
    }
}

#[derive(Debug)]
pub struct StatusBar {
    pub mode: Weak<RefCell<Mode>>,
}
