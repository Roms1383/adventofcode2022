use std::ops::{Range, RangeInclusive};

#[derive(Debug, Clone)]
pub struct Section(RangeInclusive<usize>);

#[derive(Debug, Clone)]
pub struct Pair {
    first: Section,
    second: Section,
}

pub struct Pairs(pub Vec<Pair>);

impl From<&str> for Section {
    fn from(v: &str) -> Self {
        let parts: Vec<usize> = v
            .split('-')
            .map(|x| x.parse::<usize>().expect("should be a digit"))
            .collect();
        if parts.len() > 2 {
            panic!("there should be only 2 digits");
        }
        let from = parts.get(0).expect("from should not be empty");
        let to = parts.get(1).expect("to should not be empty");
        Section(*from..=*to)
    }
}

impl From<&str> for Pair {
    fn from(v: &str) -> Self {
        let parts: Vec<&str> = v.trim().split(',').collect();
        if parts.len() > 2 {
            panic!("there should be only 2 digits");
        }
        let first = Section::from(*parts.get(0).expect("first should not be empty"));
        let second = Section::from(*parts.get(1).expect("second should not be empty"));
        Pair { first, second }
    }
}

impl From<&str> for Pairs {
    fn from(v: &str) -> Self {
        let mut pairs = vec![];
        for line in v.lines() {
            pairs.push(Pair::from(line));
        }
        Pairs(pairs)
    }
}

pub trait Overlap {
    fn overlap(&self, other: Self) -> bool;
}

pub trait AnyOverlap {
    fn any_overlap(&self) -> bool;
}

impl Overlap for RangeInclusive<usize> {
    fn overlap(&self, other: Self) -> bool {
        for v in other {
            if !self.contains(&v) {
                return false;
            }
        }
        true
    }
}

impl AnyOverlap for Pair {
    fn any_overlap(&self) -> bool {
        let outer = self.first.0.clone();
        let inner = self.second.0.clone();
        outer.overlap(inner.clone()) || inner.overlap(outer)
    }
}

impl Pairs {
    pub fn overlapping(&self) -> Pairs {
        Pairs(
            self.0
                .iter()
                .filter(|x| x.any_overlap())
                .map(Clone::clone)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Pairs;

    #[test]
    fn convert() {
        let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let pairs = Pairs::from(s);
        let one = pairs.0.get(0).unwrap();
        assert_eq!(one.first.0, 2..=4);
        assert_eq!(one.second.0, 6..=8);
        let two = pairs.0.get(1).unwrap();
        assert_eq!(two.first.0, 2..=3);
        assert_eq!(two.second.0, 4..=5);
    }

    #[test]
    fn overlap() {
        let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let pairs = Pairs::from(s);
        let pairs = pairs.overlapping();
        assert_eq!(pairs.0.len(), 2);
    }
}
