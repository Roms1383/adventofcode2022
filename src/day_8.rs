#![allow(dead_code)]

use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Tree {
    size: usize,
    position: Position,
    parent: Rc<Forest>,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.position == other.position
    }
}

impl Tree {
    fn size(&self) -> usize {
        self.size
    }
    fn leftmost(&self) -> bool {
        self.position.x == 0
    }
    fn rightmost(&self) -> bool {
        self.position.x == self.parent.width
    }
    fn topmost(&self) -> bool {
        self.position.y == 0
    }
    fn bottommost(&self) -> bool {
        self.position.y == self.parent.height
    }
    fn left(&self) -> Option<&Tree> {
        if self.leftmost() {
            return None;
        }
        self.parent.find(&Position {
            x: self.position.x - 1,
            y: self.position.y,
        })
    }
    fn right(&self) -> Option<&Tree> {
        if self.rightmost() {
            return None;
        }
        self.parent.find(&Position {
            x: self.position.x + 1,
            y: self.position.y,
        })
    }
    fn up(&self) -> Option<&Tree> {
        if self.topmost() {
            return None;
        }
        self.parent.find(&Position {
            x: self.position.x,
            y: self.position.y - 1,
        })
    }
    fn down(&self) -> Option<&Tree> {
        if self.bottommost() {
            return None;
        }
        self.parent.find(&Position {
            x: self.position.x,
            y: self.position.y + 1,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Forest {
    /// <(x, y), tree>
    grid: HashMap<(usize, usize), Tree>,
    width: usize,
    height: usize,
}

impl Forest {
    fn find(&self, at: &Position) -> Option<&Tree> {
        self.grid.get(&(at.x, at.y))
    }
}

impl Default for Forest {
    fn default() -> Self {
        Self {
            grid: HashMap::new(),
            width: 0,
            height: 0,
        }
    }
}

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let mut forest = Self::default();
        forest.width = v.clone().lines().next().unwrap().len();
        forest.height = v.clone().lines().count();
        let reference = Rc::new(forest.clone());
        for (y, line) in v.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position { x, y };
                forest.grid.insert(
                    (position.x, position.y),
                    Tree {
                        size: c.to_digit(10).expect("should be a digit") as usize,
                        parent: Rc::clone(&reference),
                        position: position,
                    },
                );
            }
        }
        forest
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
        // println!("{forest:#?}");
        let first = forest.grid.get(&(0, 0)).unwrap();
        let second = forest.grid.get(&(1, 0)).unwrap();
        let somewhere = forest.grid.get(&(3, 4)).unwrap();
        println!("somewhere:\n{somewhere:#?}");
        // println!("first:\n{first:#?}");
        // println!("second:\n{second:#?}");
        // println!("first(left):\n{:#?}", first.left());
        // println!("first(right):\n{:#?}", first.right());
        // println!("second(left):\n{:#?}", second.left());
        // println!("second(right):\n{:#?}", second.right());
        assert_eq!(first.left(), None);
        assert_eq!(first.right(), Some(second));
        assert_eq!(second.left(), Some(first));
    }
}
