use std::fs::read_to_string;

use day_4::Pairs;

mod day_4;

fn main() {
    let puzzle = read_to_string("./day.4.txt").expect("cannot read puzzle.txt");
    let pairs = Pairs::from(puzzle.as_str());
    let pairs = pairs.overlapping();
    println!("overlapping assignments: {}", pairs.0.len());
}
