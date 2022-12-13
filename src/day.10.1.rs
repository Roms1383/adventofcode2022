use std::fs::read_to_string;

use crate::day_10::{Clock, Instructions};

mod day_10;

fn main() {
    let puzzle = read_to_string("./day.10.txt").expect("cannot read puzzle.txt");
    let instructions = Instructions::from(puzzle.as_str());

    let mut clock = Clock::new(instructions);
    clock.execute();
    println!("{}", clock.total_signals_strength());
}

#[cfg(test)]
mod tests {
    use crate::day_10::{Clock, Instructions};

    const INPUT: &'static str = "noop
addx 3
addx -5";

    const LARGER_INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn parse() {
        let instructions = Instructions::from(INPUT);
        assert_eq!(instructions.0.len(), 3);
    }

    #[test]
    fn basics() {
        let instructions = Instructions::from(LARGER_INPUT);
        assert_eq!(instructions.0.len(), 146);

        let mut clock = Clock::new(instructions);
        clock.execute();
        assert_eq!(&clock.total_signals_strength(), &13140);
    }
}
