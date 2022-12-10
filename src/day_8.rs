use ndarray::prelude::*;

#[derive(Debug)]
pub struct Forest(Array2<usize>);

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let h = v.lines().clone().count();
        let mut peekable = v.lines().peekable();
        let w = peekable.peek().unwrap().len();
        let mut a = Array2::<usize>::zeros((h, w));
        for mut row in a.axis_iter_mut(Axis(0)) {
            let line = peekable.next().unwrap();
            for (idx, c) in line.chars().enumerate() {
                row[idx] = c.to_digit(10).unwrap() as usize;
            }
        }
        Self(a)
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;
    use ndarray::arr2;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn parse() {
        let forest = Forest::from(INPUT);
        assert_eq!(
            forest.0,
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
