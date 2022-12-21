use std::fs;
use std::io::Read;

fn find_window(masks: &Vec<u32>, size: usize) -> Result<usize, String> {
    let mut accumulate = 0u32;
    for (i, mask) in masks.iter().enumerate() {
        accumulate ^= mask;

        if i >= size {
            accumulate ^= masks[i - size];
            if accumulate.count_ones() == size as u32 {
                return Ok(i + 1 as usize);
            }
        }
    }
    Err("Couldn't find sequence for specified window".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day6", &args.file).unwrap();
    let mut file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;

    let bitmasks: Vec<u32> = buffer
        .into_iter()
        .filter(|c| (*c as char).is_ascii_lowercase())
        .map(|c| 1u32 << (c as u8 - 'a' as u8))
        .collect();

    println!("{}", find_window(&bitmasks, 4)?);
    println!("{}", find_window(&bitmasks, 14)?);

    Ok(())
}
