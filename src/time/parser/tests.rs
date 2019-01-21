use std::time::Duration;
use crate::time::prelude::*;
use super::parse_time;

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
fn parse_time_with_identifiers() {
    let input    = "10m20s";
    let expected = TimeBuilder::new()
        .minutes(10)
        .seconds(20)
        .build();
    assert_eq!(parse_time(input, None).unwrap(), expected);
}

#[test]
fn parse_time_with_identifiers_with_whitespace() {
    let input    = "10m  20s";
    let expected = TimeBuilder::new()
        .minutes(10)
        .seconds(20)
        .build();
    assert_eq!(parse_time(input, None).unwrap(), expected);
}

#[test]
fn parse_time_with_identifiers_out_of_order() {
    let input    = "20s 10m";
    let expected = TimeBuilder::new()
        .minutes(10)
        .seconds(20)
        .build();
    assert_eq!(parse_time(input, None).unwrap(), expected);
}

#[test]
fn parse_time_with_all_identifiers() {
    let input    = "1h 5m 25s 500ms 500000ns";
    let expected = TimeBuilder::new()
        .hours(1)
        .minutes(5)
        .seconds(25)
        .milliseconds(500)
        .nanoseconds(500000)
        .build();
    assert_eq!(parse_time(input, None).unwrap(), expected);
}
