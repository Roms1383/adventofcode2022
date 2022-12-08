#[derive(Debug, Clone)]
pub struct Supply(Vec<u32>);

#[derive(Debug)]
pub struct Supplies(Vec<Supply>);

pub trait Calories {
    fn calories(&self) -> u32;
}

pub trait TopCalories {
    fn top_calories(&self, amount: usize) -> Vec<u32>;
}

impl Calories for Supply {
    fn calories(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl Calories for Vec<Supply> {
    fn calories(&self) -> u32 {
        let mut max = 0;
        let mut current;
        for supply in self {
            current = supply.calories();
            if current > max {
                max = current;
            }
        }
        max
    }
}

impl Calories for Supplies {
    fn calories(&self) -> u32 {
        self.0.calories()
    }
}

impl Calories for Vec<u32> {
    fn calories(&self) -> u32 {
        self.iter().sum()
    }
}

impl TopCalories for Supplies {
    fn top_calories(&self, times: usize) -> Vec<u32> {
        let mut inner = self.0.clone();
        let mut calories = Vec::with_capacity(times);
        let mut v;
        let mut idx;
        for _ in 0..times {
            v = inner.calories();
            calories.push(v);
            idx = inner
                .iter()
                .enumerate()
                .find_map(|(index, item)| {
                    if item.calories() == v {
                        return Some(index);
                    } else {
                        None
                    }
                })
                .unwrap();
            inner.remove(idx);
        }
        calories
    }
}

impl From<&str> for Supplies {
    fn from(v: &str) -> Self {
        let mut supplies = vec![];
        let mut supply = vec![];
        for line in v.lines() {
            if !line.trim().is_empty() {
                supply.push(line.trim().parse().expect("should be a digit"));
            } else {
                supplies.push(Supply(supply.clone()));
                supply.clear();
            }
        }
        supplies.push(Supply(supply.clone()));
        Supplies(supplies)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_1::TopCalories;

    use super::{Calories, Supplies};

    #[test]
    fn convert() {
        let s = "1000
      2000
      3000
      
      4000
      
      5000
      6000
      
      7000
      8000
      9000
      
      10000";
        let supplies = Supplies::from(s);
        println!("{supplies:#?}");
        assert_eq!(supplies.0.len(), 5);
        assert_eq!(supplies.0.get(0).unwrap().calories(), 6_000);
        assert_eq!(supplies.0.get(1).unwrap().calories(), 4_000);
        assert_eq!(supplies.0.get(2).unwrap().calories(), 11_000);
        assert_eq!(supplies.0.get(3).unwrap().calories(), 24_000);
        assert_eq!(supplies.0.get(4).unwrap().calories(), 10_000);
        assert_eq!(supplies.calories(), 24_000);
        assert_eq!(supplies.top_calories(3), vec![24_000, 11_000, 10_000]);
        assert_eq!(supplies.top_calories(3).calories(), 45_000);
    }
}
