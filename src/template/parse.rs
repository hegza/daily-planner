use itertools::Itertools;
use std::{
    iter::Peekable,
    str::{FromStr, SplitWhitespace},
};
use thiserror::Error;

use crate::{
    activity::{activity::ActivityDeserializationError, Activity},
    time::Duration,
};

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

const MARKDOWN_LIST_TOKENS: &[char] = &['-', '*'];

impl FromStr for TimeBoxTemplate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().peekable();

        let parser = TemplateTimeBoxParser::from_tokens(tokens);

        parser.generate()
    }
}

struct TemplateTimeBoxParser<'t> {
    tokens: Peekable<SplitWhitespace<'t>>,
    timeslot_detected: bool,
}

impl<'t> TemplateTimeBoxParser<'t> {
    pub fn from_tokens(tokens: Peekable<SplitWhitespace<'t>>) -> Self {
        TemplateTimeBoxParser {
            tokens,
            timeslot_detected: false,
        }
    }

    pub fn generate(mut self) -> Result<TimeBoxTemplate, ParseError> {
        let first = self.tokens.peek();

        if first == None {
            return Err(ParseError::EmptyString);
        }

        let mut next = *first.unwrap();
        let first_char = next.trim().chars().nth(0).unwrap();
        if MARKDOWN_LIST_TOKENS.contains(&first_char) {
            // Skip it
            next = self.tokens.next().unwrap();
        }

        let mut time = None;
        let mut activity = None;

        while let Some(next) = self.tokens.next() {
            match self.token_kind(next) {
                TokenKind::ListInitializer => {
                    unreachable!();
                }
                TokenKind::Time(timeslot_template) => {
                    // TODO: initialize appropriately
                    time = Some(timeslot_template);
                }
                TokenKind::ActivityText => {
                    let all = std::iter::once(next).chain(self.tokens.clone()).join(" ");
                    activity = Some(Activity::from_str(&all)?);
                    break;
                }
            }
        }

        match activity {
            Some(activity) => Ok(TimeBoxTemplate { time, activity }),
            None => {
                return Err(ParseError::NoActivity);
            }
        }
    }

    /// Sets self.timeslot_detected as true if a timeslot is parsed.
    fn token_kind(&mut self, token: &str) -> TokenKind {
        let token = token.trim();

        // List initializer, e.g. '-' or '*'
        if token.len() == 1 && MARKDOWN_LIST_TOKENS.contains(&token.chars().nth(0).unwrap()) {
            return TokenKind::ListInitializer;
        }

        // If no time-slot is detected and the token parses into one, use that
        if !self.timeslot_detected {
            if let Ok(time_slot) = TimeSlotTemplate::from_str(token) {
                self.timeslot_detected = true;
                return TokenKind::Time(time_slot);
            }
        }

        // If nothing else applies, this token is "content"
        TokenKind::ActivityText
    }
}

enum TokenKind {
    ListInitializer,
    Time(TimeSlotTemplate),
    ActivityText,
}

const SPAN_SEP_TOKEN: &str = "--";
const TIME_SEP_TOKEN: char = ':';

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
        } else if let Ok(int) = hours_token.parse::<u8>() {
            Ok(Some(int))
        } else {
            Err(ParseError::CantParseTime(s.to_owned()))
        }?;

        let minutes = if minutes_token == "%M" {
            Ok(None)
        } else if let Ok(int) = minutes_token.parse::<u8>() {
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
