use pest::error::Error as PestError;
use thiserror::Error;
use crate::parser::parser::{CppParser, Rule};
use std::io;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Value error: {0}")]
    ValueError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    PestError(#[from] PestError<Rule>),
}


