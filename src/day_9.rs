#![allow(dead_code)]

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    steps: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
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
