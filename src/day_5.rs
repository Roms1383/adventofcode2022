use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Crate(char);

#[derive(Debug, Clone, PartialEq)]
pub struct Stack(VecDeque<Crate>);

#[derive(Debug)]
pub struct Move {
    steps: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        let mut stack = VecDeque::new();
        for line in v {
            if line.is_some() {
                stack.push_back(Crate::from(line.unwrap()));
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
                    }
                }
            }
            stacks.push(stack.clone());
            stack.clear();
            index += 4;
        }
        Stacks::from(stacks)
    }
}

#[allow(dead_code)]
impl Stack {
    fn get_top_crate(&self) -> Option<char> {
        for item in self.0.iter() {
            return Some(item.0);
        }
        None
    }
    fn get_top_crate_idx(&self) -> Option<usize> {
        if self.0.len() == 0 {
            return None;
        }
        Some(0)
    }
    fn get_bottom_crate_idx(&self) -> Option<usize> {
        if self.0.len() == 0 {
            return None;
        }
        Some(self.0.len() - 1)
    }
}

#[allow(dead_code)]
impl Stacks {
    fn swap_crates(&mut self, from: usize, to: usize) {
        let origin = self.0.get_mut(from).unwrap();
        let source = origin.get_top_crate_idx().expect("source");
        let moved = origin.0.remove(source).unwrap();
        drop(origin);

        let destination = self.0.get_mut(to).unwrap();
        destination.0.push_front(moved);
    }
    fn move_crates(&mut self, m: &Move) {
        let mut times = m.steps;
        while times > 0 {
            self.swap_crates(m.from - 1, m.to - 1);
            times -= 1;
        }
    }
    pub fn multiple_move_crates(&mut self, moves: Moves) {
        for m in moves.0.iter() {
            self.move_crates(m);
        }
    }
    pub fn get_top_crates(&self) -> String {
        let mut found = String::from("");
        for stack in self.0.iter() {
            if let Some(c) = stack.get_top_crate() {
                found.push(c);
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::{Moves, Stack};

    use super::{Crate, Stacks};

    const INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
  1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn convert() {
        let stacks = Stacks::from(INPUT);
        assert_eq!(stacks.0.len(), 3);
        let moves = Moves::from(INPUT);
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

    #[test]
    fn steps() {
        let mut stacks = Stacks::from(INPUT);
        let moves = Moves::from(INPUT);

        let first = moves.0.get(0).unwrap();
        stacks.move_crates(first);
        let current = stacks.0.get(0).unwrap();
        assert_eq!(*current, Stack([Crate('D'), Crate('N'), Crate('Z')].into()));
        let current = stacks.0.get(1).unwrap();
        assert_eq!(*current, Stack([Crate('C'), Crate('M')].into()));
        let current = stacks.0.get(2).unwrap();
        assert_eq!(*current, Stack([Crate('P')].into()));

        let second = moves.0.get(1).unwrap();
        stacks.move_crates(second);
        let current = stacks.0.get(0).unwrap();
        assert_eq!(*current, Stack([].into()));
        let current = stacks.0.get(1).unwrap();
        assert_eq!(*current, Stack([Crate('C'), Crate('M')].into()));
        let current = stacks.0.get(2).unwrap();
        assert_eq!(
            *current,
            Stack([Crate('Z'), Crate('N'), Crate('D'), Crate('P')].into())
        );

        let third = moves.0.get(2).unwrap();
        stacks.move_crates(third);
        let current = stacks.0.get(0).unwrap();
        assert_eq!(*current, Stack([Crate('M'), Crate('C')].into()));
        let current = stacks.0.get(1).unwrap();
        assert_eq!(*current, Stack([].into()));
        let current = stacks.0.get(2).unwrap();
        assert_eq!(
            *current,
            Stack([Crate('Z'), Crate('N'), Crate('D'), Crate('P')].into())
        );

        let fourth = moves.0.get(3).unwrap();
        stacks.move_crates(fourth);
        let current = stacks.0.get(0).unwrap();
        assert_eq!(*current, Stack([Crate('C')].into()));
        let current = stacks.0.get(1).unwrap();
        assert_eq!(*current, Stack([Crate('M')].into()));
        let current = stacks.0.get(2).unwrap();
        assert_eq!(
            *current,
            Stack([Crate('Z'), Crate('N'), Crate('D'), Crate('P')].into())
        );
    }

    #[test]
    fn swap() {
        let mut stacks = Stacks::from(INPUT);
        let moves = Moves::from(INPUT);
        stacks.multiple_move_crates(moves);
        assert_eq!(stacks.get_top_crates().as_str(), "CMZ");
    }
}
