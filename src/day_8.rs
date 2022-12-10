use std::ops::Add;

use ndarray::prelude::*;
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct Tree(usize);

impl Zero for Tree {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Add for Tree {
    type Output = Tree;

    fn add(self, rhs: Self) -> Self::Output {
        Tree(self.0 + rhs.0)
    }
}

#[derive(Debug)]
pub struct Forest(Array2<Tree>);

impl From<Forest> for Array2<usize> {
    fn from(v: Forest) -> Self {
        v.0.mapv(|a: Tree| a.0)
    }
}

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let h = v.lines().clone().count();
        let mut peekable = v.lines().peekable();
        let w = peekable.peek().unwrap().len();
        let mut a = Array2::<Tree>::zeros((h, w));
        for mut row in a.axis_iter_mut(Axis(0)) {
            let line = peekable.next().unwrap();
            for (idx, c) in line.chars().enumerate() {
                row[idx] = Tree(c.to_digit(10).unwrap() as usize);
            }
        }
        Self(a)
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;
    use ndarray::{arr2, Array2};

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn parse() {
        let forest = Forest::from(INPUT);
        let raw = Array2::<usize>::from(forest);
        assert_eq!(
            raw,
            arr2(&[
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0],
            ])
        );
    }
}
