mod day_3;
use day_3::{Priorities, Rucksacks};

use std::fs::read_to_string;

fn main() {
    let puzzle = read_to_string("./day.3.txt").expect("cannot read puzzle.txt");
    let rucksacks = Rucksacks::from(puzzle.as_str());
    let priorities = rucksacks.priorities();
    println!("sum: {}", priorities.iter().fold(0, |acc, (_, x)| acc + x));
    println!("priorities:\n{priorities:#?}");
}
