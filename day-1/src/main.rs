use std::collections::HashMap;
use std::fs;

fn main() {
    let mut calibrator = Calibrator::new(0);
    calibrator.read_input("input.txt");
    println!("{}", calibrator.frequency());
}

struct Calibrator {
    current_frequency: i32,
    frequency_count: HashMap<i32, i32>,
}

impl Calibrator {
    fn new(initial_value: i32) -> Calibrator {
        Calibrator {
            current_frequency: initial_value,
            frequency_count: HashMap::new(),
        }
    }
    // read_input will take a filename and read the contents
    // into the calibrator.
    fn read_input(&mut self, file: &str) {
        let mut finished = false;

        let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
        let iter: Vec<&str> = contents.split('\n').collect();
        while !finished {
            for i in &iter {
                if let true = self.update_frequency(i.parse::<i32>().unwrap()) {
                    println!("seen frequency {} twice!", self.current_frequency);
                    finished = true;
                    break;
                }
            }
        }
    }
    // update_frequency adds a value to the current frequency.
    fn update_frequency(&mut self, value: i32) -> bool {
        self.current_frequency += value;
        self.bump_seen_count()
    }

    fn bump_seen_count(&mut self) -> bool {
        if let None = self.frequency_count.get(&self.current_frequency) {
            self.frequency_count.insert(self.current_frequency, 1);
        } else {
            return true;
        }
        false
    }

    // frequency just returns the current value of the calibrators
    // frequency.
    fn frequency(self) -> i32 {
        self.current_frequency
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn adds_positive_values() {
        let mut c = crate::Calibrator::new(1);
        c.update_frequency(10);
        assert_eq!(c.current_frequency, 11)
    }

    #[test]
    fn adds_negative_values() {
        let mut c = crate::Calibrator::new(1);
        c.update_frequency(-1);
        assert_eq!(c.current_frequency, 0);

        c.update_frequency(-10);
        assert_eq!(c.current_frequency, -10);
    }
}
