use std::collections::HashSet;

pub trait IsStartMarker {
    fn is_start_marker(&self) -> bool;
}

impl<'a> IsStartMarker for &'a str {
    fn is_start_marker(&self) -> bool {
        let mut set = HashSet::with_capacity(4);
        for c in self.chars() {
            if !set.insert(c) {
                return false;
            }
        }
        true
    }
}

pub fn start_at(buf: &str, len: usize) -> usize {
    let mut index = 0;
    let bound = buf.len();
    let delim = len - 1;
    while index <= bound {
        if index > delim {
            if (&buf[index - len..index]).is_start_marker() {
                return index;
            }
        }
        index += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::start_at;

    const ONE: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TWO: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const THREE: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    const FOUR: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const FIVE: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn starter() {
        const LEN: usize = 4;
        assert_eq!(start_at(ONE, LEN), 7);
        assert_eq!(start_at(TWO, LEN), 5);
        assert_eq!(start_at(THREE, LEN), 6);
        assert_eq!(start_at(FOUR, LEN), 10);
        assert_eq!(start_at(FIVE, LEN), 11);
    }

    #[test]
    fn message() {
        const LEN: usize = 14;
        assert_eq!(start_at(ONE, LEN), 19);
        assert_eq!(start_at(TWO, LEN), 23);
        assert_eq!(start_at(THREE, LEN), 23);
        assert_eq!(start_at(FOUR, LEN), 29);
        assert_eq!(start_at(FIVE, LEN), 26);
    }
}
