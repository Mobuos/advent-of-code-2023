use std::collections::HashSet;

fn points(count: usize) -> usize {
    match count {
        c if c <= 0 => 0,
        _ => usize::pow(2, (count - 1).try_into().unwrap()),
    }
}

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = solver(input);
    dbg!(output);
}

fn solver(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line.split(':').last().unwrap().trim();
        let (winning, have) = numbers.split_once('|').unwrap();

        let winning: HashSet<i32> = winning
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        sum += points(
            have.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .filter(|num| winning.contains(num))
                .count(),
        );
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example.txt");
        let output = 13;
        assert_eq!(solver(input), output)
    }
}
