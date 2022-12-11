use super::{
    traits::{Adjacent, AdjacentPositions, Aligned, Next, Overlap, Strategy},
    types::{Axis, Convolution, Direction, Knot, Knots, Motion, Motions, Position},
};
use colored::Colorize;

impl From<&Position> for Knot {
    fn from(v: &Position) -> Self {
        Self(v.clone())
    }
}

impl From<Position> for Knot {
    fn from(v: Position) -> Self {
        Self(v)
    }
}

impl<const LENGTH: usize> Knots<LENGTH> {
    fn do_motion(&mut self, motion: &Motion) {
        todo!()
    }
    fn record_tail_visited(&mut self, at: &Position) {
        if !self.visited.contains(&at.into()) {
            self.visited.push(at.clone().into());
        }
    }
    pub fn do_motions(&mut self, motions: &Motions) {
        todo!()
    }
    pub fn total_tail_visited(&self) -> usize {
        self.visited.len()
    }
}

impl<const LENGTH: usize> Default for Knots<LENGTH> {
    fn default() -> Self {
        Self {
            knots: [Knot::default(); LENGTH],
            visited: vec![],
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Knot {
    fn default() -> Self {
        Self(Position::default())
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
            Direction::Down if self.x < followed.x => Convolution::DownRight,
            Direction::Down if self.x > followed.x => Convolution::DownLeft,
            Direction::Left if self.y < followed.y => Convolution::DownLeft,
            Direction::Left if self.y > followed.y => Convolution::UpLeft,
            Direction::Right if self.y < followed.y => Convolution::DownRight,
            Direction::Right if self.y > followed.y => Convolution::UpRight,
            _ => panic!("should not happen"),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
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

impl std::fmt::Display for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {} y: {}",
            self.0.x.to_string().red(),
            self.0.y.to_string().red()
        )
    }
}

impl<const LENGTH: usize> std::fmt::Display for Knots<LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = "".to_string();
        for (idx, knot) in self.knots.iter().enumerate() {
            acc.push_str(format!("{} {knot}\n", idx + 1).as_str());
        }
        write!(f, "{}", acc)
    }
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

impl AsRef<str> for Direction {
    fn as_ref(&self) -> &str {
        match self {
            Self::Up => "U",
            Self::Down => "D",
            Self::Left => "L",
            Self::Right => "R",
        }
    }
}

impl AsRef<Position> for Knot {
    fn as_ref(&self) -> &Position {
        &self.0
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
            Direction::Up => Self::Up,
            Direction::Down => Self::Down,
            Direction::Left => Self::Left,
            Direction::Right => Self::Right,
        }
    }
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
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
