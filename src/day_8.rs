#![allow(dead_code)]

use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Position(usize, usize);

#[derive(Debug, Clone)]
pub struct Tree {
    size: usize,
    surroundings: Rc<HashMap<Position, Tree>>,
}

impl Tree {
    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct Forest(HashMap<Position, Tree>);

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let mut inner: HashMap<Position, Tree> = HashMap::new();
        let reference = Rc::new(inner.clone());
        for (y, line) in v.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                inner.insert(
                    Position(x, y),
                    Tree {
                        size: c.to_digit(10).expect("should be a digit") as usize,
                        surroundings: Rc::clone(&reference),
                    },
                );
            }
        }
        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn parse() {
        let forest = Forest::from(INPUT);
        println!("{forest:#?}");
        assert!(false);
    }
}
