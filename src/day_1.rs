#[derive(Debug)]
pub struct Supply(Vec<u32>);

#[derive(Debug)]
pub struct Supplies(Vec<Supply>);

pub trait Calories {
    fn calories(&self) -> u32;
}

impl Calories for Supply {
    fn calories(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl Calories for Supplies {
    fn calories(&self) -> u32 {
        let mut max = 0;
        let mut current;
        for supply in &self.0 {
            current = supply.calories();
            if current > max {
                max = current;
            }
        }
        max
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
    }
}
