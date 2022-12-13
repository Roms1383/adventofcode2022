use std::fs::read_to_string;

use day_11::{Monkeys, Myself};

mod day_11;

fn main() {
    let puzzle = read_to_string("./day.11.txt").expect("cannot read puzzle.txt");
    let mut monkeys = Monkeys::from(puzzle.as_str());
    Myself::observe(&mut monkeys, 20, true);
    monkeys.0.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let mut taker = monkeys.0.into_iter().take(2);
    let first = taker.next().unwrap();
    let second = taker.next().unwrap();
    println!("{}", first.inspected * second.inspected);
}

#[cfg(test)]
mod tests {
    use crate::day_11::{Monkeys, Myself};

    const INPUT: &'static str = "  Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn basics() {
        let mut monkeys = Monkeys::from(INPUT);
        Myself::observe(&mut monkeys, 20, true);
        assert_eq!(monkeys.0.get(0).unwrap().inspected, 101);
        assert_eq!(monkeys.0.get(1).unwrap().inspected, 95);
        assert_eq!(monkeys.0.get(2).unwrap().inspected, 7);
        assert_eq!(monkeys.0.get(3).unwrap().inspected, 105);
    }
}
