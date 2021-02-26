mod time;
mod timebox;

use chrono::format::parse;
use std::str::FromStr;
use thiserror::Error;

use crate::activity::activity::{ActivityDeserializationError, ActivityKind};

use super::{
    template::{TimeBoxTemplate, TimeSlotTemplate, TimeTemplate},
    Template,
};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("attempted to parse template from empty string")]
    EmptyString,
    #[error("time box has no associated activity")]
    NoActivity,
    #[error("could not parse activity")]
    InvalidActivity(#[from] ActivityDeserializationError),
    #[error("could not parse time")]
    CantParseTime(String),
}

impl FromStr for Template {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timeboxes = Vec::new();
        for line in s.lines() {
            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Each line is a time box
            let time_box = TimeBoxTemplate::from_str(line)?;
            timeboxes.push(time_box);
        }

        Ok(Template(timeboxes))
    }
}

const SPAN_SEP_TOKEN: &str = "--";

impl FromStr for TimeSlotTemplate {
    type Err = ParseError;

    /// Parse a full time-slot template e.g. "+3:00--%H:%M" from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let span_sep = s.find(SPAN_SEP_TOKEN);

        // This is a span
        Ok(if let Some(sep_pos) = span_sep {
            let first = &s[0..sep_pos];
            let second = &s[sep_pos + SPAN_SEP_TOKEN.len()..];

            let time_1 = TimeTemplate::from_str(first)?;
            let time_2 = TimeTemplate::from_str(second)?;

            TimeSlotTemplate::Span(time_1, time_2)
        }
        // This is a simple time
        else {
            TimeSlotTemplate::Time(TimeTemplate::from_str(s)?)
        })
    }
}

impl FromStr for ActivityKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "meal" => Ok(ActivityKind::Meal),
            "sprint" => Ok(ActivityKind::Sprint),
            _ => Ok(ActivityKind::Unknown),
        }
    }
}
