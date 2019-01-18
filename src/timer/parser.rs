use crate::error::{ ClimerResult, ClimerError };
use crate::settings::parser::codes::*;
use super::{ Time, TimeBuilder };

use self::TimeType::*;

enum TimeType {
    Hour,
    Minute,
    Second,
    Millisecond,
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
        (Millisecond, MILLISECOND)
    ].iter() {
        if let Some(pos) = time.find(*letter) {
            if let Some(num) = last_number(&time[0 .. pos]) {
                builder = match time_type {
                    Hour        => builder.hours(num),
                    Minute      => builder.minutes(num),
                    Second      => builder.seconds(num),
                    Millisecond => builder.milliseconds(num),
                };
            } else {
                // Time char was given without a value
                return Err(ClimerError::NoTimeCharValue(*letter));
            }
        }
    }

    Ok(builder.build())
}

fn last_number(s: &str) -> Option<u32> {
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
mod tests {
    use std::time::Duration;
    use super::{ parse_time, last_number };
    use super::super::time::{ Time, TimeBuilder };

    fn get_expected_time() -> Time {
        TimeBuilder::new()
            .hours(1)
            .minutes(10)
            .seconds(20)
            .milliseconds(30)
            .build()
    }

    #[test]
    #[ignore]
    fn parse_time_with_format_with_punctuations() {
        let format = "M:S";
        let punctuations = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        for punct in punctuations.chars() {
            let input    = &format!("10{}20", punct);
            let expected = get_expected_time();
            assert_eq!(parse_time(input, Some(format)).unwrap(), expected);
        }
    }

    #[test]
    fn parse_time_with_denominators() {
        let input    = "10M20S";
        let expected = TimeBuilder::new()
            .minutes(10)
            .seconds(20)
            .build();
        assert_eq!(parse_time(input, None).unwrap(), expected);
    }

    #[test]
    fn parse_time_with_denominators_with_whitespace() {
        let input    = "10M  20S";
        let expected = TimeBuilder::new()
            .minutes(10)
            .seconds(20)
            .build();
        assert_eq!(parse_time(input, None).unwrap(), expected);
    }

    #[test]
    fn parse_time_with_denominators_out_of_order() {
        let input    = "20S 10M";
        let expected = TimeBuilder::new()
            .minutes(10)
            .seconds(20)
            .build();
        assert_eq!(parse_time(input, None).unwrap(), expected);
    }

    #[test]
    fn last_number_with_pure_number() {
        let s = "1234";
        assert_eq!(last_number(s), Some(1234));
    }

    #[test]
    fn last_number_with_trailing_char() {
        let s = "1234abc";
        assert_eq!(last_number(s), Some(1234));
    }

    #[test]
    fn last_number_with_leading_char() {
        let s = "abc1234";
        assert_eq!(last_number(s), Some(1234));
    }

    #[test]
    fn last_number_with_leading_and_trailing_char() {
        let s = "abc1234def";
        assert_eq!(last_number(s), Some(1234));
    }

    #[test]
    fn last_number_with_multiple_nums() {
        let s = "123abc456";
        assert_eq!(last_number(s), Some(456));
    }

    #[test]
    fn last_number_with_multiple_nums_and_trailing_char() {
        let s = "123abc456def";
        assert_eq!(last_number(s), Some(456));
    }

    #[test]
    fn last_number_with_multiple_nums_and_leading_char() {
        let s = "abc123def456";
        assert_eq!(last_number(s), Some(456));
    }

    #[test]
    fn last_number_with_multiple_nums_and_leading_and_trailing_char() {
        let s = "abc123def456ghi";
        assert_eq!(last_number(s), Some(456));
    }

    #[test]
    fn last_number_with_pure_string() {
        let s = "abc";
        assert_eq!(last_number(s), None);
    }

    #[test]
    fn last_number_with_empty_string() {
        let s = "";
        assert_eq!(last_number(s), None);
    }
}
