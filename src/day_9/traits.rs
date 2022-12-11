use super::types::{Convolution, Direction};

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

pub trait AdjacentPositions
where
    Self: Next + Sized,
{
    fn adjacent_positions(&self) -> Vec<Self>;
}

pub trait Diagonal {
    fn diagonal(&self, followed: &Self, direction: &Direction) -> Convolution;
}

pub trait TwoStepsAhead {
    fn two_steps_ahead(&self, leader: &Self) -> Option<Direction>;
}
