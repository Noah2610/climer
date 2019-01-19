use crate::error::{ ClimerResult, ClimerError };
use crate::settings::parser::codes::*;
use super::{ Time, TimeBuilder };

use self::TimeType::*;

enum TimeType {
    Hour,
    Minute,
    Second,
    Millisecond,
    Nanosecond,
}

pub fn parse_time(time: &str, format_opt: Option<&str>) -> ClimerResult<Time> {
    Ok(if let Some(format) = format_opt {
        parse_time_with_format(time, format)?
    } else {
        parse_time_without_format(time)?
    })
}

fn parse_time_without_format(time: &str) -> ClimerResult<Time> {
    let mut builder = TimeBuilder::new();

    for (time_type, letter) in [
        (Hour, HOUR),
        (Minute, MINUTE),
        (Second, SECOND),
        (Millisecond, MILLISECOND),
        (Nanosecond, NANOSECOND)
    ].iter() {
        if let Some(pos) = time.find(*letter) {
            if let Some(num) = last_number(&time[0 .. pos]) {
                builder = match time_type {
                    Hour        => builder.hours(num),
                    Minute      => builder.minutes(num),
                    Second      => builder.seconds(num),
                    Millisecond => builder.milliseconds(num),
                    Nanosecond  => builder.nanoseconds(num),
                };
            } else {
                // Time char was given without a value
                return Err(ClimerError::NoTimeCharValue(*letter));
            }
        }
    }

    Ok(builder.build())
}

fn last_number(s: &str) -> Option<u64> {
    if s.is_empty() { return None; }
    let mut s = s.to_string();
    while !s.chars().last().unwrap().is_digit(10) {
        s.pop();
        if s.is_empty() { return None; }
    }
    let start = if let Some(index) = s.rfind( |c: char| !c.is_digit(10) ) {
        index + 1
    } else {
        0
    };
    (&s[start .. ]).parse().ok()
}

fn parse_time_with_format(time: &str, format: &str) -> ClimerResult<Time> {
    unimplemented!()
}

#[cfg(test)]
mod tests;
