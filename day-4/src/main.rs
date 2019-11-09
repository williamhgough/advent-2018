use chrono::{DateTime, FixedOffset};
use regex::Regex;
//use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut commands: Vec<&str> = contents.split('\n').collect();
    commands.sort_by(|&a, &b| {
        let first_date = parse_date(a);
        let second_date = parse_date(b);
        first_date.partial_cmp(&second_date).unwrap()
    });
}

//enum GuardActivity {
//    ShiftStarted,
//    FallsAsleep,
//    WakesUp,
//}
//
//struct Guard {
//    id: String,
//    schedule: HashMap<DateTime<Utc>, GuardActivity>,
//}

fn parse_date(text: &str) -> DateTime<FixedOffset> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
\s
(?P<hours>\d{2}) # the hours
:
(?P<mins>\d{2}) # the minutes
",
        )
        .unwrap();
    }

    let captures = RE.captures(text).unwrap();
    let formatted = format!("{}:00 {}", captures.get(0).unwrap().as_str(), "+0000");
    DateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S %z").unwrap()
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, FixedOffset};

    #[test]
    fn parse_date_works() {
        let expected: DateTime<FixedOffset> =
            DateTime::parse_from_str("1518-11-22 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap();
        assert_eq!(
            expected.to_string(),
            crate::parse_date("[1518-11-22 00:00] Guard #1231 begins shift").to_string()
        );
    }
}
