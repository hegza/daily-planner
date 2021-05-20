use std::io;

use crossterm::ErrorKind;
use thiserror::Error;

/// Represents all errors that can happen while we are in the input/display loop
/// of the editor.
#[derive(Error, Debug)]
pub enum Error {
    #[error("crossterm error")]
    Crossterm(#[from] ErrorKind),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("strfmt format error")]
    Strfmt(#[from] strfmt::FmtError),
}
