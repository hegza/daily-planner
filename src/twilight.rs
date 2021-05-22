use chrono::NaiveTime;
use std::result;

pub fn get_sunrise_sunset_online() -> result::Result<(NaiveTime, NaiveTime), ureq::Error> {
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
