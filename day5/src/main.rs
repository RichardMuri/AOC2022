use parse_display::{Display, FromStr};
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Display, FromStr, Debug, Clone, Copy)]
#[display("move {num} from {begin} to {end}")]
struct Move {
    num: u32,
    begin: u32,
    end: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day5", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());

    let mut moves: Vec<Move> = vec![];
    let mut stacks: Vec<VecDeque<char>> = vec![];

    for line in lines {
        // If the line starts with [ it's guaranteed to have crates
        if line.trim().starts_with('[') {
            // Skip the initial [, every crate is spaced by four characters.
            // This iterator will yield a list of all potential crates in a
            // line, or a space if a crate isn't present in that column
            let chars = line.chars().skip(1).step_by(4);
            for (stack, char) in (0..).zip(chars)
            {
                // If our Vec<Vec> is too small, allocate more space
                if stack >= stacks.len()
                {
                    stacks.resize_with(stack + 1, Default::default);
                }
                match char{
                    char if char.is_ascii_uppercase() => stacks[stack].push_front(char),
                    ' ' => (),
                    _ => todo!()
                }
            }
        } else if let Ok(m) = line.parse::<Move>() {
            moves.push(m);
        }
    }

    println!("{:?}", stacks);
    println!("{:?}", moves);

    Ok(())
}
