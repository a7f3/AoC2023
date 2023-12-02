use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn do_calculation() {
    let mut list: Vec<u32> = Vec::new();
    let lines = read_lines("./input");
    for line in lines {
        let nums = parse_num(&line);
        list.push(
            format!("{}{}", nums[0], nums.last().unwrap())
                .parse()
                .unwrap(),
        );
    }

    println!("{}", list.into_iter().sum::<u32>());
}

pub fn parse_num(s: &str) -> Vec<u32> {
    let mut nums = Vec::<u32>::new();

    for (i, c) in s.chars().clone().enumerate() {
        if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap());
            continue;
        }

        /* the words in range zero to nine, are 3 to 5 characters long */
        for offset in 3..=5 {
            if i + offset > s.len() {
                break; /* break if index larger than string slice */
            }

            let c = &s[i..i + offset];

            match str_to_num(c) {
                Some(num) => {
                    nums.push(num);
                    break;
                }
                None => (),
            }
        }
    }

    nums
}

pub fn str_to_num(s: &str) -> Option<u32> {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, e) in words.iter().enumerate() {
        if s == *e {
            return Some(i as u32);
        }
    }
    None
}

fn main() {
    do_calculation();
}

#[cfg(test)]
mod tests {
    mod str_to_num {
        use crate::str_to_num;

        #[test]
        fn valid_words_0_to_9() {
            assert_eq!(str_to_num("zero").unwrap(), 0);
            assert_eq!(str_to_num("one").unwrap(), 1);
            assert_eq!(str_to_num("two").unwrap(), 2);
            assert_eq!(str_to_num("three").unwrap(), 3);
            assert_eq!(str_to_num("four").unwrap(), 4);
            assert_eq!(str_to_num("five").unwrap(), 5);
            assert_eq!(str_to_num("six").unwrap(), 6);
            assert_eq!(str_to_num("seven").unwrap(), 7);
            assert_eq!(str_to_num("eight").unwrap(), 8);
            assert_eq!(str_to_num("nine").unwrap(), 9);
        }

        #[test]
        fn valid_digits() {
            assert_eq!(str_to_num("0"), None);
            assert_eq!(str_to_num("1"), None);
            assert_eq!(str_to_num("2"), None);
            assert_eq!(str_to_num("3"), None);
            assert_eq!(str_to_num("4"), None);
            assert_eq!(str_to_num("5"), None);
            assert_eq!(str_to_num("6"), None);
            assert_eq!(str_to_num("7"), None);
            assert_eq!(str_to_num("8"), None);
            assert_eq!(str_to_num("9"), None);
        }

        #[test]
        fn two_valid_words() {
            assert_eq!(str_to_num("onetwo"), None);
        }
    }

    mod parse_num {
        use crate::parse_num;
        #[test]
        fn one_valid_word_and_digit() {
            assert_eq!(parse_num("1two"), vec![1, 2]);
            assert_eq!(parse_num("three4"), vec![3, 4]);
        }

        #[test]
        fn two_valid_word_and_digit() {
            assert_eq!(parse_num("1twothree4"), vec![1, 2, 3, 4]);
            assert_eq!(parse_num("five67eight"), vec![5, 6, 7, 8]);
        }

        #[test]
        fn two_valid_digits() {
            assert_eq!(parse_num("01"), vec![0, 1]);
            assert_eq!(parse_num("23"), vec![2, 3]);
            assert_eq!(parse_num("45"), vec![4, 5]);
            assert_eq!(parse_num("67"), vec![6, 7]);
            assert_eq!(parse_num("89"), vec![8, 9]);
        }

        #[test]
        fn two_valid_words() {
            assert_eq!(parse_num("onetwo"), vec![1, 2]);
            assert_eq!(parse_num("threefour"), vec![3, 4]);
            assert_eq!(parse_num("fivesix"), vec![5, 6]);
            assert_eq!(parse_num("seveneight"), vec![7, 8]);
            assert_eq!(parse_num("ninezero"), vec![9, 0]);
        }
    }
}
