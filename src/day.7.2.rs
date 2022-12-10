use std::fs::read_to_string;

use day_7::{FileSystem, StdOut};

mod day_7;

fn main() {
    let puzzle = read_to_string("./day.7.txt").expect("cannot read puzzle.txt");
    let stdout = StdOut::from(puzzle.as_str());
    let fs = FileSystem::from(stdout);
    let total = fs.smallest_of_the_biggest(70000000, 30000000);
    println!("{}", total.unwrap().0);
}
