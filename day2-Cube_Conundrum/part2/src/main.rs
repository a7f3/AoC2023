use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn calculate(red: u32, green: u32, blue: u32, line: String) -> bool {
    let mut a = line.split(&[':', ';']);
    let game_num: u32 = a.next().unwrap().split(' ').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    for i in a {
        let b = i.trim();
        let b = b.split(',');
        for unit in b {
            let s: Vec<&str> = unit.trim().split_whitespace().collect();
            let num = s[0].parse::<u32>().unwrap();
            match s[1] {
                "red" => {
                    if num > red {
                        return false;
                    }
                }
                "green" => {
                    if num > green {
                        return false;
                    }
                }
                "blue" => {
                    if num > blue {
                        return false;
                    }
                }

                &_ => (),
            }
        }
    }

    true
}
pub fn calc_total(file: &str) -> u32 {
    let lines = read_lines(file);
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        if !calculate(12, 13, 14, line.to_string()) {
            continue;
        }

        total += i + 1;
    }
    total as u32
}

fn main() {
    println!("{}", calc_total("./input"));
}

#[cfg(test)]
mod tests {
    use crate::{calc_total, calculate, read_lines};

    #[test]
    fn line_parsing() {
        let lines = read_lines("./test-input");
        let mut lines = lines.iter();

        assert_eq!(calculate(12, 13, 14, lines.next().unwrap().clone()), true);
        assert_eq!(calculate(12, 13, 14, lines.next().unwrap().clone()), true);
        assert_eq!(calculate(12, 13, 14, lines.next().unwrap().clone()), false);
        assert_eq!(calculate(12, 13, 14, lines.next().unwrap().clone()), false);
        assert_eq!(calculate(12, 13, 14, lines.next().unwrap().clone()), true);
    }

    #[test]
    fn limits_correct() {
        let l1 = "Game 1: 3 blue, 12 red; 1 red, 2 green, 6 blue; 2 green";
        let l2 = "Game 1: 3 blue, 5 red; 1 red, 13 green, 6 blue; 2 green";
        let l3 = "Game 1: 14 blue, 5 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(calculate(12, 13, 14, l1.to_string()), true);
        assert_eq!(calculate(12, 13, 14, l2.to_string()), true);
        assert_eq!(calculate(12, 13, 14, l3.to_string()), true);
    }

    #[test]
    fn over_limits() {
        let l1 = "Game 1: 3 blue, 13 red; 1 red, 2 green, 6 blue; 2 green";
        let l2 = "Game 1: 3 blue, 5 red; 1 red, 14 green, 6 blue; 2 green";
        let l3 = "Game 1: 15 blue, 5 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(calculate(12, 13, 14, l1.to_string()), false);
        assert_eq!(calculate(12, 13, 14, l2.to_string()), false);
        assert_eq!(calculate(12, 13, 14, l3.to_string()), false);
    }

    #[test]
    fn correct_output() {
        assert_eq!(calc_total("./test-input"), 8);
    }
}
