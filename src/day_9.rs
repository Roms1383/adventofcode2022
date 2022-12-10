#![allow(dead_code)]

pub enum Axis {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Convolution {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Motion {
    fn axis(&self) -> Axis {
        match self.direction {
            Direction::Right | Direction::Left => Axis::Row,
            Direction::Up | Direction::Down => Axis::Column,
        }
    }
}

impl From<Direction> for Convolution {
    fn from(v: Direction) -> Self {
        match v {
            Direction::Up => Convolution::Up,
            Direction::Down => Convolution::Down,
            Direction::Left => Convolution::Left,
            Direction::Right => Convolution::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    steps: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
pub struct Head(Position);

#[derive(Debug, Clone, Copy)]
pub struct Tail(Position);

pub struct Manager {
    head: Head,
    tail: Tail,
    visited: Vec<Position>,
}

impl Manager {
    fn do_motion(&mut self, motion: &Motion) {
        for step in 0..motion.steps {
            self.head.0 = self.head.as_ref().next(&motion.direction.into());
            if !self.tail.0.touching(&self.head.0) {
                let convolution = if self.tail.0.aligned(&self.head.0, &motion.axis()) {
                    motion.direction.into()
                } else {
                    // diagonal motion
                    todo!()
                };
                let position = self.tail.as_ref().next(&convolution);
                self.tail.0 = position;
                self.record_tail_visited(&position);
            }
        }
    }
    fn record_tail_visited(&mut self, at: &Position) {
        if !self.visited.contains(at) {
            self.visited.push(at.clone());
        }
    }
}

#[derive(Debug)]
pub struct Motions(Vec<Motion>);

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unknown direction"),
        }
    }
}

impl From<&str> for Motion {
    fn from(v: &str) -> Self {
        let parts: Vec<&str> = v.split(' ').collect();
        assert!(parts.len() == 2);
        let direction = Direction::from(*parts.first().expect("direction"));
        let steps = parts
            .get(1)
            .expect("steps")
            .parse()
            .expect("should be a digit");
        Self { steps, direction }
    }
}

impl From<&str> for Motions {
    fn from(v: &str) -> Self {
        let mut movements = vec![];
        for line in v.lines() {
            movements.push(Motion::from(line));
        }
        Self(movements)
    }
}

impl AsRef<Position> for Head {
    fn as_ref(&self) -> &Position {
        &self.0
    }
}

impl AsRef<Position> for Tail {
    fn as_ref(&self) -> &Position {
        &self.0
    }
}

impl Next for Position {
    fn next(&self, toward: &Convolution) -> Position {
        match toward {
            Convolution::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Convolution::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Convolution::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Convolution::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Convolution::UpLeft => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Convolution::UpRight => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Convolution::DownLeft => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Convolution::DownRight => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }
}

impl Overlap for Position {
    fn overlap(&self, related: &Self) -> bool {
        self.x == related.x && self.y == related.y
    }
}

impl AdjacentPositions for Position {
    fn adjacent_positions(&self) -> Vec<Position> {
        vec![
            self.next(&Convolution::Up),
            self.next(&Convolution::Down),
            self.next(&Convolution::Left),
            self.next(&Convolution::Right),
            self.next(&Convolution::UpLeft),
            self.next(&Convolution::UpRight),
            self.next(&Convolution::DownLeft),
            self.next(&Convolution::DownRight),
        ]
    }
}

impl Adjacent for Position {
    fn adjacent(&self, related: &Self) -> bool {
        self.adjacent_positions().iter().any(|x| x.overlap(related))
    }
}

impl Aligned for Position {
    fn aligned(&self, related: &Self, axis: &Axis) -> bool {
        match axis {
            Axis::Row => self.y == related.y,
            Axis::Column => self.x == related.x,
        }
    }
}

pub trait Aligned {
    fn aligned(&self, related: &Self, axis: &Axis) -> bool;
}

pub trait Next {
    fn next(&self, toward: &Convolution) -> Self;
}

pub trait Overlap {
    fn overlap(&self, related: &Self) -> bool;
}

pub trait Adjacent {
    fn adjacent(&self, related: &Self) -> bool;
}

pub trait Touching {
    fn touching(&self, related: &Self) -> bool;
}

impl<T> Touching for T
where
    T: Overlap + Adjacent,
{
    fn touching(&self, related: &Self) -> bool {
        self.overlap(related) || self.adjacent(related)
    }
}

pub trait AdjacentPositions {
    fn adjacent_positions(&self) -> Vec<Position>;
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Motion;
    use super::Motions;

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn parse() {
        let motions = Motions::from(INPUT);
        assert_eq!(motions.0.len(), 8);
        let motion = motions.0.get(0).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Right,
                steps: 4
            }
        );
        let motion = motions.0.get(2).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Left,
                steps: 3
            }
        );
        let motion = motions.0.get(5).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Down,
                steps: 1
            }
        );
    }
}
