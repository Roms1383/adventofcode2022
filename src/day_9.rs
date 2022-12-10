#![allow(dead_code)]

use colored::Colorize;

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

impl std::fmt::Display for Motion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.direction.as_ref().yellow(),
            self.steps.to_string().yellow()
        )
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Up => "⬆️".to_string(),
            Self::Down => "⬇️".to_string(),
            Self::Left => "⬅️".to_string(),
            Self::Right => "➡️".to_string(),
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, PartialEq)]
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

impl std::fmt::Display for Convolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Up => "⬆️".to_string(),
            Self::Down => "⬇️".to_string(),
            Self::Left => "⬅️".to_string(),
            Self::Right => "➡️".to_string(),
            Self::UpLeft => "↖️".to_string(),
            Self::UpRight => "↗️".to_string(),
            Self::DownLeft => "↙️".to_string(),
            Self::DownRight => "↘️".to_string(),
        };
        write!(f, "{}", symbol)
    }
}

impl Motion {
    fn axis(&self) -> Axis {
        match self.direction {
            Direction::Right | Direction::Left => Axis::Row,
            Direction::Up | Direction::Down => Axis::Column,
        }
    }
}

impl From<(isize, isize)> for Position {
    fn from(v: (isize, isize)) -> Self {
        Self { x: v.0, y: v.1 }
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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

impl std::fmt::Display for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {} y: {}",
            self.0.x.to_string().blue(),
            self.0.y.to_string().blue()
        )
    }
}

impl std::fmt::Display for Tail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {} y: {}",
            self.0.x.to_string().red(),
            self.0.y.to_string().red()
        )
    }
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

impl Default for Manager {
    fn default() -> Self {
        Self {
            head: Head(Position { x: 0, y: 0 }),
            tail: Tail(Position { x: 0, y: 0 }),
            visited: vec![],
        }
    }
}

impl Manager {
    fn do_motion(&mut self, motion: &Motion) {
        println!("{motion} (head: {}, tail: {})", self.head, self.tail);
        for _ in 0..motion.steps {
            self.head.0 = self.head.as_ref().next(&motion.direction.into());
            println!("{}  head ({})", motion.direction, self.head);
            if !self.tail.0.touching(&self.head.0) {
                let convolution = if self.tail.0.aligned(&self.head.0, &motion.axis()) {
                    motion.direction.into()
                } else {
                    self.tail.0.should_move(&self.head.0, &motion.direction)
                };
                let position = self.tail.as_ref().next(&convolution);
                self.tail.0 = position;
                println!("{convolution}  tail ({})", self.tail);
                self.record_tail_visited(&position);
            }
        }
        println!("\n");
    }
    fn record_tail_visited(&mut self, at: &Position) {
        if !self.visited.contains(at) {
            self.visited.push(at.clone());
        }
    }
    pub fn do_motions(&mut self, motions: &Motions) {
        self.visited.push(self.tail.0);
        for motion in motions.0.iter() {
            self.do_motion(motion);
        }
    }
    pub fn total_tail_visited(&self) -> usize {
        self.visited.len()
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

impl AsRef<str> for Direction {
    fn as_ref(&self) -> &str {
        match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
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

impl Strategy for Position {
    fn should_move(&self, followed: &Self, direction: &Direction) -> Convolution {
        match direction {
            Direction::Up if self.x < followed.x => Convolution::UpRight,
            Direction::Up if self.x > followed.x => Convolution::UpLeft,
            Direction::Down if self.x < followed.x => Convolution::DownLeft,
            Direction::Down if self.x > followed.x => Convolution::DownRight,
            Direction::Left if self.y < followed.y => Convolution::DownLeft,
            Direction::Left if self.y > followed.y => Convolution::UpLeft,
            Direction::Right if self.y < followed.y => Convolution::DownRight,
            Direction::Right if self.y > followed.y => Convolution::UpRight,
            _ => panic!("should not happen"),
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

pub trait Strategy {
    fn should_move(&self, followed: &Self, direction: &Direction) -> Convolution;
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::Convolution;
    use super::Direction;
    use super::Manager;
    use super::Motion;
    use super::Motions;
    use super::Position;
    use super::Strategy;

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

    // #[test]
    // fn should_move() {
    //     let head: Position = (3, -2).into();
    //     let tail: Position = (2, 0).into();
    //     let direction: Direction = Direction::Up;
    //     let position = tail.should_move(&head, &direction);
    //     assert_eq!(position, Convolution::UpLeft);

    //     let head: Position = (2, -2).into();
    //     let tail: Position = (3, 0).into();
    //     let direction: Direction = Direction::Up;
    //     let position = tail.should_move(&head, &direction);
    //     assert_eq!(position, Convolution::UpRight);

    //     let head: Position = (3, 2).into();
    //     let tail: Position = (2, 0).into();
    //     let direction: Direction = Direction::Down;
    //     let position = tail.should_move(&head, &direction);
    //     assert_eq!(position, Convolution::DownLeft);

    //     let head: Position = (2, 2).into();
    //     let tail: Position = (3, 0).into();
    //     let direction: Direction = Direction::Down;
    //     let position = tail.should_move(&head, &direction);
    //     assert_eq!(position, Convolution::DownRight);
    // }

    #[test]
    fn visited() {
        let motions = Motions::from(INPUT);
        let mut manager = Manager::default();
        manager.do_motions(&motions);
        assert_eq!(manager.visited.len(), 13);
    }
}
