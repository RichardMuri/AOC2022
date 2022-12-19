use std::char;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

fn get_item_priority(item: char) -> usize {
    match item {
        'a'..='z' => (item as i32 - 'a' as i32 + 1) as usize,
        'A'..='Z' => (item as i32 - 'A' as i32 + 27) as usize,
        _ => !panic!("Unexpected item: {:?}", item),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day3", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let lines = io::BufReader::new(file).lines();

    let mut first_pocket: HashSet<char> = HashSet::new();
    let mut second_pocket: HashSet<char> = HashSet::new();

    let mut badge_tracker: HashSet<char> = HashSet::new();
    let mut elf_index: usize = 0;

    let mut total_priority: usize = 0;
    let mut badge_priority: usize = 0;

    // Read input line by line
    for line in lines {
        let s = line?;
        let nitems = s.len();
        let mut index: usize = 0;

        // Iterate over individual items in knapsack
        for c in s.chars() {
            // Place first half of items into set representing first pocket
            if index < (nitems / 2) {
                first_pocket.insert(c);
            } else {
                second_pocket.insert(c);
            }
            index += 1;
        }

        // Find the intersection of pocket sets and add that item score to our total
        let intersection: Vec<_> = first_pocket.intersection(&second_pocket).collect();
        let item = intersection[0].clone();
        let item_score = get_item_priority(item);
        total_priority += item_score;
        // println!("{item} {item_score} {total_priority}");

        // The union of pockets represents the set of items the current "elf" (line) possesses
        let current_elf: HashSet<_> = first_pocket.union(&second_pocket).copied().collect();
        elf_index += 1;

        // Elves are grouped by 3 that share exactly one item
        if elf_index == 3 {
            elf_index = 0;
            // println!(
            //     "Badge tracker: {:?}\nCurrent elf: {:?}",
            //     badge_tracker, current_elf
            // );

            // Elf badge is represented by the intersection of current elf, and elf 1 + 2
            let badge_intersect: Vec<_> = badge_tracker.intersection(&current_elf).collect();
            let badge_item = badge_intersect[0].clone();
            let badge_item_score = get_item_priority(badge_item);
            badge_priority += badge_item_score;
            // println!("{badge_item} {badge_item_score} {badge_priority}");
            badge_tracker.clear();
        } else if elf_index == 2 {
            // Copy the intersection of elf 1 and elf 2 into the badge set
            badge_tracker = badge_tracker.intersection(&current_elf).copied().collect();
        } else {
            // First elf in group, copy all of their items into the badge set
            badge_tracker.extend(current_elf);
        }

        first_pocket.clear();
        second_pocket.clear();
    }

    println!("The total priority of duplicated items is {total_priority}.\nThe total priority of badges is {badge_priority}");

    Ok(())
}
