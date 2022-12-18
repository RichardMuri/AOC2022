use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io;
use std::io::BufRead;

use common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = common::get_input_path("day1", "input.txt").unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let mut current_calories = 0;
    let mut calorie_tracker = BinaryHeap::from([Reverse(0), Reverse(0), Reverse(0)]);

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let item_calories = line?.parse::<i32>();
        match item_calories {
            Ok(item_calories) => current_calories += item_calories,
            _ => {
                let mut lowest_tracked = calorie_tracker.peek_mut().unwrap();
                if current_calories > lowest_tracked.0 {
                    *lowest_tracked = Reverse(current_calories);
                }
                current_calories = 0;
            }
        };
    }

    {
        // Cover the case where the final calorie collection is followed directly by EOF
        let mut lowest_tracked = calorie_tracker.peek_mut().unwrap();
        if current_calories > lowest_tracked.0 {
            *lowest_tracked = Reverse(current_calories);
        }
    }

    let mut total_calories : i32 = 0;

    for i in &calorie_tracker
    {
        total_calories += i.0;
    }

    println!("Final calorie tracker: {:?}", calorie_tracker);
    println!("Total calories carried: {total_calories}");

    Ok(())
}
