// read input
// for each line parse to coordinates on grid.
// for each set of coordinates set if they clash against all others.
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let iter: Vec<&str> = contents.split('\n').collect();
    let mut fabric = Fabric::from(1000, 1000);

    for &i in &iter {
        fabric.add_claim(Claim::from(i));
    }

    println!("{}", fabric.positions.len());
}

// grid holds all the state of the fabric.
struct Fabric {
    height: i32,
    width: i32,
    positions: Vec<Claim>,
}

impl Fabric {
    fn from(width: i32, height: i32) -> Fabric {
        Fabric {
            height: height,
            width: width,
            positions: vec![],
        }
    }
    fn add_claim(&mut self, claim: Claim) {
        self.positions.push(claim);
    }
}

struct Claim {
    id: String,
    left: i32,
    top: i32,
    height: i32,
    width: i32,
}

impl Claim {
    fn from(input: &str) -> Claim {
        let chunks: Vec<&str> = input.split(" ").collect();
        let (left, right) = parse_coords(chunks[2]);
        let (width, height) = parse_area(chunks[3]);
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
    let chunks: Vec<&str> = x.split(",").collect();
    (
        chunks[0].parse::<i32>().unwrap(),
        chunks[1].parse::<i32>().unwrap(),
    )
}

fn parse_area(input: &str) -> (i32, i32) {
    let chunks: Vec<&str> = input.split("x").collect();
    (
        chunks[0].parse::<i32>().unwrap(),
        chunks[1].parse::<i32>().unwrap(),
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn claim_from() {
        let c = crate::Claim::from("#1316 @ 7,377: 14x23");
        assert_eq!("1316", c.id);
        assert_eq!(7, c.left);
        assert_eq!(377, c.top);
        assert_eq!(14, c.height);
        assert_eq!(23, c.width);

        let b = crate::Claim::from("#1336 @ 202,485: 26x10");
        assert_eq!("1336", b.id);
        assert_eq!(202, b.left);
        assert_eq!(485, b.top);
        assert_eq!(26, b.height);
        assert_eq!(10, b.width);
    }
}
