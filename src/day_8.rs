#[derive(Debug, Clone)]
pub struct Tree {
    x: usize,
    y: usize,
    size: usize,
}

#[derive(Debug)]
pub struct Forest(Vec<Vec<Tree>>);

impl From<&str> for Forest {
    fn from(v: &str) -> Self {
        let h = v.lines().clone().count();
        let w = v.lines().clone().next().unwrap().len();
        let mut row: Vec<Tree> = Vec::with_capacity(w);
        let mut grid = Vec::with_capacity(h);
        let mut y = 0;
        for line in v.lines() {
            for (x, c) in line.chars().enumerate() {
                row.push(Tree {
                    x,
                    y,
                    size: c.to_digit(10).expect("should be a digit") as usize,
                })
            }
            grid.push(row.clone());
            row.clear();
            y += 1;
        }
        Forest(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn parse() {
        let forest = Forest::from(INPUT);
        assert_eq!(forest.0.get(0).unwrap().len(), 5);
        assert_eq!(forest.0.len(), 5);
    }
}
