use crate::editor;

use thiserror::Error;

use std::io;

/// Represents all errors that can in this application
/// of the editor.
#[derive(Error, Debug)]
pub enum Error {
    #[error("editor error")]
    Editor(#[from] editor::Error),
    #[error("ureq error")]
    Ureq(#[from] Box<ureq::Error>),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("chrono parse error")]
    ChronoParse(#[from] chrono::ParseError),
    #[error("sunrise/sunset API error")]
    SunriseApi(#[from] SunriseApiError),
    #[error("serde JSON error")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
#[error("sunrise/sunset API error {0}")]
pub struct SunriseApiError(pub String);
