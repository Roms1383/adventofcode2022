use std::fs::read_to_string;

mod day_11;

fn main() {
    let puzzle = read_to_string("./day.11.txt").expect("cannot read puzzle.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {}
}
