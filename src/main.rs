mod dom;
mod editor;
mod keys;
mod schedule;
mod template_parsing;
mod time;

use chrono::NaiveTime;
use clap::{App, Arg};
use crossterm::Result;
use editor::{EditorLike, State};
use std::result;
use std::{fs, str::FromStr};
use template_parsing::{Template, TemplateMeta};
use time::Duration;
pub use time::{Clock, Time};

fn get_sunrise_sunset_online() -> result::Result<(NaiveTime, NaiveTime), ureq::Error> {
    // HACK: dawn/sunset REST testing
    let lat = "61.441443";
    let lng = "23.8658000";
    let today = chrono::Local::now().date().naive_local();
    let body_json: String = ureq::get(&format!(
        "https://api.sunrise-sunset.org/json?lat={}&lng={}&date={}",
        lat, lng, today
    ))
    .call()?
    .into_string()
    .unwrap();

    let data: serde_json::Value = serde_json::from_str(&body_json).unwrap();
    let results = data.get("results").unwrap();
    let sunrise = chrono::NaiveTime::parse_from_str(
        results.get("sunrise").unwrap().as_str().unwrap(),
        "%I:%M:%S %p",
    )
    .unwrap()
        + chrono::Duration::hours(2);
    let sunset_str = results.get("sunset").unwrap().as_str().unwrap();
    let sunset = chrono::NaiveTime::parse_from_str(sunset_str, "%I:%M:%S %p").unwrap()
        + chrono::Duration::hours(2);
    Ok((sunrise, sunset))
}

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
    let sunrise_sunset = get_sunrise_sunset_online();
    let meta = TemplateMeta {
        wake_up,
        span_len: Duration::hm(3, 15),
        sunrise: sunrise_sunset.as_ref().ok().map(|x| x.0),
        sunset: sunrise_sunset.ok().map(|x| x.1),
    };
    let schedule = template.schedule(meta);

    // Create the editor
    let mut editor = State::try_from_schedule(schedule)?;

    // Capture IO in main loop
    editor.attach();

    Ok(())
}
