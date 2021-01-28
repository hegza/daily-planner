mod activity;
mod editor;
mod schedule;
mod template;
mod time;

use activity::timebox::TimeSlotKind;
use clap::{App, Arg};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, style, Colorize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand, Result,
};
use std::{
    fs,
    io::{stdout, Write},
    str::FromStr,
};
use template::{Template, TemplateMeta};
use time::Duration;
pub use time::{Clock, Time};

fn main() -> Result<()> {
    let matches = App::new("daily-planner")
        .arg(Arg::from_usage(
            "-w --wake-up=[TIME] 'Sets the wake-up time. It will auto-round to half an hour.'",
        ))
        .get_matches();

    let default_wake_up = Time::hm(9, 0);
    let wake_up = if let Some(wake_up) = matches.value_of("wake-up") {
        Time::from_str(wake_up).unwrap_or(default_wake_up)
    } else {
        default_wake_up
    }
    .round_to_half();

    let template_text = fs::read_to_string("data/template.md").unwrap();
    let template = Template::from_str(&template_text).unwrap();

    let meta = TemplateMeta {
        wake_up,
        span_len: Duration::hm(3, 15),
    };
    let schedule = template.schedule(meta);

    let mut stdout = stdout();

    enable_raw_mode()?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;

    let line_count = schedule.0.len();
    for (line_y, time_box) in schedule.0.iter().enumerate() {
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
    if let Some(last_timed_item) = schedule
        .0
        .iter()
        .rev()
        .find_map(|time_box| time_box.time.clone())
    {
        let first_timed_item = schedule
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
        //let time_left: Time = (Duration::hm(24, 0) + first_time.into() - last_time.into()).into;
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

    // Detect keys
    loop {
        match read().unwrap() {
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

    disable_raw_mode()?;

    Ok(())
}
