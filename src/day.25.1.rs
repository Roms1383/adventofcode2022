//! ❌ puzzle solved with extra help from [Unfold and table based testing | Advent of Code 2022 Day 25](https://www.youtube.com/watch?v=ziywvUTGxms)
//! (factorisation at 26:23)

use std::fs::read_to_string;

use day_25::{SnafuNumber, SnafuNumbers};

mod day_25;

fn main() {
    let puzzle = read_to_string("./day.25.txt").expect("cannot read puzzle.txt");
    let snafus = SnafuNumbers::from(puzzle.as_str());
    let sum = snafus.sum();
    let snafu = SnafuNumber::from(sum);
    println!("{}", snafu.to_string());
}
