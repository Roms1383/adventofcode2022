#![allow(dead_code, unused_imports)]

use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::BufReader,
    ops::Index,
    path::Path,
};

fn main() {
    let puzzle = read_to_string("./day.3.1.txt").expect("cannot read puzzle.txt");
    let rucksacks = Rucksacks::from(puzzle.as_str());
    let priorities = rucksacks.priorities();
    println!("sum: {}", priorities.iter().fold(0, |acc, (_, x)| acc + x));
    println!("priorities:\n{priorities:#?}");
}

pub trait Priority {
    fn priority(&self) -> usize;
}

impl Priority for char {
    fn priority(&self) -> usize {
        if let Some(index) = ('a'..='z').into_iter().position(|ref x| x == self) {
            return index + 1;
        }
        if let Some(index) = ('A'..='Z').into_iter().position(|ref x| x == self) {
            return index + 1 + 26;
        }
        panic!("char not found");
    }
}

#[derive(Debug)]
struct Compartment(String);

impl Compartment {
    fn unique_chars(&self) -> Vec<char> {
        let mut uniques = vec![];
        for char in self.0.chars() {
            if uniques.len() == 0 || !uniques.contains(&char) {
                uniques.push(char);
            }
        }
        uniques
    }
}

#[derive(Debug)]
struct Rucksack {
    first: Compartment,
    second: Compartment,
}

impl Rucksack {
    fn find_common(&self) -> char {
        let first = self.first.unique_chars();
        let second = self.second.unique_chars();
        for a in first.iter() {
            for b in second.iter() {
                if a == b {
                    return *a;
                }
            }
        }

        panic!("common char not found");
    }
}

#[derive(Debug)]
struct Rucksacks(Vec<Rucksack>);

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let input = input.trim();
        let (first, second) = input.split_at(input.len() / 2);
        Rucksack {
            first: Compartment(first.into()),
            second: Compartment(second.into()),
        }
    }
}

impl From<&str> for Rucksacks {
    fn from(input: &str) -> Self {
        Rucksacks(input.lines().map(Rucksack::from).collect())
    }
}

impl Rucksacks {
    fn priorities(&self) -> Vec<(char, usize)> {
        let mut priorities = vec![];
        let mut c: char;
        for rucksack in self.0.iter() {
            c = rucksack.find_common();
            priorities.push((c, c.priority()));
        }
        priorities
    }
}

#[cfg(test)]
mod tests {
    use crate::{Compartment, Priority, Rucksack, Rucksacks};

    #[test]
    fn convert_one() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from(s);
        assert_eq!(rucksack.first.0.as_str(), "vJrwpWtwJgWr");
        assert_eq!(rucksack.second.0.as_str(), "hcsFMMfFFhFp");
    }

    #[test]
    fn convert_all() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks = Rucksacks::from(s);
        assert_eq!(rucksacks.0.len(), 6);
        assert_eq!(rucksacks.0.get(0).unwrap().first.0.as_str(), "vJrwpWtwJgWr");
        assert_eq!(
            rucksacks.0.get(0).unwrap().second.0.as_str(),
            "hcsFMMfFFhFp"
        );
    }

    #[test]
    fn find_uniques() {
        let s = "vJrwpWtwJgWr";
        assert_eq!(
            Compartment(s.into()).unique_chars(),
            vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'g']
        );
    }

    #[test]
    fn find_common() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from(s);
        assert_eq!(rucksack.find_common(), 'p');

        let s = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::from(s);
        assert_eq!(rucksack.find_common(), 'L');
    }

    #[test]
    fn priority() {
        let c = 'a';
        assert_eq!(c.priority(), 1);

        let c = 'z';
        assert_eq!(c.priority(), 26);

        let c = 'A';
        assert_eq!(c.priority(), 27);

        let c = 'Z';
        assert_eq!(c.priority(), 52);
    }

    #[test]
    fn summary() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks = Rucksacks::from(s);
        let priorities = rucksacks.priorities();
        assert_eq!(
            priorities,
            vec![
                ('p', 16),
                ('L', 38),
                ('P', 42),
                ('v', 22),
                ('t', 20),
                ('s', 19)
            ]
        );
        assert_eq!(priorities.iter().fold(0, |acc, (_, x)| acc + x), 157);
    }
}
