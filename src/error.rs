//! Error and Result types. Trying to be an idiomatic Rustacean.

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ClimerError {
    NoTimeCharValue(char),
    UnknownError(String),
}

impl fmt::Display for ClimerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ClimerError::*;
        match *self {
            NoTimeCharValue(c) => write!(f, "Time character '{}' was given without a value", c),
            _                  => write!(f, "ClimerError {:?}", self)
        }
    }
}

impl Error for ClimerError {}

pub type ClimerResult<T = ()> = Result<T, ClimerError>;
