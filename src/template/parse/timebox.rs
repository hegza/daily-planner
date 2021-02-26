use itertools::Itertools;
use std::{
    iter::Peekable,
    str::{FromStr, SplitWhitespace},
};

use crate::{
    activity::Activity,
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

        let next = *first.unwrap();
        let first_char = next.trim().chars().nth(0).unwrap();
        if MARKDOWN_LIST_TOKENS.contains(&first_char) {
            // Skip it
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
