pub trait Priority {
    fn priority(&self) -> usize;
}

pub trait UniqueChars {
    fn unique_chars(&self) -> Vec<char>;
}

pub trait CommonChar {
    fn common_char(&self) -> char;
}

pub trait Priorities {
    fn priorities(&self) -> Vec<(char, usize)>;
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

#[derive(Debug, Clone)]
pub struct Compartment(String);

impl UniqueChars for Compartment {
    fn unique_chars(&self) -> Vec<char> {
        self.0.unique_chars()
    }
}

impl UniqueChars for String {
    fn unique_chars(&self) -> Vec<char> {
        let mut uniques = vec![];
        for char in self.chars() {
            if uniques.is_empty() || !uniques.contains(&char) {
                uniques.push(char);
            }
        }
        uniques
    }
}

#[derive(Debug, Clone)]
pub struct Rucksack {
    first: Compartment,
    second: Compartment,
}

impl CommonChar for Rucksack {
    fn common_char(&self) -> char {
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

impl CommonChar for Group {
    fn common_char(&self) -> char {
        let first = self.first.unique_chars();
        let second = self.second.unique_chars();
        let third = self.third.unique_chars();
        for a in first.iter() {
            for b in second.iter() {
                if a == b {
                    for c in third.iter() {
                        if a == c {
                            return *a;
                        }
                    }
                }
            }
        }

        panic!("common char not found");
    }
}

#[derive(Debug)]
pub struct Rucksacks(Vec<Rucksack>);

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

impl<T> Priorities for Vec<T>
where
    T: CommonChar,
{
    fn priorities(&self) -> Vec<(char, usize)> {
        let mut priorities = vec![];
        let mut c: char;
        for rucksack in self.iter() {
            c = rucksack.common_char();
            priorities.push((c, c.priority()));
        }
        priorities
    }
}

impl Priorities for Rucksacks {
    fn priorities(&self) -> Vec<(char, usize)> {
        self.0.priorities()
    }
}

#[derive(Debug)]
pub struct Group {
    first: String,
    second: String,
    third: String,
}

#[derive(Debug)]
pub struct Groups(Vec<Group>);

impl From<&str> for Groups {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let mut groups = vec![];
        while let (Some(first), Some(second), Some(third)) =
            (lines.next(), lines.next(), lines.next())
        {
            let group = Group {
                first: first.trim().into(),
                second: second.trim().into(),
                third: third.trim().into(),
            };
            groups.push(group);
        }
        Groups(groups)
    }
}

impl Priorities for Groups {
    fn priorities(&self) -> Vec<(char, usize)> {
        self.0.priorities()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommonChar, Compartment, Groups, Priorities, Priority, Rucksack, Rucksacks, UniqueChars,
    };

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
        assert_eq!(rucksack.common_char(), 'p');

        let s = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::from(s);
        assert_eq!(rucksack.common_char(), 'L');
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

    #[test]
    fn convert_to_groups() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
      jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
      PmmdzqPrVvPwwTWBwg
      wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
      ttgJtRGJQctTZtZT
      CrZsJsPPZsGzwwsLwLmpwMDw";
        let groups = Groups::from(s);
        assert_eq!(groups.0.len(), 2);
        assert_eq!(groups.0.get(0).unwrap().common_char(), 'r');
        assert_eq!(groups.0.get(1).unwrap().common_char(), 'Z');
    }
}
