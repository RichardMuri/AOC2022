use std::fs;
use std::io;
use std::io::BufRead;

struct TreeTracker {
    width: usize,
    row_height: Vec<u8>,
    column_height: u8,
    visible: Vec<bool>,
}

fn count_interior<'a, I>(trees: I, tracker: &mut TreeTracker)
where
    I: IntoIterator<Item = &'a u8>,
{
    for (i, tree) in trees.into_iter().enumerate() {
        let mut _hit = false;
        let column_index = i % tracker.width;

        // Hit leftmost outer tree
        if column_index == 0 {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // Hit rightmost outer tree
        if column_index == (tracker.width - 1) {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // Left/right comparison of interior trees
        if tree > &tracker.column_height {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // North/south comparison of interior trees
        if tree > &tracker.row_height[column_index] {
            tracker.visible[i] = true;
            tracker.row_height[column_index] = *tree;
            _hit = true;
        }

        // println!("{i} \t {column_index} {tree} {hit}");
    }
}

fn count_interior_rev(trees: &Vec<u8>, tracker: &mut TreeTracker) {
    for (i, tree) in trees.into_iter().enumerate().rev() {
        let mut _hit = false;
        let column_index = i % tracker.width;

        // Hit leftmost outer tree
        if column_index == 0 {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // Hit rightmost outer tree
        if column_index == (tracker.width - 1) {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // Left/right comparison of interior trees
        if tree > &tracker.column_height {
            tracker.visible[i] = true;
            tracker.column_height = *tree;
            _hit = true;
        }

        // North/south comparison of interior trees
        if tree > &tracker.row_height[column_index] {
            tracker.visible[i] = true;
            tracker.row_height[column_index] = *tree;
            _hit = true;
        }

        // println!("{i} \t {column_index} {tree} {hit}");
    }
}

fn nsview(trees: &Vec<u8>, width: usize, index: usize) -> (usize, usize) {
    let height = trees[index];
    let mut nview = 0usize;
    let mut sview = 0usize;

    let min_index = index % width;
    let max_index = trees.len() - (width - min_index);

    let mut i = index.wrapping_sub(width);
    while i >= min_index && i <= max_index {
        nview += 1;
        let cmp_tree = trees[i as usize];
        if height <= cmp_tree {
            break;
        }
        i = i.wrapping_sub(width);
    }

    let mut i = index + width;
    while i >= min_index && i <= max_index {
        sview += 1;
        let cmp_tree = trees[i as usize];
        if height <= cmp_tree {
            break;
        }
        i += width;
    }

    (nview, sview)
}

fn ewview(trees: &Vec<u8>, width: usize, index: usize) -> (usize, usize) {
    let height = trees[index];
    let row_min = index - (index % width);
    let row_max = row_min + width - 1;
    let mut eview = 0usize;
    let mut wview = 0usize;

    let mut i = index.wrapping_sub(1);
    while i >= row_min && i <= row_max {
        eview += 1;
        let cmp_tree = trees[i];
        if height <= cmp_tree {
            break;
        }
        i = i.wrapping_sub(1);
    }

    let mut i = index + 1;
    while i >= row_min && i <= row_max {
        wview += 1;
        let cmp_tree = trees[i];
        if height <= cmp_tree {
            break;
        }
        i += 1;
    }

    (eview, wview)
}

fn calculate_view(trees: &Vec<u8>, width: usize) -> usize {
    let mut max_view = 0usize;

    for (i, _) in trees.iter().enumerate() {

        let (nview, sview) = nsview(trees, width, i);
        let (eview, wview) = ewview(trees, width, i);

        let view = nview * sview * eview * wview;

        // println!("{nview}, {sview}, {eview}, {wview}, {view}");
        if view > max_view {
            max_view = view;
        }
    }

    max_view
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = common::Args::init();
    let filepath = common::get_input_path("day8", &args.file).unwrap();
    let file = match fs::File::open(&filepath) {
        Err(err) => panic!("Failed to open {}: {}", filepath.display(), err),
        Ok(file) => file,
    };

    println!("Reading input from file: {}", filepath.display());

    let mut lines = io::BufReader::new(file).lines();

    let first_line = lines
        .by_ref()
        .next()
        .ok_or_else(|| format!("Unable to read first line."))??;
    let row_width = first_line.len();
    // let column_height = lines.count();

    let tallest_row: Vec<_> = first_line
        .chars()
        .by_ref()
        .clone()
        .map(|c| (c as u8 - '0' as u8) as u8)
        .collect();

    let mut buffer = lines
        .flat_map(|x| {
            x.unwrap()
                .chars()
                .map(|c| (c as u8 - '0' as u8) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let lrange = (buffer.len() - row_width)..;
    let last_line: Vec<u8> = buffer.drain(lrange).collect();

    let mut tracker: TreeTracker = TreeTracker {
        width: row_width,
        row_height: tallest_row.clone(),
        column_height: 0,
        visible: vec![false; buffer.len()],
    };

    count_interior(&buffer, &mut tracker);

    // tracker.row_height = buffer.rchunks(row_width).next().unwrap().to_vec();
    tracker.row_height = last_line.clone();

    count_interior_rev(&buffer, &mut tracker);

    // We don't pass the first or last rows in for counting because they are
    // exterior visible by definition
    let total_visible: usize = tracker.visible.iter().filter(|t| **t).count() + (row_width * 2);

    println!("Found {total_visible} visible trees.");


    for i in 0..row_width{
        buffer.insert(i, tallest_row[i]);
    }
    
    buffer.extend_from_slice(&last_line[..]);
    let best_view = calculate_view(&buffer, row_width);
    println!("Best view had a score of {best_view}.");

    // Print picture of visible trees
    // let _var: () = tracker
    //     .visible
    //     .chunks(row_width)
    //     .map(|x| {
    //         x.iter()
    //             .for_each(|y| if *y { print!("1") } else { print!("0") });
    //         println!("")
    //     })
    //     .collect();

    Ok(())
}
