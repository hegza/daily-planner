mod activity;
mod editor;
mod schedule;
mod template;
mod time;

use clap::{App, Arg};
use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{fs, str::FromStr};
use template::{Template, TemplateMeta};
use time::Duration;
pub use time::Time;

fn main() -> Result<()> {
    let matches = App::new("daily-planner")
        .arg(Arg::from_usage(
            "-w --wake-up=[TIME] 'Sets the wake-up time'",
        ))
        .get_matches();

    let default_wake_up = Time::hm(9, 0);
    let wake_up = if let Some(wake_up) = matches.value_of("wake-up") {
        Time::from_str(wake_up).unwrap_or(default_wake_up)
    } else {
        default_wake_up
    };

    let template_text = fs::read_to_string("data/template.md").unwrap();
    let template = Template::from_str(&template_text).unwrap();

    let meta = TemplateMeta {
        wake_up,
        span_len: Duration::hours(3),
    };
    let schedule = template.schedule(meta);

    for time_box in schedule.0 {
        let t_str = match time_box.time {
            Some(t) => format!("{}", t),
            None => "     ".to_owned(),
        };
        println!("{:<12} {}", t_str, time_box.activity);
    }
    /*
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent("█".magenta()))?;
            }
        }
    }
    stdout.flush()?;
    */
    Ok(())
}
