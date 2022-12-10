use std::fs::read_to_string;

use crate::day_8::Forest;

mod day_8;

fn main() {
    let puzzle = read_to_string("./day.8.txt").expect("cannot read puzzle.txt");
    let forest = Forest::from(puzzle.as_str());
    let mut visible_ones = 0;
    for key in forest.positions() {
        if forest.visible_from_any_direction(*key) {
            visible_ones += 1;
        }
    }
    println!("{visible_ones}");
}
