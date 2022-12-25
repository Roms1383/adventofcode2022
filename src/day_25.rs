use num_traits::pow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl SnafuDigit {
    fn as_digit(&self) -> isize {
        match self {
            SnafuDigit::Two => 2,
            SnafuDigit::One => 1,
            SnafuDigit::Zero => 0,
            SnafuDigit::Minus => -1,
            SnafuDigit::DoubleMinus => -2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SnafuNumber(Vec<SnafuDigit>);

impl SnafuNumber {
    fn get_place(&self, at: usize) -> usize {
        if at > (self.0.len() - 1) {
            panic!("{at} is out of bounds ({})", self.0.len());
        }
        pow(5, self.0.len() - (at + 1))
    }
    fn sum(&self) -> isize {
        self.0.iter().enumerate().fold(0isize, |acc, (idx, digit)| {
            acc + (self.get_place(idx) as isize * digit.as_digit())
        })
    }
}

#[derive(Debug)]
pub struct SnafuNumbers(Vec<SnafuNumber>);

impl SnafuNumbers {
    pub fn sum(&self) -> isize {
        self.0.iter().fold(0, |acc, x| acc + x.sum())
    }
}

// all credits to Chris Biscardi
impl From<isize> for SnafuNumber {
    fn from(value: isize) -> Self {
        let v = itertools::unfold(value, |x| {
            if x == &0 {
                None
            } else {
                match *x % 5 {
                    0 => {
                        *x /= 5;
                        Some(SnafuDigit::Zero)
                    }
                    1 => {
                        *x -= 1;
                        *x /= 5;
                        Some(SnafuDigit::One)
                    }
                    2 => {
                        *x -= 2;
                        *x /= 5;
                        Some(SnafuDigit::Two)
                    }
                    3 => {
                        *x -= -2;
                        *x /= 5;
                        Some(SnafuDigit::DoubleMinus)
                    }
                    4 => {
                        *x -= -1;
                        *x /= 5;
                        Some(SnafuDigit::Minus)
                    }
                    _ => panic!("should not happen"),
                }
            }
        })
        .collect::<Vec<SnafuDigit>>();
        SnafuNumber(v.into_iter().rev().collect())
    }
}

impl From<char> for SnafuDigit {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '1' => Self::One,
            '0' => Self::Zero,
            '-' => Self::Minus,
            '=' => Self::DoubleMinus,
            _ => panic!("should not happen"),
        }
    }
}

impl From<SnafuDigit> for char {
    fn from(value: SnafuDigit) -> Self {
        match value {
            SnafuDigit::Two => '2',
            SnafuDigit::One => '1',
            SnafuDigit::Zero => '0',
            SnafuDigit::Minus => '-',
            SnafuDigit::DoubleMinus => '=',
        }
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        let mut acc = String::from("");
        for snafu in self.0.iter() {
            acc.push(char::from(*snafu));
        }
        acc
    }
}

impl From<&str> for SnafuNumber {
    fn from(value: &str) -> Self {
        let mut digits = vec![];
        for character in value.chars() {
            digits.push(character.into());
        }
        Self(digits)
    }
}

impl From<&str> for SnafuNumbers {
    fn from(value: &str) -> Self {
        let mut snafus = vec![];
        for line in value.lines() {
            snafus.push(SnafuNumber::from(line));
        }
        Self(snafus)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{SnafuDigit, SnafuNumber, SnafuNumbers};

    #[test]
    fn simple() {
        let input = "2=-01";
        let snafu = SnafuNumber::from(input);
        assert_eq!(
            snafu,
            SnafuNumber(vec![
                SnafuDigit::Two,
                SnafuDigit::DoubleMinus,
                SnafuDigit::Minus,
                SnafuDigit::Zero,
                SnafuDigit::One
            ])
        );
        assert_eq!(snafu.get_place(0), 625);
        assert_eq!(snafu.sum(), 976);
    }

    #[test_case("1=-0-2", 1747; "1=-0-2")]
    #[test_case("12111", 906 ; "12111")]
    #[test_case("2=0=", 198 ; "2=0=")]
    #[test_case("21", 11 ; "21")]
    #[test_case("2=01", 201 ; "2=01")]
    #[test_case("111", 31 ; "111")]
    #[test_case("20012", 1257 ; "20012")]
    #[test_case("112", 32 ; "112")]
    #[test_case("1=-1=", 353 ; "1=-1=")]
    #[test_case("1-12", 107 ; "1-12")]
    #[test_case("12", 7 ; "12")]
    #[test_case("1=", 3 ; "1=")]
    #[test_case("122", 37 ; "122")]
    fn multiple(given: &str, expected: isize) {
        let snafu = SnafuNumber::from(given);
        assert_eq!(snafu.sum(), expected);
    }

    #[test]
    fn complete() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let snafus = SnafuNumbers::from(input);
        assert_eq!(snafus.0.len(), 13);
        assert_eq!(snafus.sum(), 4890);
        // all credits to Chris Biscardi
        assert_eq!(SnafuNumber::from(4890).to_string(), "2=-1=0");
    }
}
