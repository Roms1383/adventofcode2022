use super::types::{Axis, Convolution, Direction, Position};

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

pub trait AdjacentPositions {
    fn adjacent_positions(&self) -> Vec<Position>;
}

pub trait Strategy {
    fn should_move(&self, followed: &Self, direction: &Direction) -> Convolution;
}
