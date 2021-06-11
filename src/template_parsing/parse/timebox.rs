use itertools::Itertools;
use std::{
    iter::Peekable,
    str::{FromStr, SplitWhitespace},
};

use crate::{
    dom::{activity::ActivityKind, Activity},
    template_parsing::template::{TimeBoxTemplate, TimeSlotTemplate},
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

        let next = *first_token.ok_or(ParseError::EmptyString)?;
        let first_char = next.trim().chars().next().ok_or(ParseError::EmptyString)?;
        if MARKDOWN_LIST_TOKENS.contains(&first_char) {
            // Skip the list token
            self.tokens.next().expect("prgrammer logic error");
        }

        let mut time = None;
        let mut activity = None;

        while let Some(next) = self.tokens.next() {
            match self.token_kind(next)? {
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
                    activity.as_mut().expect("programmer logic error").kind = kind;
                }
                TokenKind::ActivityText => {
                    let all = std::iter::once(next).chain(self.tokens.clone()).join(" ");
                    if activity.is_none() {
                        activity = Some(Activity::default());
                    }
                    activity.as_mut().expect("programmer logic error").summary = all;
                    break;
                }
            }
        }

        match activity {
            Some(activity) => Ok(TimeBoxTemplate { time, activity }),
            None => Err(ParseError::NoActivity),
        }
    }

    /// Figure out which kind of token this one is. State machine. Sets
    /// self.timeslot_detected as true if a timeslot is parsed.
    fn token_kind(&mut self, token: &str) -> Result<TokenKind, ParseError> {
        let token = token.trim();

        // List initializer, e.g. '-' or '*'
        if token.len() == 1
            && MARKDOWN_LIST_TOKENS.contains(&token.chars().next().ok_or(ParseError::EmptyString)?)
        {
            return Ok(TokenKind::ListInitializer);
        }

        // If no time-slot is detected and the token parses into one, use that
        if !self.timeslot_detected {
            if let Ok(time_slot) = TimeSlotTemplate::from_str(token) {
                self.timeslot_detected = true;
                return Ok(TokenKind::Time(time_slot));
            }
        }

        // If no kind is detected for activity, try if one can be parsed
        if !self.activity_kind_identified && token.ends_with(':') {
            let activity_maybe = token
                .chars()
                .take(token.chars().count() - 1)
                .collect::<String>();
            if let Ok(activity_kind) = ActivityKind::from_str(&activity_maybe) {
                if activity_kind != ActivityKind::Unknown {
                    self.activity_kind_identified = true;
                    return Ok(TokenKind::ActivityKind(activity_kind));
                }
            }
        }

        // If nothing else applies, this token is "content"
        Ok(TokenKind::ActivityText)
    }
}

enum TokenKind {
    ListInitializer,
    Time(TimeSlotTemplate),
    ActivityKind(ActivityKind),
    ActivityText,
}
