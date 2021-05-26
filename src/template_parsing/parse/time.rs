use std::str::FromStr;

use crate::{template_parsing::template::TimeTemplate, time::Duration};

use super::ParseError;

const TIME_SEP_TOKEN: char = ':';

impl FromStr for TimeTemplate {
    type Err = ParseError;

    /// Parse a time template from a string like "+3:00", "%H:%M", "14:00".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Separate into hours and minutes part
        let mut time_tokens = s.split(TIME_SEP_TOKEN);

        // hh:
        let hours_token = time_tokens
            .next()
            .ok_or_else(|| ParseError::CantParseTime(s.to_owned()))?;
        // :mm
        let minutes_token = time_tokens
            .next()
            .ok_or_else(|| ParseError::CantParseTime(s.to_owned()))?;

        let relative_time;
        // Relative time
        if ['+', '-'].contains(&hours_token.chars().next().expect("hours token empty")) {
            relative_time = true;
        } else {
            relative_time = false;
        }

        // TODO: more involved format detection logic
        let hours = if hours_token == "%H" {
            Ok(None)
        } else if let Ok(int) = hours_token.parse::<i8>() {
            Ok(Some(int))
        } else {
            Err(ParseError::CantParseTime(s.to_owned()))
        }?;

        let minutes = if minutes_token == "%M" {
            Ok(None)
        } else if let Ok(int) = minutes_token.parse::<i8>() {
            Ok(Some(int))
        } else {
            Err(ParseError::CantParseTime(s.to_owned()))
        }?;

        Ok(match (hours, minutes) {
            (Some(h), Some(m)) => {
                let d = Duration::hm(h, m);
                if relative_time {
                    TimeTemplate::RelativeTime(d)
                } else {
                    TimeTemplate::AbsoluteTime(d)
                }
            }
            (None, None) => TimeTemplate::TimeFormat,
            (None, Some(_)) | (Some(_), None) => unimplemented!(),
        })
    }
}
