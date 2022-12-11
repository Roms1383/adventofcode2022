use std::fs::read_to_string;

use crate::day_9::types::{Motions, Knots};

mod day_9;

fn main() {
    let puzzle = read_to_string("./day.9.txt").expect("cannot read puzzle.txt");
    let motions = Motions::from(puzzle.as_str());
    let mut knots: Knots<2> = Knots::default();
    knots.do_motions(&motions);
    println!("{}", knots.total_tail_visited());
}
