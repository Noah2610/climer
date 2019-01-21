//! Error and Result types. Trying to be an idiomatic Rustacean.

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ClimerError {
    NoTimeIdentifierValue(String),
    InvalidTimeIdentifier(String),
    InvalidInput(String),
    Unimplemented(String),
    UnknownError(String),
}

impl fmt::Display for ClimerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ClimerError::*;
        match self {
            NoTimeIdentifierValue(c) =>
                write!(f,
                       "Time string '{}' was given without a value\nExample: '10{}'",
                       c, c),
            InvalidTimeIdentifier(input) =>
                write!(f,
                       "Invalid time identifier: '{}'",
                       input),
            InvalidInput(input) =>
                write!(f,
                       "This part of the input could not be parsed: '{}'",
                       input),
            Unimplemented(feature) =>
                write!(f,
                       "Sorry, this feature is not implemented yet ('{}')",
                       feature),
                _ =>
                write!(f,
                       "ClimerError {:?}", self)
        }
    }
}

impl Error for ClimerError {}

pub type ClimerResult<T = ()> = Result<T, ClimerError>;
