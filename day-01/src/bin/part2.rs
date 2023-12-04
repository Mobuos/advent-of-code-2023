use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref WORD_TO_NUM: HashMap<&'static str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]
    .iter()
    .copied()
    .collect();
}

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = solver(input);
    dbg!(output);
}

// Takes a line and gets the combination of it's first and last value
fn combine_line(input: &str) -> u32 {
    let (mut first, mut last) = (None, None);
    let mut stack: String = String::new();

    let mut set_nums = |num: u32| {
        if first.is_none() {
            first = Some(num);
        } else {
            last = Some(num);
        }
    };

    let word_to_num: &HashMap<&'static str, u32> = &WORD_TO_NUM;
    for c in input.chars() {
        // Reset stack if we hit a digit, otherwise increase it
        if let Some(c_digit) = c.to_digit(10) {
            set_nums(c_digit);
            stack.clear();
        } else {
            stack.push(c);
            for (&word, &num) in word_to_num {
                if stack.contains(word) {
                    set_nums(num);
                    stack.clear();
                    stack.push(c);
                }
            }
        }
    }

    let first = first.expect("Failed to get first digit");
    let last = last.unwrap_or(first);
    first * 10 + last
}

// Solves the problem for a given input
fn solver(input: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input.lines() {
        dbg!(combine_line(line));
        result += combine_line(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_nums {
        use super::*;

        #[test]
        fn test_to_nums_0() {
            let input = "12345";
            assert_eq!(combine_line(input), 15);
        }

        #[test]
        fn test_to_nums_1() {
            let input = "two1nine";
            assert_eq!(combine_line(input), 29);
        }

        #[test]
        fn test_to_nums_2() {
            let input = "eightwothree";
            assert_eq!(combine_line(input), 83);
        }

        #[test]
        fn test_to_nums_3() {
            let input = "abcone2threexyz";
            assert_eq!(combine_line(input), 13);
        }

        #[test]
        fn test_to_nums_4() {
            let input = "xtwone3four";
            assert_eq!(combine_line(input), 24);
        }

        #[test]
        fn test_to_nums_5() {
            let input = "4nineeightseven2";
            assert_eq!(combine_line(input), 42);
        }

        #[test]
        fn test_to_nums_6() {
            let input = "zoneight234";
            assert_eq!(combine_line(input), 14);
        }

        #[test]
        fn test_to_nums_7() {
            let input = "7pqrstsixteen";
            assert_eq!(combine_line(input), 76);
        }
    }

    #[test]
    fn example_1() {
        let input = include_str!("../input/test2.txt");
        let result = solver(input);
        assert_eq!(result, 281);
    }
}
