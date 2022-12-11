use std::fs::read_to_string;

use crate::day_9::types::{Knots, Motions};

mod day_9;

fn main() {
    let puzzle = read_to_string("./day.9.txt").expect("cannot read puzzle.txt");
    let motions = Motions::from(puzzle.as_str());
    let mut knots: Knots<2> = Knots::default();
    knots.do_motions(&motions);
    println!("{}", knots.total_tail_visited());
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use crate::day_9::types::{Direction, Knots, Motion, Motions};

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn duo() {
        let motions = Motions::from(INPUT);
        let mut knots: Knots<2> = Knots::default();
        knots.do_motions(&motions);
        assert_eq!(knots.visited.len(), 13);
    }
}
