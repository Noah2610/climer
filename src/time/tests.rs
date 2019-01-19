use super::Time;

fn get_expected_time() -> Time {
    // 1 hours, 30 minutes, 25 seconds, 5000 milliseconds
    // 01:30:30
    Time::new(1, 30, 25, 5000, 0)
}

#[test]
fn time_as_hours() {
    let expected = 1.5083333;
    let actual   = get_expected_time().as_hours();
    assert_eq!(actual, expected);
}

#[test]
fn time_as_minutes() {
    let expected = 90.5;
    let actual   = get_expected_time().as_minutes();
    assert_eq!(actual, expected);
}

#[test]
fn time_as_seconds() {
    let expected = 5430.0;
    let actual   = get_expected_time().as_seconds();
    assert_eq!(actual, expected);
}

#[test]
fn time_as_milliseconds() {
    let expected = 5430000.0;
    let actual   = get_expected_time().as_milliseconds();
    assert_eq!(actual, expected);
}

#[test]
fn time_as_nanoseconds() {
    let expected = 5.43e12;
    let actual   = get_expected_time().as_nanoseconds();
    assert_eq!(actual, expected);
}
