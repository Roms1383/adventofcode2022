#![allow(dead_code, unused_variables)]

pub mod impls;
pub mod traits;
pub mod types;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use crate::day_9::types::{Direction, Knots, Motion, Motions};

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const LARGER_INPUT: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn parse() {
        let motions = Motions::from(INPUT);
        assert_eq!(motions.0.len(), 8);
        let motion = motions.0.get(0).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Right,
                steps: 4
            }
        );
        let motion = motions.0.get(2).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Left,
                steps: 3
            }
        );
        let motion = motions.0.get(5).unwrap();
        assert_eq!(
            motion,
            &Motion {
                direction: Direction::Down,
                steps: 1
            }
        );
    }

    #[test]
    fn duo() {
        let motions = Motions::from(INPUT);
        let mut manager: Knots<2> = Knots::default();
        manager.do_motions(&motions);
        assert_eq!(manager.visited.len(), 13);
    }

    #[test]
    fn snake() {
        let motions = Motions::from(INPUT);
        let mut manager: Knots<10> = Knots::default();
        manager.do_motions(&motions);
        assert_eq!(manager.visited.len(), 1);

        let motions = Motions::from(LARGER_INPUT);
        let mut manager: Knots<10> = Knots::default();
        manager.do_motions(&motions);
        assert_eq!(manager.visited.len(), 36);
    }
}
