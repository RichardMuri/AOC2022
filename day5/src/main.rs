use parse_display::{Display, FromStr};
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Display, FromStr, Debug, Clone, Copy)]
#[display("move {num} from {begin} to {end}")]
struct Move {
    num: usize,
    begin: usize,
    end: usize,
}

fn operate(moves : &Vec<Move>, crates : &mut Vec<VecDeque<char>>) -> String
{
    // Follow each move instruction, moving one crate from the top of one stack
    // to the other at a time
    for m in moves{
        for _ in 0..m.num{
            let start = m.begin - 1;
            let dest = m.end - 1;
            let c = crates[start].pop_back();
            crates[dest].push_back(c.unwrap());
        }
    }

    // Collect the back character from each vector into a string
    crates[..]
    .iter()
    .map(|s| s.back().unwrap())
    .collect()
}

fn operate_improved(moves : &Vec<Move>, crates : &mut Vec<VecDeque<char>>) -> String
{
    // Follow each move instruction, moving num crates at a time from the top of
    // one stack to the other
    for m in moves{
        let start = m.begin - 1;
        let dest = m.end - 1;
        let crate_ref = &mut crates[start];
        let stack_range = (crate_ref.len() - m.num)..crate_ref.len();

        // Drain a range of crates
        let crate_stack = crate_ref.drain(stack_range).collect::<Vec<_>>();
        // Move the range onto a new stack
        crates[dest].extend(crate_stack);
    }

    // Collect the back character from each vector into a string
    crates[..]
    .iter()
    .map(|s| s.back().unwrap())
    .collect()
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
    // println!("{:?}", stacks);
    // println!("{:?}", moves);
    println!("{}", operate(&moves, &mut stacks.clone()));
    println!("{}", operate_improved(&moves, &mut stacks));
    // println!("{:?}", stacks);
    Ok(())
}
