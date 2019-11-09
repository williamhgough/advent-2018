use chrono::{DateTime, FixedOffset};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut guards: Vec<Guard> = vec![];
    // read the puzzle input
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut commands: Vec<&str> = contents.split('\n').collect();
    // sort the commands in chronological order
    commands.sort_by(|&a, &b| {
        let first_date = parse_date(a);
        let second_date = parse_date(b);
        first_date.partial_cmp(&second_date).unwrap()
    });

    for command in commands {
        let time = parse_date(command);
        if is_new_guard(command) {
            let mut g = Guard {
                id: parse_guard_id(command),
                schedule: HashMap::new(),
            };
            g.schedule.insert(time, GuardActivity::ShiftStarted);
            guards.push(g);
            continue;
        }

        let activity = parse_activity(command);
        let len = guards.len();
        let last_guard = guards.get_mut(len - 1).unwrap();
        last_guard.schedule.insert(time, activity);
    }

    println!("Guard count: {}", guards.len());
    guards.sort_by(|a, b| a.asleep_minutes().cmp(&b.asleep_minutes()));
    println!("{:?}", guards[guards.len() - 1].asleep_minutes());
}

#[derive(Debug, PartialOrd, PartialEq)]
enum GuardActivity {
    ShiftStarted,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Guard {
    id: String,
    schedule: HashMap<DateTime<FixedOffset>, GuardActivity>,
}

impl Guard {
    fn asleep_minutes(&self) -> i32 {
        self.schedule
            .iter()
            .filter(|&(_, v)| {
                if *v == GuardActivity::FallsAsleep {
                    return true;
                }
                false
            })
            .count() as i32
    }
}

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

fn is_new_guard(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Guard\s#[0-9]+").unwrap();
    }
    RE.is_match(input)
}

fn parse_guard_id(input: &str) -> String {
    let chunks: Vec<&str> = input.split(' ').collect();
    chunks[3].replace("#", "")
}

fn parse_activity(input: &str) -> GuardActivity {
    lazy_static! {
        static ref SLEEPING: Regex = Regex::new(r"asleep").unwrap();
    }

    if SLEEPING.is_match(input) {
        return GuardActivity::FallsAsleep;
    }

    // if sleeping is not a match, the only other command this could be
    // is the guard waking up
    GuardActivity::WakesUp
}

#[cfg(test)]
mod test {
    use crate::GuardActivity;
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

    #[test]
    fn is_new_guard_works() {
        assert_eq!(
            true,
            crate::is_new_guard("[1518-11-22 00:00] Guard #1231 begins shift")
        );
        assert_eq!(false, crate::is_new_guard("[1518-11-22 00:00] wakes up"))
    }

    #[test]
    fn parse_guard_id_works() {
        assert_eq!(
            "1234",
            crate::parse_guard_id("[1518-11-22 00:00] Guard #1234 begins shift")
        );
        assert_eq!(
            "3371",
            crate::parse_guard_id("[1518-11-22 00:00] Guard #3371 begins shift")
        );
    }

    #[test]
    fn parse_activity_works() {
        assert_eq!(
            format!("{:?}", GuardActivity::FallsAsleep),
            format!(
                "{:?}",
                crate::parse_activity("[1518-04-28 00:19] falls asleep")
            )
        );
        assert_eq!(
            format!("{:?}", GuardActivity::WakesUp),
            format!("{:?}", crate::parse_activity("[1518-04-09 00:36] wakes up"))
        );
    }
}
