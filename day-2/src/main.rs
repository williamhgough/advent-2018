use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs;

fn main() {
    let (twos, threes) = read_input("input.txt");
    println!("Checksum: {}", twos * threes);
}

// read_input will take a filename and read the contents
// into the calibrator.
fn read_input(file: &str) -> (i32, i32) {
    let mut twos = 0;
    let mut threes = 0;

    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let iter: Vec<&str> = contents.split('\n').collect();
    for &i in &iter {
        let (has_two, has_three) = analyse_id(i);
        if has_two {
            twos += 1;
        }

        if has_three {
            threes += 1;
        }
    }

    (twos, threes)
}

fn analyse_id(id: &str) -> (bool, bool) {
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

    for (_, v) in letter_map {
        if v == 3 {
            has_three = true;
        }
        if v == 2 {
            has_two = true;
        }
    }

    (has_two, has_three)
}

#[cfg(test)]
mod test {
    #[test]
    fn works_with_two_letters() {
        let (has_two, has_three) = crate::analyse_id("aabcdefg");
        assert_eq!(has_two, true);
        assert_eq!(has_three, false);
    }

    #[test]
    fn works_with_three_letters() {
        let (has_two, has_three) = crate::analyse_id("aaabcdefg");
        assert_eq!(has_two, false);
        assert_eq!(has_three, true);
    }

    #[test]
    fn works_with_two_and_three_letters() {
        let (has_two, has_three) = crate::analyse_id("aaabcdeefg");
        assert_eq!(has_two, true);
        assert_eq!(has_three, true);
    }

    #[test]
    fn does_not_count_4_of_same_letters() {
        let (has_two, has_three) = crate::analyse_id("aaaabcdefg");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }
}
