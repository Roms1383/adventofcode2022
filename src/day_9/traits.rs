use super::types::{Axis, Convolution, Direction};

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

pub trait AdjacentPositions
where
    Self: Next + Sized,
{
    fn adjacent_positions(&self) -> Vec<Self>;
}

pub trait Strategy {
    fn should_move(&self, followed: &Self, direction: &Direction) -> Convolution;
}
