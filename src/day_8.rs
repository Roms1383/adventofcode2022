#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    width: usize,
    height: usize,
}

impl From<(usize, usize)> for Position {
    fn from(v: (usize, usize)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl Position {
    fn edge(&self, toward: Direction, bounds: Bounds) -> bool {
        match toward {
            Direction::Left => self.x == 0,
            Direction::Right => self.x == bounds.width - 1,
            Direction::Up => self.y == 0,
            Direction::Down => self.y == bounds.height - 1,
        }
    }
    fn next(&self, toward: Direction) -> Position {
        match toward {
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size(usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Forest {
    grid: HashMap<Position, Size>,
    bounds: Bounds,
}

impl Forest {
    fn edge(&self, at: Position, toward: Direction) -> bool {
        at.edge(toward, self.bounds)
    }
    fn neighbor(&self, at: Position, toward: Direction) -> Option<&Size> {
        if self.edge(at, toward) {
            return None;
        }
        self.grid.get(&at.next(toward))
    }
    fn visible(&self, at: Position, toward: Direction) -> bool {
        if self.edge(at, toward) {
            return true;
        }
        todo!()
    }
    fn neighbors(&self, at: Position, toward: Direction) -> Vec<&Size> {
        if self.edge(at, toward) {
            return Vec::with_capacity(0);
        }
        let mut neighbors = vec![];
        let mut next = at.clone();
        loop {
            next = next.next(toward);
            neighbors.push(self.grid.get(&next).unwrap());
            if next.edge(toward, self.bounds) {
                break;
            }
        }
        neighbors
    }
}

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let mut forest = Self {
            grid: HashMap::new(),
            bounds: Bounds {
                width: v.clone().lines().next().unwrap().len(),
                height: v.clone().lines().count(),
            },
        };
        for (y, line) in v.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                forest.grid.insert(
                    Position { x, y },
                    Size(c.to_digit(10).expect("should be a digit") as usize),
                );
            }
        }
        forest
    }
}
#[cfg(test)]
mod tests {
    use super::{Direction, Forest};

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn parse() {
        let forest = Forest::from(INPUT);
        println!("{forest:#?}");
        assert_eq!(forest.bounds.width, 5);
        assert_eq!(forest.bounds.height, 5);
    }

    #[test]
    fn neighbors() {
        let forest = Forest::from(INPUT);
        let at = (0, 0);
        let neighbors = forest.neighbors(at.into(), Direction::Left);
        assert_eq!(neighbors.len(), 0);
        let at = (1, 0);
        let neighbors = forest.neighbors(at.into(), Direction::Left);
        assert_eq!(neighbors.len(), 1);
        let at = (1, 0);
        let neighbors = forest.neighbors(at.into(), Direction::Right);
        assert_eq!(neighbors.len(), 3);
        let at = (0, 3);
        let neighbors = forest.neighbors(at.into(), Direction::Up);
        assert_eq!(neighbors.len(), 3);
        let at = (0, 3);
        let neighbors = forest.neighbors(at.into(), Direction::Down);
        assert_eq!(neighbors.len(), 1);
    }
}
