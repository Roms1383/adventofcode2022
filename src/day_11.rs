#![allow(dead_code, unused_variables)]

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    starting_items: StartingItems,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
pub struct Monkeys(Vec<Monkey>);

#[derive(Debug)]
pub struct Item(usize);

#[derive(Debug)]
pub struct Worry {
    level: usize,
    item: Item,
}

#[derive(Debug)]
pub struct StartingItems(Vec<Item>);

#[derive(Debug)]
pub enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
pub enum Operand {
    Old,
    Digit(usize),
}

#[derive(Debug)]
pub struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug)]
pub struct Decision {
    recipient: usize,
}

#[derive(Debug)]
pub struct Test {
    divisible: usize,
    if_true: Decision,
    if_false: Decision,
}

impl Monkey {
    fn inspect(&self, worry: &Worry) {
        todo!()
    }
}

impl From<&str> for StartingItems {
    fn from(v: &str) -> Self {
        Self(
            v["Starting items: ".len()..]
                .split(", ")
                .map(|x| Item(x.parse().expect("should be a digit")))
                .collect(),
        )
    }
}

impl From<&str> for Operator {
    fn from(v: &str) -> Self {
        match v {
            "+" => Operator::Plus,
            "*" => Operator::Times,
            _ => panic!("should not happen"),
        }
    }
}

impl From<&str> for Operand {
    fn from(v: &str) -> Self {
        if v == "old" {
            return Self::Old;
        }
        Self::Digit(v.parse().expect("should be a digit"))
    }
}

impl From<&str> for Operation {
    fn from(v: &str) -> Self {
        let parts: Vec<&str> = v.split(' ').collect();
        assert_eq!(parts.len(), 6);
        let operator: Operator = Operator::from(*parts.get(4).expect("operator"));
        let operand = Operand::from(*parts.get(5).expect("operand"));
        Self { operator, operand }
    }
}

impl From<&str> for Decision {
    fn from(v: &str) -> Self {
        let recipient = v[v.rfind(' ').expect("last space") + 1..]
            .parse::<usize>()
            .expect("should be a digit");
        Self { recipient }
    }
}

impl From<&str> for Test {
    fn from(v: &str) -> Self {
        let lines: Vec<&str> = v.lines().collect();
        assert_eq!(lines.len(), 3);
        let line = lines.get(0).expect("divisible");
        let divisible = *&line[line.rfind(' ').expect("last space") + 1..]
            .parse::<usize>()
            .expect("should be a digit");
        let line = lines.get(1).expect("if true");
        let if_true = Decision::from(*line);
        let line = lines.get(2).expect("if false");
        let if_false = Decision::from(*line);
        Self {
            divisible,
            if_true,
            if_false,
        }
    }
}

impl From<Vec<&str>> for Monkey {
    fn from(lines: Vec<&str>) -> Self {
        println!("{:#?}\n", lines);
        assert_eq!(lines.len(), 6);
        let line = lines.get(0).expect("monkey");
        let idx = line[line.rfind(' ').unwrap() + 1..line.len() - 1]
            .parse::<usize>()
            .expect("should be a digit");
        let line = lines.get(1).expect("starting items list");
        let starting_items = StartingItems::from(*line);
        let line = lines.get(2).expect("operation");
        let operation = Operation::from(*line);
        let test = Test::from(lines[3..].join("\n").as_str());
        Self {
            idx,
            starting_items,
            operation,
            test,
        }
    }
}

impl From<&str> for Monkeys {
    fn from(v: &str) -> Self {
        let mut monkeys = vec![];
        let mut acc = Vec::with_capacity(6);
        for (i, line) in v.lines().map(|x| x.trim()).enumerate() {
            if !line.is_empty() {
                acc.push(line);
            } else {
                monkeys.push(Monkey::from(acc.clone()));
                acc.clear();
            }
        }
        monkeys.push(Monkey::from(acc.clone()));
        Self(monkeys)
    }
}

#[cfg(test)]
mod tests {
    use super::Monkeys;

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
    fn parse() {
        let monkeys = Monkeys::from(INPUT);
        assert_eq!(monkeys.0.len(), 4);
    }
}
