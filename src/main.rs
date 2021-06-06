use clap::{App, Arg};
use crossterm::Result;
use daily_planner::editor::{EditorLike, State};
use daily_planner::template_parsing::{Template, TemplateMeta};
use daily_planner::time::Duration;
pub use daily_planner::time::{Clock, Time};
use fs_err as fs;
use std::io::stdout;
use std::str::FromStr;

fn main() -> std::result::Result<(), daily_planner::editor::Error> {
    let matches = App::new("daily-planner")
        .arg(Arg::from_usage(
            "-w --wake-up=[TIME] 'Sets the wake-up time. Will be rounded to next half an hour.'",
        ))
        .arg(Arg::from_usage(
            "--wake-up-tomorrow=[TIME] 'Sets the wake-up time of tomorrow. Will be rounded to next half an hour.'",
        ))
        .arg(Arg::from_usage(
            "-t --template=[FILE] 'Sets the schedule template.'",
        ))
        .get_matches();

    // Determine time of wake up
    let default_wake_up = Time::hm(9, 0);
    let wake_up_today = if let Some(wake_up) = matches.value_of("wake-up") {
        Time::from_str(wake_up).unwrap_or(default_wake_up)
    } else {
        default_wake_up
    }
    .round_to_half();
    let wake_up_tomorrow = if let Some(wake_up_tomorrow) = matches.value_of("wake-up-tomorrow") {
        Time::from_str(wake_up_tomorrow).unwrap_or(wake_up_today)
    } else {
        wake_up_today
    }
    .round_to_half();

    // Load template
    let default_template_file = "data/template.md";
    let template_file = match matches.value_of("template") {
        Some(f) => f,
        None => default_template_file,
    };
    let template_text = fs::read_to_string(template_file).expect("could not read file");
    let template = Template::from_str(&template_text)?;

    // Create schedule from template
    let sunrise_sunset = daily_planner::twilight::get_sunrise_sunset_online();
    let meta = TemplateMeta {
        wake_up_today,
        wake_up_tomorrow,
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
