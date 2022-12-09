use std::fs::read_to_string;

use day_6::start_at;

mod day_6;

fn main() {
    let puzzle = read_to_string("./day.6.txt").expect("cannot read puzzle.txt");
    println!("{}", start_at(puzzle.as_str(), 14));
}
