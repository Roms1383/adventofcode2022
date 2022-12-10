#![allow(dead_code)]

use std::collections::{hash_map::Keys, HashMap};

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
    fn any_edge(&self, bounds: Bounds) -> bool {
        self.x == 0 || self.x == bounds.width - 1 || self.y == 0 || self.y == bounds.height - 1
    }
    fn edge(&self, toward: Direction, bounds: Bounds) -> bool {
        match toward {
            Direction::Left => self.x == 0,
            Direction::Right => self.x == bounds.width - 1,
            Direction::Top => self.y == 0,
            Direction::Bottom => self.y == bounds.height - 1,
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
            Direction::Top => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Bottom => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size(usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Forest {
    grid: HashMap<Position, Size>,
    bounds: Bounds,
}

impl Forest {
    pub fn positions(&self) -> Keys<Position, Size> {
        self.grid.keys()
    }
    fn edge(&self, at: Position, toward: Direction) -> bool {
        at.edge(toward, self.bounds)
    }
    fn neighbor(&self, at: Position, toward: Direction) -> Option<&Size> {
        if self.edge(at, toward) {
            return None;
        }
        self.grid.get(&at.next(toward))
    }
    fn size(&self, at: Position) -> &Size {
        self.grid.get(&at).as_ref().expect("no tree at position")
    }
    fn visible(&self, at: Position, toward: Direction) -> bool {
        if at.any_edge(self.bounds) {
            return true;
        }
        let neighbors = self.neighbors(at, toward);
        let size = self.size(at);
        neighbors.iter().all(|x| *x < size)
    }
    pub fn visible_from_any_direction(&self, at: Position) -> bool {
        if self.visible(at, Direction::Top) {
            return true;
        }
        if self.visible(at, Direction::Bottom) {
            return true;
        }
        if self.visible(at, Direction::Left) {
            return true;
        }
        if self.visible(at, Direction::Right) {
            return true;
        }
        false
    }
    fn blocked_at(&self, at: Position, toward: Direction) -> usize {
        let size = self.size(at);
        let mut neighbors = vec![];
        let mut next = at.clone();
        let mut neighbor_size: &Size;
        loop {
            next = next.next(toward);
            neighbor_size = self.grid.get(&next).unwrap();
            neighbors.push(self.grid.get(&next).unwrap());
            if neighbor_size >= size || next.edge(toward, self.bounds) {
                break;
            }
        }
        neighbors.len()
    }
    fn scenic_score(&self, at: Position) -> usize {
        self.blocked_at(at, Direction::Top)
            * self.blocked_at(at, Direction::Left)
            * self.blocked_at(at, Direction::Right)
            * self.blocked_at(at, Direction::Bottom)
    }
    pub fn highest_scenic_score(&self) -> usize {
        self.positions()
            .filter(|x| !x.any_edge(self.bounds))
            .map(|x| self.scenic_score(*x))
            .max()
            .expect("find max scenic score")
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
        let neighbors = forest.neighbors(at.into(), Direction::Top);
        assert_eq!(neighbors.len(), 3);
        let at = (0, 3);
        let neighbors = forest.neighbors(at.into(), Direction::Bottom);
        assert_eq!(neighbors.len(), 1);
    }

    #[test]
    fn visibility() {
        let forest = Forest::from(INPUT);
        let at = (0, 0);
        assert!(forest.visible(at.into(), Direction::Left));
        assert!(forest.visible(at.into(), Direction::Right));
        assert!(forest.visible(at.into(), Direction::Top));
        assert!(forest.visible(at.into(), Direction::Bottom));

        let at = (0, 4);
        assert!(forest.visible(at.into(), Direction::Left));
        assert!(forest.visible(at.into(), Direction::Right));
        assert!(forest.visible(at.into(), Direction::Top));
        assert!(forest.visible(at.into(), Direction::Bottom));

        // top-left 5
        let at = (1, 1);
        assert!(forest.visible(at.into(), Direction::Left));
        assert!(forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));

        // top-middle 5
        let at = (2, 1);
        assert!(forest.visible(at.into(), Direction::Top));
        assert!(forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // top-right 1
        let at = (3, 1);
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // left-middle 5
        let at = (1, 2);
        assert!(forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // center 3
        let at = (2, 2);
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // right-middle 3
        let at = (3, 2);
        assert!(forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // bottom-middle 5
        let at = (2, 3);
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(forest.visible(at.into(), Direction::Bottom));
        assert!(forest.visible(at.into(), Direction::Left));

        // bottom-left 3
        let at = (1, 3);
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));

        // bottom-right 4
        let at = (3, 3);
        assert!(!forest.visible(at.into(), Direction::Top));
        assert!(!forest.visible(at.into(), Direction::Right));
        assert!(!forest.visible(at.into(), Direction::Bottom));
        assert!(!forest.visible(at.into(), Direction::Left));
    }

    #[test]
    fn visibility_from_any_direction() {
        let forest = Forest::from(INPUT);
        let mut visible_ones = 0;
        for key in forest.grid.keys() {
            if forest.visible_from_any_direction(*key) {
                visible_ones += 1;
            }
        }
        assert_eq!(visible_ones, 21);
    }

    #[test]
    fn scenic_score() {
        let forest = Forest::from(INPUT);
        let at = (2, 3);
        assert_eq!(forest.blocked_at(at.into(), Direction::Top), 2);
        assert_eq!(forest.blocked_at(at.into(), Direction::Left), 2);
        assert_eq!(forest.blocked_at(at.into(), Direction::Bottom), 1);
        assert_eq!(forest.blocked_at(at.into(), Direction::Right), 2);
        assert_eq!(forest.scenic_score(at.into()), 8);
    }
}
