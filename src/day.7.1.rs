use std::fs::read_to_string;

use day_7::{FileSystem, StdOut};

mod day_7;

fn main() {
    let puzzle = read_to_string("./day.7.txt").expect("cannot read puzzle.txt");
    println!("---\n{puzzle}\n");
    let stdout = StdOut::from(puzzle.as_str());
    println!(">>>\n{stdout:#?}\n");
    let fs = FileSystem::from(stdout);
    println!("+++\n{fs:#?}\n");
    let total = fs.sum_lightweight_dirs(100_000);
    println!("{total}");
}
