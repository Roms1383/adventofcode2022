use std::fs::read_to_string;

use crate::day_1::{Calories, Supplies};

mod day_1;

fn main() {
    let puzzle = read_to_string("./day.1.txt").expect("cannot read puzzle.txt");
    let supplies = Supplies::from(puzzle.as_str());
    let calories = supplies.calories();
    println!("{calories}");
}
