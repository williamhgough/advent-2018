use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs;

fn main() {
    let checksum = find_checksum("input.txt");
    println!("Checksum: {}", checksum);

    if let Ok(result) = find_common("input.txt") {
        println!("{}", result);
    }
}

// read_input will take a filename and read the contents
// into the calibrator.
fn find_checksum(file: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let iter: Vec<&str> = contents.split('\n').collect();
    for &i in &iter {
        let (has_two, has_three) = find_id_letter_counts(i);
        if has_two {
            twos += 1;
        }

        if has_three {
            threes += 1;
        }
    }

    twos * threes
}

fn find_id_letter_counts(id: &str) -> (bool, bool) {
    let mut has_two = false;
    let mut has_three = false;
    let mut letter_map: HashMap<char, i32> = HashMap::new();

    for c in id.chars() {
        match letter_map.entry(c) {
            Vacant(entry) => entry.insert(1),
            Occupied(entry) => {
                let v = entry.into_mut();
                *v += 1;
                v
            }
        };
    }

    for &v in letter_map.values() {
        if v == 3 {
            has_three = true;
        }
        if v == 2 {
            has_two = true;
        }
    }

    (has_two, has_three)
}

fn find_common(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let iter: Vec<&str> = contents.split('\n').collect();
    for i in 0..iter.len() {
        for j in &i + 1..iter.len() {
            if let Some(x) = common_correct_letters(&iter[i], &iter[j]) {
                return Ok(x);
            }
        }
    }
    Err(From::from("could not find to correct Box IDs"))
}

fn common_correct_letters(first: &str, second: &str) -> Option<String> {
    if first.len() != second.len() {
        return None;
    }

    let mut found_one_wrong = false;
    for (a, b) in first.chars().zip(second.chars()) {
        if a != b {
            if found_one_wrong {
                return None;
            }
            found_one_wrong = true;
        }
    }
    Some(
        first
            .chars()
            .zip(second.chars())
            .filter(|&(a, b)| a == b)
            .map(|(c, _)| c)
            .collect(),
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn works_with_two_letters() {
        let (has_two, has_three) = crate::find_id_letter_counts("aabcdefg");
        assert_eq!(has_two, true);
        assert_eq!(has_three, false);
    }

    #[test]
    fn works_with_three_letters() {
        let (has_two, has_three) = crate::find_id_letter_counts("aaabcdefg");
        assert_eq!(has_two, false);
        assert_eq!(has_three, true);
    }

    #[test]
    fn works_with_two_and_three_letters() {
        let (has_two, has_three) = crate::find_id_letter_counts("aaabcdeefg");
        assert_eq!(has_two, true);
        assert_eq!(has_three, true);
    }

    #[test]
    fn does_not_count_4_of_same_letters() {
        let (has_two, has_three) = crate::find_id_letter_counts("aaaabcdefg");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }
}
