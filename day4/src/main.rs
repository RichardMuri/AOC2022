use std::fs;
use std::io;
use std::io::BufRead;

use parse_display::{Display, FromStr};

#[derive(Copy, Clone, Display, FromStr, Debug)]
#[display("{min}-{max}")]
struct Section {
    min: u64,
    max: u64,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Section) -> bool {
        (self.min <= other.min && self.max >= other.min) ||
        (self.min <= other.max && self.max >= other.min)
    }
}

#[derive(Display, FromStr, Debug)]
#[display("{a},{b}")]
struct Pair {
    a: Section,
    b: Section,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day4", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let lines = io::BufReader::new(file).lines();

    let pairings = lines
        .map(|line| line.unwrap().parse::<Pair>())
        .collect::<Result<Vec<Pair>, _>>()?;

    // println!("{:?}", pairings);

    let contains = pairings
        .iter()
        .filter(|pairing| pairing.a.contains(&pairing.b) || pairing.b.contains(&pairing.a))
        .count();

    println!("{contains} pairings fully contain each other.");

    let overlaps = pairings
    .iter()
    .filter(|pairing| pairing.a.overlaps(&pairing.b))
    .count();

    println!("{overlaps} pairings overlap with each other.");

    // for line in lines{
    // }

    Ok(())
}
