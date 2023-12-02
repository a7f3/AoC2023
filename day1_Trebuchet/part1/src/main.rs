use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let mut list: Vec<u32> = Vec::new();
    let lines = read_lines("./input");
    for line in lines {
        let nums: String = line.chars().filter(|c| c.is_numeric()).collect();
        let mut result = String::new();
        result.push(nums.clone().chars().next().unwrap());
        result.push(nums.clone().chars().last().unwrap());
        list.push(result.parse().unwrap());
    }

    println!("{}", list.into_iter().sum::<u32>());
}
