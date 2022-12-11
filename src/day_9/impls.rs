use super::{
    traits::{Adjacent, AdjacentPositions, Follow, Next, Overlap, Touching},
    types::{Convolution, Direction, Knot, Knots, Motion, Motions, Position},
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
    #[allow(unused_assignments)]
    fn do_motion(&mut self, motion: &Motion) {
        for _ in 0..motion.steps {
            let knots = self.knots.as_mut();
            let mut leader: Knot;
            let mut follower;
            let mut next;
            let mut moved = None;
            let count = knots.len();
            let mut projection: Knot = knots
                .get(0)
                .unwrap()
                .clone()
                .0
                .next(&motion.direction.into())
                .into();
            *knots.get_mut(0).unwrap() = projection.clone();
            for current in 0..(count - 1) {
                next = current + 1;
                leader = knots.get(current).unwrap().clone();
                follower = knots.get(next).unwrap().clone();
                let lead: &Convolution = &motion.direction.into();
                projection = leader.0.clone().into();
                if !follower.touching(&projection) {
                    let follow = follower.follow(&leader);
                    *knots.get_mut(next).unwrap() = follower.0.next(&follow).into();
                    if next == (count - 1) {
                        moved = Some(knots.get(next).unwrap().clone().0);
                    }
                }
            }
            if let Some(ref moved) = moved {
                self.record_tail_visited(moved);
            }
        }
    }
    fn record_tail_visited(&mut self, at: &Position) {
        if !self.visited.contains(&at.into()) {
            self.visited.push(at.clone().into());
        }
    }
    pub fn do_motions(&mut self, motions: &Motions) {
        self.visited.push(self.knots.last().unwrap().clone());
        for motion in motions.0.iter() {
            self.do_motion(&motion);
        }
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
        let len = self.knots.len() - 1;
        for (idx, knot) in self.knots.iter().enumerate() {
            match (true, idx) {
                (_, 0) => {
                    acc.push_str(&format!("{} {knot}\n", "H".to_string().cyan()));
                }
                (_, idx) if idx == len => {
                    acc.push_str(&format!("{} {knot}\n", "T".to_string().yellow()));
                }
                _ => {
                    acc.push_str(&format!("{} {knot}\n", idx));
                }
            }
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

impl<T> AdjacentPositions for T
where
    T: Next + Sized,
{
    fn adjacent_positions(&self) -> Vec<Self> {
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

impl<T> Touching for T
where
    T: Overlap + Adjacent,
{
    fn touching(&self, related: &Self) -> bool {
        self.overlap(related) || self.adjacent(related)
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

impl Overlap for Knot {
    fn overlap(&self, related: &Self) -> bool {
        self.0.overlap(&related.0)
    }
}

impl Adjacent for Position {
    fn adjacent(&self, related: &Self) -> bool {
        self.adjacent_positions().iter().any(|x| x.overlap(related))
    }
}

impl Adjacent for Knot {
    fn adjacent(&self, related: &Self) -> bool {
        self.0.adjacent(&related.0)
    }
}

impl Follow for Position {
    fn follow(&self, leader: &Self) -> Convolution {
        match true {
            // two steps ahead
            _ if self.y == leader.y && (self.x + 2) == leader.x => Convolution::Right,
            _ if self.y == leader.y && (self.x - 2) == leader.x => Convolution::Left,
            _ if self.x == leader.x && (self.y + 2) == leader.y => Convolution::Down,
            _ if self.x == leader.x && (self.y - 2) == leader.y => Convolution::Up,
            // diagonal
            _ if (self.x + 1) == leader.x && (self.y - 2) == leader.y => Convolution::UpRight,
            _ if (self.x + 2) == leader.x && (self.y - 1) == leader.y => Convolution::UpRight,
            _ if (self.x + 2) == leader.x && (self.y - 2) == leader.y => Convolution::UpRight,
            _ if (self.x - 1) == leader.x && (self.y - 2) == leader.y => Convolution::UpLeft,
            _ if (self.x - 2) == leader.x && (self.y - 1) == leader.y => Convolution::UpLeft,
            _ if (self.x - 2) == leader.x && (self.y - 2) == leader.y => Convolution::UpLeft,
            _ if (self.x + 1) == leader.x && (self.y + 2) == leader.y => Convolution::DownRight,
            _ if (self.x + 2) == leader.x && (self.y + 1) == leader.y => Convolution::DownRight,
            _ if (self.x + 2) == leader.x && (self.y + 2) == leader.y => Convolution::DownRight,
            _ if (self.x - 1) == leader.x && (self.y + 2) == leader.y => Convolution::DownLeft,
            _ if (self.x - 2) == leader.x && (self.y + 1) == leader.y => Convolution::DownLeft,
            _ if (self.x - 2) == leader.x && (self.y + 2) == leader.y => Convolution::DownLeft,
            _ => panic!("should not happen"),
        }
    }
}

impl Follow for Knot {
    fn follow(&self, leader: &Self) -> Convolution {
        self.0.follow(&leader.0)
    }
}
