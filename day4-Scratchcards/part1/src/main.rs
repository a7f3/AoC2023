use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
fn parse_scratchcard(line: &str) -> u32 {
    let split: Vec<&str> = line.split(": ").flat_map(|s| s.split(" | ")).collect();

    let winning_nums: Vec<u32> = split[1]
        .split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .collect();
    let your_nums: Vec<u32> = split[2]
        .split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .collect();

    let mut counter = 1;
    let mut points = 0;
    for num in winning_nums {
        if your_nums.contains(&num) {
            points = counter;
            counter *= 2;
        }
    }
    points
}

fn main() {
    let f = read_lines("./input");
    let mut total = 0;
    for line in f {
        total += parse_scratchcard(&line);
    }
    println!("{}", total);
}

#[cfg(test)]
mod test {
    use crate::{parse_scratchcard, read_lines};

    #[test]
    fn test_input() {
        let f = read_lines("./test-input");
        let mut lines = f.iter();
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 8);
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 2);
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 2);
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 1);
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 0);
        assert_eq!(parse_scratchcard(lines.next().unwrap()), 0);
    }
}
