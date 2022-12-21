use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::BufRead;

mod node;
use node::Node;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day7", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let mut lines = io::BufReader::new(file)
    .lines()
    .skip(1)
    .map(Result::unwrap)
    .peekable();

    let mut root = Node::Dir(BTreeMap::new());
    root.traverse(&mut lines)?;
    let mut sizes: Vec<usize> = vec![];
    let root_size = root.calculate_size(&mut sizes);
    // println!("{:?}", root);

    const MAX_SIZE : usize = 100_000;
    const DISK : usize = 70000000;
    const REQUIRED_SPACE : usize = 30000000;

    let total_size :usize = sizes
    .iter()
    .filter(|x| **x <= MAX_SIZE)
    .sum();
    println!("Total size of all directories under 100,000 is {}.", total_size);

    let available_space = DISK - root_size;
    let min_free = REQUIRED_SPACE - available_space;

    let deleted_size = sizes
    .iter()
    .filter(|x| **x >= min_free)
    .min()
    .unwrap();

    println!("Deleting directory of size {}.", deleted_size);



    Ok(())
}