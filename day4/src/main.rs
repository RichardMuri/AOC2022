use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("dayX", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let lines = io::BufReader::new(file).lines();

    for line in lines{

    }

    Ok(())
}