#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    steps: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Head(Position);

#[derive(Debug, Clone, Copy)]
pub struct Tail(Position);

pub struct Something {
    head: Head,
    tail: Tail,
    visited: Vec<Position>,
}

#[derive(Debug)]
pub struct Movements(Vec<Movement>);

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unknown direction"),
        }
    }
}

impl From<&str> for Movement {
    fn from(v: &str) -> Self {
        let parts: Vec<&str> = v.split(' ').collect();
        assert!(parts.len() == 2);
        let direction = Direction::from(*parts.first().expect("direction"));
        let steps = parts
            .get(1)
            .expect("steps")
            .parse()
            .expect("should be a digit");
        Self { steps, direction }
    }
}

impl From<&str> for Movements {
    fn from(v: &str) -> Self {
        let mut movements = vec![];
        for line in v.lines() {
            movements.push(Movement::from(line));
        }
        Self(movements)
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Movement;
    use super::Movements;

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn parse() {
        let movements = Movements::from(INPUT);
        assert_eq!(movements.0.len(), 8);
        let movement = movements.0.get(0).unwrap();
        assert_eq!(
            movement,
            &Movement {
                direction: Direction::Right,
                steps: 4
            }
        );
        let movement = movements.0.get(2).unwrap();
        assert_eq!(
            movement,
            &Movement {
                direction: Direction::Left,
                steps: 3
            }
        );
        let movement = movements.0.get(5).unwrap();
        assert_eq!(
            movement,
            &Movement {
                direction: Direction::Down,
                steps: 1
            }
        );
    }
}
