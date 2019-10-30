use std::fs;

fn main() {
    let mut calibrator = Calibrator::new();
    calibrator.read_input("input.txt");
    println!("{}", calibrator.frequency());
}

struct Calibrator {
    current_frequency: i32,
}

impl Calibrator {
    fn new() -> Calibrator {
        Calibrator {
            current_frequency: 0,
        }
    }
    // read_input will take a filename and read the contents
    // into the calibrator.
    fn read_input(&mut self, file: &str) {
        let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
        let iter: Vec<&str> = contents.split('\n').collect();
        for i in &iter {
            self.update_frequency(i.parse::<i32>().unwrap());
        }
    }
    // update_frequency adds a value to the current frequency.
    fn update_frequency(&mut self, value: i32) {
        self.current_frequency += value;
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
        let mut c = crate::Calibrator {
            current_frequency: 1,
        };
        c.update_frequency(10);
        assert_eq!(c.current_frequency, 11)
    }

    #[test]
    fn adds_negative_values() {
        let mut c = crate::Calibrator {
            current_frequency: 1,
        };
        c.update_frequency(-1);
        assert_eq!(c.current_frequency, 0);

        c.update_frequency(-10);
        assert_eq!(c.current_frequency, -10);
    }
}
