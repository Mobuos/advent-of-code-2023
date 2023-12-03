fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut first_char = 'a';
        let mut last_char = 'a';
        for c in line.chars() {
            if first_char == 'a' {
                if c.is_numeric() {
                    first_char = c;
                }
            } else {
                if c.is_numeric() {
                    last_char = c;
                }
            }
        }
        if last_char == 'a' {
            last_char = first_char;
        }
        // dbg!(format!("{first_char}{last_char}"));
        result += format!("{first_char}{last_char}").parse::<i32>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = include_str!("../input/test1.txt");
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
