use crate::{error::SunriseApiError, Error};
use chrono::NaiveTime;
use std::result;

pub fn get_sunrise_sunset_online() -> result::Result<(NaiveTime, NaiveTime), Error> {
    // HACK: dawn/sunset REST testing
    let lat = "61.441443";
    let lng = "23.8658000";
    let today = chrono::Local::now().date().naive_local();
    let body_json: String = ureq::get(&format!(
        "https://api.sunrise-sunset.org/json?lat={}&lng={}&date={}",
        lat, lng, today
    ))
    .call()
    .map_err(Box::new)?
    .into_string()?;

    let data: serde_json::Value = serde_json::from_str(&body_json)?;
    let results = data
        .get("results")
        .ok_or_else(|| SunriseApiError("'results' not contained in body".to_owned()))?;
    let sunrise = chrono::NaiveTime::parse_from_str(
        results
            .get("sunrise")
            .ok_or_else(|| {
                SunriseApiError("'sunrise' not contained in results returned from API".to_owned())
            })?
            .as_str()
            .ok_or_else(|| SunriseApiError("value of 'sunrise' was empty string".to_owned()))?,
        "%I:%M:%S %p",
    )? + chrono::Duration::hours(2);
    let sunset_str = results
        .get("sunset")
        .ok_or_else(|| {
            SunriseApiError("'sunset' not contained in results returned from API".to_owned())
        })?
        .as_str()
        .ok_or_else(|| SunriseApiError("value of 'sunset' was empty string".to_owned()))?;
    let sunset =
        chrono::NaiveTime::parse_from_str(sunset_str, "%I:%M:%S %p")? + chrono::Duration::hours(2);
    Ok((sunrise, sunset))
}
