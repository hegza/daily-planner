use std::io;

use crossterm::ErrorKind;
use thiserror::Error;

use crate::template_parsing;

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
    #[error("template parse error")]
    TemplateParse(#[from] template_parsing::ParseError),
    #[error("resource ownership error")]
    ResourceOwnership(#[from] ResourceOwnershipError),
}

#[derive(Error, Debug)]
#[error("ownership violation")]
pub struct ResourceOwnershipError(pub String);
