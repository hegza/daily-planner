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
    event::{read, Event},
    style, terminal, ExecutableCommand, QueueableCommand,
};

use crate::{
    activity::{timebox::TimeSlotKind, TimeBox},
    schedule::Schedule,
};

#[derive(Debug)]
pub struct Editor {
    stdout: Rc<RefCell<Stdout>>,
    cursor: Option<ContentCursor>,
    /// The y-position of the cursor was when the schedule started to render
    schedule_y: Option<Rc<RefCell<u16>>>,
    /// The height of the schedule when rendered, based on cursor y-position when the schedule stopped rendering
    schedule_h: Option<Rc<RefCell<u16>>>,
    pub mode: Rc<RefCell<Mode>>,
    parent_mode: Mode,
    pub schedule: Rc<RefCell<Schedule>>,
    status_bar: StatusBar,
    clipboard: Option<TimeBox>,
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
    // Go to something (transient)
    GoTo,
    // Delete something (transient)
    Delete,
}

impl Mode {
    fn is_transient(&self) -> bool {
        match self {
            Mode::Cursor => false,
            Mode::Insert => false,
            Mode::Time => false,
            Mode::GoTo => true,
            Mode::Delete => true,
        }
    }
}

impl Editor {
    pub fn with_stdout(stdout: Stdout, schedule: Schedule) -> Editor {
        let stdout = Rc::new(RefCell::new(stdout));
        let schedule = Rc::new(RefCell::new(schedule));
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
            parent_mode: Mode::Cursor,
            clipboard: None,
            quit: false,
        }
    }

    /// Main entry point
    pub fn run(&mut self) -> Result<()> {
        self.render()?;

        // Create cursor at top-left
        let y = self
            .schedule_y
            .as_ref()
            .expect("schedule position on screen must be known")
            .clone();
        self.cursor = Some(ContentCursor::init(
            y,
            self.schedule_h
                .as_ref()
                .expect("schedule position on screen must be known")
                .clone(),
            self.stdout.clone(),
            self.schedule.clone(),
        ));

        // Detect keys until exit
        self.loop_input()
    }

    /// Main re-draw function
    fn render(&mut self) -> Result<()> {
        {
            // Rename binding, we all know what stdout is
            let stdout = &mut (*self.stdout).borrow_mut();

            // Clear screen and move cursor to top-left
            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            stdout.queue(cursor::MoveTo(0, 0)).unwrap();

            // Render schedule while measuring it's height
            let y = cursor::position()?.1;
            self.schedule_y.replace(Rc::new(RefCell::new(y)));
            (*self.schedule).borrow_mut().render(stdout)?;
            self.schedule_h
                .replace(Rc::new(RefCell::new(cursor::position()?.1 - y)));

            if let Some(last_timed_item) = self
                .schedule
                .borrow()
                .0
                .iter()
                .rev()
                .find_map(|time_box| time_box.time.clone())
            {
                let first_timed_item = self
                    .schedule
                    .borrow()
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
        }

        if let Some(cursor) = self.cursor.as_ref() {
            cursor.redraw()?;
        }

        Ok(())
    }

    /// Main input processing loop
    fn loop_input(&mut self) -> Result<()> {
        loop {
            let ev = read()?;
            let redraw = match ev {
                Event::Key(key_ev) => {
                    // Determine command
                    let editor_command = Command::map(key_ev.clone(), self);
                    let mut redraw = false;

                    // Return to parent mode on command for transient modes like 'g' and 'd'
                    if self.mode.borrow().is_transient() {
                        self.mode.replace(self.parent_mode.clone());
                        redraw = true;
                    }

                    redraw |= if let Some(cmd) = editor_command {
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
                            let schedule = &mut self.schedule.borrow_mut();
                            let redraw = schedule.edit_content(
                                &key_ev,
                                self.cursor
                                    .as_mut()
                                    .expect("must have cursor when editing schedule"),
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
                    command::MoveCursor::Dir(dir) => match dir {
                        command::Dir::Up => {
                            cursor.move_up()?;
                        }
                        command::Dir::Down => {
                            cursor.move_down()?;
                        }
                        command::Dir::Left => {
                            cursor.move_left()?;
                        }
                        command::Dir::Right => {
                            cursor.move_right()?;
                        }
                    },
                    command::MoveCursor::Top => unimplemented!(),
                    command::MoveCursor::Bottom => unimplemented!(),
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
            Command::GoToMode => {
                // Store current mode as parent mode
                self.parent_mode = self.mode.borrow().clone();

                *self.mode.borrow_mut() = Mode::GoTo;

                // Redraw
                true
            }
            Command::InsertTimeBoxBelow => {
                // Insert time box below
                let cursor = self
                    .cursor
                    .as_ref()
                    .expect("must have cursor when editing schedule");

                let pos = cursor.map_to_content();
                let cursor_line = pos.1 + 1;

                self.schedule.borrow_mut().insert_time_box(cursor_line)?;

                // Move one down and to the beginning
                let pos = cursor.map_to_content();
                self.cursor
                    .as_mut()
                    .expect("must have cursor")
                    .move_to_content(0, pos.1 as u16 + 1)?;

                // Redraw
                true
            }
            Command::InsertTimeBoxAbove => {
                // Insert time box below
                let cursor = self
                    .cursor
                    .as_ref()
                    .expect("must have cursor when editing schedule");

                let pos = cursor.map_to_content();
                let cursor_line = pos.1;

                self.schedule.borrow_mut().insert_time_box(cursor_line)?;

                // Move one up and to the beginning
                let pos = cursor.map_to_content();
                self.cursor
                    .as_mut()
                    .expect("must have cursor")
                    .move_to_content(0, pos.1 as u16)?;

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
                let cursor_pos = cursor.map_to_content();

                match col_kind {
                    command::ColumnKind::Index(idx) => {
                        cursor.move_to_content(*idx as u16, cursor_pos.1 as u16)?
                    }
                    command::ColumnKind::Last => {
                        let x = self.schedule.borrow_mut().0[cursor_pos.1 as usize]
                            .activity
                            .summary
                            .len() as u16;
                        cursor.move_to_content(x, cursor_pos.1 as u16)?
                    }
                }
            }
            Command::DeleteMode => {
                *self.mode.borrow_mut() = Mode::Delete;
                true
            }
            Command::CutCurrentLine => {
                let cursor = self.cursor.as_mut().unwrap();
                let cursor_pos = cursor.map_to_content();
                let removed = self.schedule.borrow_mut().0.remove(cursor_pos.1 as usize);

                self.clipboard = Some(removed);

                true
            }
            Command::PasteBelow => {
                if let Some(content) = self.clipboard.as_ref() {
                    let cursor = self.cursor.as_mut().unwrap();
                    let cursor_pos = cursor.map_to_content();

                    let sched: &mut Schedule = &mut self.schedule.borrow_mut();
                    sched.0.insert(cursor_pos.1 as usize + 1, content.clone());
                    true
                } else {
                    false
                }
            }
            Command::PasteAbove => {
                if let Some(content) = self.clipboard.as_ref() {
                    let cursor = self.cursor.as_mut().unwrap();
                    let cursor_pos = cursor.map_to_content();

                    self.schedule
                        .borrow_mut()
                        .0
                        .insert(cursor_pos.1 as usize, content.clone());
                    true
                } else {
                    false
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
