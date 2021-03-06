use std::{fmt, str::FromStr};
use thiserror::Error;

/// Represents an activity with a kind and a summary. Kind is unkown my default.
#[derive(Clone, Debug)]
pub struct Activity {
    pub summary: String,
    pub kind: ActivityKind,
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            summary: String::new(),
            kind: ActivityKind::Unknown,
        }
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.summary)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActivityKind {
    Unknown,
    Meal,
    Sprint,
}

// Errors

#[derive(Error, Debug)]
pub enum ActivityDeserializationError {}

impl FromStr for Activity {
    type Err = ActivityDeserializationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // TODO: determine kind
        Ok(Activity {
            kind: ActivityKind::Unknown,
            summary: s.to_owned(),
        })
    }
}
