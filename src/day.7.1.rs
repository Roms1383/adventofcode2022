use std::fs::read_to_string;

use day_7::{FileSystem, StdOut};

mod day_7;

fn main() {
    let puzzle = read_to_string("./day.7.txt").expect("cannot read puzzle.txt");
    let stdout = StdOut::from(puzzle.as_str());
    println!("{stdout:#?}");
    let fs = FileSystem::from(stdout);
    let total = fs.sum_lightweight_dirs(100_000);
    println!("{total}");
}
