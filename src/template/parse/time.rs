use std::str::FromStr;

use crate::{template::template::TimeTemplate, time::Duration};

use super::ParseError;

const TIME_SEP_TOKEN: char = ':';

impl FromStr for TimeTemplate {
    type Err = ParseError;

    /// Parse a time template from a string like "+3:00" or "%H:%M".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Separate into hours and minutes part
        let mut time_tokens = s.split(TIME_SEP_TOKEN);

        let hours_token = time_tokens
            .next()
            .ok_or(ParseError::CantParseTime(s.to_owned()))?;
        let minutes_token = time_tokens
            .next()
            .ok_or(ParseError::CantParseTime(s.to_owned()))?;

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
            (Some(h), Some(m)) => TimeTemplate::RelativeTime(Duration::hm(h, m)),
            (None, None) => TimeTemplate::TimeFormat,
            (None, Some(_)) | (Some(_), None) => unimplemented!(),
        })
    }
}
