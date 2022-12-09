use std::fs::read_to_string;

use day_2::{Score, Strategy};

mod day_2;

fn main() {
    let puzzle = read_to_string("./day.2.txt").expect("cannot read puzzle.txt");
    let strategy = Strategy::from(puzzle.as_str());
    let strategy = strategy.cheat();
    let score = strategy.score();
    println!("{score}");
}
