use std::fs::read_to_string;

use day_5::{Moves, Stacks};

mod day_5;

fn main() {
    let puzzle = read_to_string("./day.5.txt").expect("cannot read puzzle.txt");
    let mut stacks = Stacks::from(puzzle.as_str());
    let moves = Moves::from(puzzle.as_str());
    stacks.multiple_move_crates(&moves);
    println!("{}", stacks.get_top_crates().as_str());
}
