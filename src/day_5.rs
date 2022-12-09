use std::ops::Index;

use regex::Regex;

pub struct Crate(char);

pub struct Stack(Vec<Option<Crate>>);

pub struct Move {
    steps: usize,
    from: usize,
    to: usize,
}

pub struct Moves(Vec<Move>);

impl From<&str> for Moves {
    fn from(v: &str) -> Self {
        let mut moves = vec![];
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for cap in re.captures_iter(v) {
            moves.push(Move {
                steps: cap[1].parse().expect("valid digit for steps"),
                from: cap[2].parse().expect("valid digit for from"),
                to: cap[3].parse().expect("valid digit for to"),
            });
        }
        Moves(moves)
    }
}

pub struct Stacks(Vec<Stack>);

impl From<&str> for Crate {
    fn from(v: &str) -> Self {
        let mut chars = v.chars();
        chars.next();
        Self(chars.next().expect("a char"))
    }
}

impl From<&Vec<Option<&str>>> for Stack {
    fn from(v: &Vec<Option<&str>>) -> Self {
        let mut stack = vec![];
        for line in v {
            if line.is_none() {
                stack.push(None);
            } else {
                stack.push(Some(Crate::from(line.unwrap())));
            }
        }
        Stack(stack)
    }
}

impl From<Vec<Vec<Option<&str>>>> for Stacks {
    fn from(v: Vec<Vec<Option<&str>>>) -> Self {
        let mut stacks = vec![];
        for lines in v.iter() {
            stacks.push(Stack::from(lines));
        }
        Stacks(stacks)
    }
}

// could have been done simpler by jumping from index to index ...
impl From<&str> for Stacks {
    fn from(v: &str) -> Self {
        let mut acc = vec![];
        #[allow(unused_assignments)]
        let mut till = 0;
        let mut max = 0;
        for line in v.lines() {
            if !line.contains(']') {
                break;
            }
            till = line.rfind(']').unwrap() + 1;
            acc.push(&line[..till]);
            if line.len() > max {
                max = line.len();
            }
        }
        let mut stack = vec![];
        let mut stacks = vec![];
        let mut index = 0;
        while index < max {
            for line in acc.iter() {
                if line.len() >= index + 3 {
                    let part = &line[index..index + 3];
                    if !part.trim().is_empty() {
                        stack.push(Some(part));
                    } else {
                        stack.push(None);
                    }
                } else {
                    stack.push(None);
                }
            }
            stacks.push(stack.clone());
            stack.clear();
            index += 4;
        }
        Stacks::from(stacks)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::Moves;

    use super::Stacks;

    #[test]
    fn convert() {
        let s = "    [D]    
[N] [C]    
[Z] [M] [P]
  1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let stacks = Stacks::from(s);
        assert_eq!(stacks.0.len(), 3);
        let moves = Moves::from(s);
        assert_eq!(moves.0.len(), 4);
        let current = moves.0.get(0).unwrap();
        assert_eq!(current.steps, 1);
        assert_eq!(current.from, 2);
        assert_eq!(current.to, 1);
        let current = moves.0.get(1).unwrap();
        assert_eq!(current.steps, 3);
        assert_eq!(current.from, 1);
        assert_eq!(current.to, 3);
        let current = moves.0.get(2).unwrap();
        assert_eq!(current.steps, 2);
        assert_eq!(current.from, 2);
        assert_eq!(current.to, 1);
        let current = moves.0.get(3).unwrap();
        assert_eq!(current.steps, 1);
        assert_eq!(current.from, 1);
        assert_eq!(current.to, 2);
    }
}
