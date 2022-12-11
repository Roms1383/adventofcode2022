use super::types::Convolution;

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

pub trait Follow {
    fn follow(&self, leader: &Self) -> Convolution;
}
