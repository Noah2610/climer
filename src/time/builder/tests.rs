use super::super::Time;
use super::TimeBuilder;

fn get_expected_time() -> Time {
    Time {
        hours:        1,
        minutes:      10,
        seconds:      20,
        milliseconds: 500,
    }
}

#[test]
fn builds_time() {
    let expected = get_expected_time();
    let actual   = TimeBuilder::new()
        .hours(1)
        .minutes(10)
        .seconds(20)
        .milliseconds(500)
        .build();
    assert_eq!(actual, expected);
}

fn handles_overflow() {
    let expected = get_expected_time();
    let actual   = TimeBuilder::new()
        //                   hours               minutes          seconds  milliseconds
        .milliseconds((1 * 60 * 60 * 1000) + (10 * 60 * 1000) + (20 * 1000) + 500)
        .build();
    assert_eq!(actual, expected);
}
