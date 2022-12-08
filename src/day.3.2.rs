mod day_3;

use std::fs::read_to_string;

use crate::day_3::{Groups, Priorities};

fn main() {
    let puzzle = read_to_string("./day.3.2.txt").expect("cannot read puzzle.txt");
    let groups = Groups::from(puzzle.as_str());
    let priorities = groups.priorities();
    println!("sum: {}", priorities.iter().fold(0, |acc, (_, x)| acc + x));
    println!("priorities:\n{priorities:#?}");
}
