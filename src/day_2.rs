#![allow(dead_code)]

pub enum Side {
    Myself,
    Opponent,
}

#[derive(Debug, PartialEq)]
pub enum Game {
    Rock,
    Paper,
    Scissors,
}

pub trait Code {
    fn code(&self, side: Side) -> char;
}

pub trait Score {
    fn score(&self) -> u32;
}

impl Code for Game {
    fn code(&self, side: Side) -> char {
        match (self, side) {
            (Game::Rock, Side::Myself) => 'A',
            (Game::Paper, Side::Myself) => 'B',
            (Game::Scissors, Side::Myself) => 'C',
            (Game::Rock, Side::Opponent) => 'X',
            (Game::Paper, Side::Opponent) => 'Y',
            (Game::Scissors, Side::Opponent) => 'Z',
        }
    }
}

impl Score for Game {
    fn score(&self) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

fn outcome(round: &Round) -> Outcome {
    match (&round.myself, &round.opponent) {
        (Game::Rock, Game::Paper)
        | (Game::Paper, Game::Scissors)
        | (Game::Scissors, Game::Rock) => Outcome::Lose,
        (Game::Paper, Game::Rock)
        | (Game::Scissors, Game::Paper)
        | (Game::Rock, Game::Scissors) => Outcome::Win,
        _ => Outcome::Draw,
    }
}

impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    myself: Game,
    opponent: Game,
}

impl Score for Round {
    fn score(&self) -> u32 {
        self.myself.score() + outcome(self).score()
    }
}

pub struct Strategy(Vec<Round>);

impl From<char> for Game {
    fn from(v: char) -> Self {
        match v {
            'A' | 'X' => Game::Rock,
            'B' | 'Y' => Game::Paper,
            'C' | 'Z' => Game::Scissors,
            _ => panic!("invalid code"),
        }
    }
}

impl From<&str> for Round {
    fn from(v: &str) -> Self {
        let mut chars = v.trim().chars();
        let opponent: Game = chars.next().unwrap().into();
        chars.next(); // accounts for the space
        let myself: Game = chars.next().unwrap().into();
        if let Some(_) = chars.next() {
            panic!("round should only contain 2 games");
        }
        Round { myself, opponent }
    }
}

impl From<&str> for Strategy {
    fn from(v: &str) -> Self {
        let mut strategy = vec![];
        let lines = v.lines();
        for line in lines {
            strategy.push(line.trim().into());
        }
        Strategy(strategy)
    }
}

impl Score for Strategy {
    fn score(&self) -> u32 {
        let mut score = 0;
        for round in &self.0 {
            score += round.score();
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::{outcome, Outcome, Score};

    use super::{Game, Strategy};

    #[test]
    fn strategy() {
        let s = "A Y
      B X
      C Z";
        let strategy = Strategy::from(s);
        assert_eq!(strategy.0.get(0).unwrap().opponent, Game::Rock);
        assert_eq!(strategy.0.get(0).unwrap().myself, Game::Paper);
        assert_eq!(strategy.0.get(1).unwrap().opponent, Game::Paper);
        assert_eq!(strategy.0.get(1).unwrap().myself, Game::Rock);
        assert_eq!(strategy.0.get(2).unwrap().opponent, Game::Scissors);
        assert_eq!(strategy.0.get(2).unwrap().myself, Game::Scissors);
        assert_eq!(outcome(strategy.0.get(0).unwrap()), Outcome::Win);
        assert_eq!(outcome(strategy.0.get(1).unwrap()), Outcome::Lose);
        assert_eq!(outcome(strategy.0.get(2).unwrap()), Outcome::Draw);
        assert_eq!(strategy.score(), 15);
    }
}
