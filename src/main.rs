mod activity;
mod edit_schedule;
mod editor;
mod keys;
mod schedule;
mod template;
mod time;

use clap::{App, Arg};
use crossterm::Result;
use editor::{Editor, EditorLike};
use std::{fs, str::FromStr};
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

    let mut editor = Editor::spawn(schedule)?;

    // Capture IO in main loop
    editor.attach();

    Ok(())
}
