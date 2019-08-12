//! Error and Result types. Trying to be an idiomatic Rustacean.

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ClimerError {
    NoTimeGiven,
    TimerAlreadyRunning,
    TimerNotRunning,
    NoTimeIdentifierValue(String),
    InvalidTimeIdentifier(String),
    InvalidInput(String),
    InvalidPrintIntervalValue(String),
    IoError(String),
    Unimplemented(String),
    UnknownError(String),
}

impl fmt::Display for ClimerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ClimerError::*;
        match self {
            NoTimeGiven => write!(f, "A time value must be given"),
            TimerAlreadyRunning => write!(
                f,
                "This timer is already running. Call the `stop` method before \
                 calling `start` again, or use the `restart` method"
            ),
            TimerNotRunning => write!(
                f,
                "This timer is not running. Call the `start` method before \
                 calling methods such as `update` or `stop`, which require \
                 the timer to be running before-hand"
            ),
            NoTimeIdentifierValue(c) => write!(
                f,
                "Time string '{}' was given without a value\nExample: '10{}'",
                c, c
            ),
            InvalidTimeIdentifier(input) => {
                write!(f, "Invalid time identifier: '{}'", input)
            }
            InvalidInput(input) => write!(
                f,
                "This part of the input could not be parsed: '{}'",
                input
            ),
            InvalidPrintIntervalValue(value) => write!(
                f,
                "Invalid value for '--interval': {}\nExpected an integer value",
                value
            ),
            IoError(err) => write!(f, "IO error: {}", err),
            Unimplemented(feature) => write!(
                f,
                "Sorry, this feature is not implemented yet ('{}')",
                feature
            ),
            _ => write!(f, "{}", self),
        }
    }
}

impl Error for ClimerError {
}

impl From<io::Error> for ClimerError {
    fn from(io_error: io::Error) -> Self {
        ClimerError::IoError(io_error.to_string())
    }
}

pub type ClimerResult<T = ()> = Result<T, ClimerError>;
