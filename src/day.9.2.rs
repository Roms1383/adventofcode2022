use std::fs::read_to_string;

use day_9::{Manager, Mode, Motions};

mod day_9;

fn main() {
    let puzzle = read_to_string("./day.9.txt").expect("cannot read puzzle.txt");
    let motions = Motions::from(puzzle.as_str());
    let mut manager = Manager::new(Mode::Snake);
    manager.do_motions(&motions);
    println!("{}", manager.total_tail_visited());
}
