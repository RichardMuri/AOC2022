use std::{fs};
use std::io;
use std::io::BufRead;
use std::str::FromStr;

mod rps;
use rps::{RPS, RpsResult};
use common;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day2", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;
    let mut total_alternate = 0;
    for line in lines {
        let value = line?;
        let mut words = value.split_whitespace();
        // let rps : Vec<RPS> = words.map(|x| RPS::from_str(&x).unwrap()).collect();

        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();

        let me = RPS::from_str(second_word).unwrap();
        let opponent = RPS::from_str(first_word).unwrap();
        let game_result = RpsResult::from_str(second_word).unwrap();

        total_score += RPS::play(&me, &opponent);
        total_alternate += RPS::alternate_score(&opponent, &game_result);
        
        let rps = (me, opponent);
        
        println!("{}, {:?}, {}, {}", value, rps, total_score, total_alternate);
    }

    Ok(())
}
