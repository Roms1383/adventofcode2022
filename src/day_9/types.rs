#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knot(pub Position);

pub struct Knots<const LENGTH: usize> {
    pub knots: [Knot; LENGTH],
    pub visited: Vec<Knot>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub steps: usize,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct Motions(pub Vec<Motion>);

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
