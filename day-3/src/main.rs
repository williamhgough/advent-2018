// read input
// for each line parse to coordinates on grid.
// for each set of coordinates set if they clash against all others.
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let iter: Vec<&str> = contents.split('\n').collect();
    let mut fabric = Fabric::new();

    println!("# of claims loaded: {}", iter.len());

    for &i in &iter {
        fabric.add_claim(Claim::from(i));
    }

    println!("total claimed squares: {}", fabric.total_squares());

    println!("clash count: {}", fabric.calculate_clashes());

    if let Some(id) = fabric.find_unconflicted_claim() {
        println!("unconflicting ID: {}", id);
    }
}

// grid holds all the state of the fabric.
struct Fabric {
    positions: HashMap<(i32, i32), Vec<Claim>>,
    claims: Vec<Claim>,
}

impl Fabric {
    fn new() -> Fabric {
        Fabric {
            positions: HashMap::new(),
            claims: vec![],
        }
    }
    fn add_claim(&mut self, claim: Claim) {
        // ensure we record the individual claim
        self.claims.push(claim.clone());
        // record each square inch of the claim.
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                if x > 999 || y > 999 {
                    continue;
                }
                match self.positions.entry((x, y)) {
                    Vacant(entry) => entry.insert(vec![claim.clone()]),
                    Occupied(entry) => {
                        let v = entry.into_mut();
                        v.push(claim.clone());
                        v
                    }
                };
            }
        }
    }

    fn calculate_clashes(&self) -> usize {
        self.positions
            .values()
            .filter(|claims| claims.len() as i32 >= 2)
            .count()
    }

    fn check_overlap(&self, claim: &Claim) -> bool {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                match self.positions.get(&(x, y)) {
                    Some(entry) => {
                        if entry.len() > 1 {
                            return true;
                        }
                    }
                    None => {}
                }
            }
        }
        false
    }

    fn find_unconflicted_claim(&mut self) -> Option<String> {
        for c in self.claims.iter() {
            if !self.check_overlap(&c) {
                let x = c.clone().id;
                return Some(x);
            }
        }

        None
    }

    fn total_squares(&self) -> usize {
        self.positions.iter().count()
    }
}

#[derive(Clone)]
struct Claim {
    id: String,
    left: i32,
    top: i32,
    height: i32,
    width: i32,
}

impl Claim {
    /// from produces a new claim from an input
    /// example:
    ///     Claim::from("#1 @ 1,3: 4x4")
    /// Claim {
    ///     id: "1",
    ///     left: 1,
    ///     top: 3,
    ///     width: 4,
    ///     height: 4,
    /// }
    fn from(input: &str) -> Claim {
        let chunks: Vec<&str> = input.split(" ").collect();
        let (left, right) = parse_coords(chunks[2]);
        let (width, height) = parse_chunks(chunks[3], "x");
        Claim {
            id: parse_id(chunks[0]),
            left: left,
            top: right,
            height: height,
            width: width,
        }
    }
}

fn parse_id(input: &str) -> String {
    input.replace("#", "")
}

fn parse_coords(input: &str) -> (i32, i32) {
    let x = input.trim_end_matches(":");
    parse_chunks(x, ",")
}

fn parse_chunks(input: &str, sep: &str) -> (i32, i32) {
    let chunks: Vec<&str> = input.split(sep).collect();
    (
        chunks[0].parse::<i32>().unwrap(),
        chunks[1].parse::<i32>().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use crate::Claim;
    use crate::Fabric;
    #[test]
    fn claim_from() {
        let c = Claim::from("#1316 @ 7,377: 14x23");
        assert_eq!("1316", c.id);
        assert_eq!(7, c.left);
        assert_eq!(377, c.top);
        assert_eq!(14, c.width);
        assert_eq!(23, c.height);

        let b = Claim::from("#1336 @ 202,485: 26x10");
        assert_eq!("1336", b.id);
        assert_eq!(202, b.left);
        assert_eq!(485, b.top);
        assert_eq!(26, b.width);
        assert_eq!(10, b.height);
    }

    #[test]
    fn calculate_clashes() {
        let mut fabric = Fabric::new();
        let a = Claim::from("#1 @ 1,3: 4x4");
        let b = Claim::from("#2 @ 3,1: 4x4");
        let c = Claim::from("#3 @ 5,5: 2x2");
        fabric.add_claim(a);
        fabric.add_claim(b);
        fabric.add_claim(c);
        println!("{}", fabric.calculate_clashes());

        assert_eq!(4, fabric.calculate_clashes());
    }
}
