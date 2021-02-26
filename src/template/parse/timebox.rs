use itertools::Itertools;
use std::{
    iter::Peekable,
    str::{FromStr, SplitWhitespace},
};

use crate::{
    activity::{activity::ActivityKind, Activity},
    template::template::{TimeBoxTemplate, TimeSlotTemplate},
};

use super::ParseError;
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
    activity_kind_identified: bool,
}

impl<'t> TemplateTimeBoxParser<'t> {
    pub fn from_tokens(tokens: Peekable<SplitWhitespace<'t>>) -> Self {
        TemplateTimeBoxParser {
            tokens,
            timeslot_detected: false,
            activity_kind_identified: false,
        }
    }

    pub fn generate(mut self) -> Result<TimeBoxTemplate, ParseError> {
        let first_token = self.tokens.peek();

        if first_token == None {
            return Err(ParseError::EmptyString);
        }

        let next = *first_token.unwrap();
        let first_char = next.trim().chars().nth(0).unwrap();
        if MARKDOWN_LIST_TOKENS.contains(&first_char) {
            // Skip the list token
            self.tokens.next().unwrap();
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
                TokenKind::ActivityKind(kind) => {
                    if activity.is_none() {
                        activity = Some(Activity::default());
                    }
                    activity.as_mut().unwrap().kind = kind;
                }
                TokenKind::ActivityText => {
                    let all = std::iter::once(next).chain(self.tokens.clone()).join(" ");
                    if activity.is_none() {
                        activity = Some(Activity::default());
                    }
                    activity.as_mut().unwrap().summary = all;
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

    /// Figure out which kind of token this one is. State machine. Sets self.timeslot_detected as true if a timeslot is parsed.
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

        // If no kind is detected for activity, try if one can be parsed
        if !self.activity_kind_identified {
            if token.chars().last().unwrap() == ':' {
                let activity_maybe = token
                    .chars()
                    .take(token.chars().count() - 1)
                    .collect::<String>();
                if let Ok(activity_kind) = ActivityKind::from_str(&activity_maybe) {
                    if activity_kind != ActivityKind::Unknown {
                        self.activity_kind_identified = true;
                        return TokenKind::ActivityKind(activity_kind);
                    }
                }
            }
        }

        // If nothing else applies, this token is "content"
        TokenKind::ActivityText
    }
}

enum TokenKind {
    ListInitializer,
    Time(TimeSlotTemplate),
    ActivityKind(ActivityKind),
    ActivityText,
}
