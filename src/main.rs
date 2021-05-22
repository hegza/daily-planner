mod dom;
mod editor;
mod keys;
mod schedule;
mod template_parsing;
mod time;

use clap::{App, Arg};
use crossterm::Result;
use editor::{EditorLike, State};
use std::io::stdout;
use std::{fs, str::FromStr};
use template_parsing::{Template, TemplateMeta};
use time::Duration;
pub use time::{Clock, Time};

fn main() -> Result<()> {
    let matches = App::new("daily-planner")
        .arg(Arg::from_usage(
            "-w --wake-up=[TIME] 'Sets the wake-up time. It will auto-round to half an hour.'",
        ))
        .get_matches();

    // Determine time of wake up
    let default_wake_up = Time::hm(9, 0);
    let wake_up = if let Some(wake_up) = matches.value_of("wake-up") {
        Time::from_str(wake_up).unwrap_or(default_wake_up)
    } else {
        default_wake_up
    }
    .round_to_half();

    // Load template
    let template_text = fs::read_to_string("data/template.md").unwrap();
    let template = Template::from_str(&template_text).unwrap();

    // Create schedule from template
    let sunrise_sunset = daily_planner::twilight::get_sunrise_sunset_online();
    let meta = TemplateMeta {
        wake_up,
        span_len: Duration::hm(3, 15),
        sunrise: sunrise_sunset.as_ref().ok().map(|x| x.0),
        sunset: sunrise_sunset.ok().map(|x| x.1),
    };
    let schedule = template.schedule(meta);

    // Create the editor
    let stdout = stdout();
    let mut editor = State::try_from_schedule(schedule, stdout)?;

    // Capture IO in main loop
    editor.attach();

    Ok(())
}
