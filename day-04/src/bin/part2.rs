use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = solver(input);
    dbg!(output);
}

fn solver(input: &str) -> usize {
    let mut card_to_matches: HashMap<usize, usize> = HashMap::new();
    let mut card_qtd: HashMap<usize, usize> = HashMap::new();
    let mut sum_cards = 0;

    for line in input.lines() {
        let card_num = get_card_num(line);

        card_to_matches.insert(card_num, get_qtd_matches(line));
        card_qtd.insert(card_num, 1);

        sum_cards += 1; // Accounts for the original cards
    }

    let card_max = sum_cards;

    // Is there a more rusty way to do this?
    for card in 1..=card_qtd.len() {
        if card > card_max {
            continue;
        }
        let qtd_to_add = card_qtd.get(&card).unwrap().clone();

        for card_to_add in (card + 1)..=(card + card_to_matches.get(&card).unwrap()) {
            let current_qtd = card_qtd.get(&card_to_add).unwrap();

            card_qtd.insert(card_to_add, current_qtd + qtd_to_add);
            sum_cards += qtd_to_add;
        }
    }

    sum_cards
}

fn get_card_num(card: &str) -> usize {
    card.split_once(':')
        .expect("Could not find ':' delimiter")
        .0
        .split_whitespace()
        .last()
        .expect("Could not find game number (Maybe missing a space between game and it's number?)")
        .parse::<usize>()
        .expect("Could not parse game number into an integer")
}

fn get_qtd_matches(card: &str) -> usize {
    let numbers = card.split(':').last().unwrap().trim();
    let (winning, have) = numbers.split_once('|').unwrap();

    let winning: HashSet<usize> = winning
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    have.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|num| winning.contains(num))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example.txt");
        let output = 30;
        assert_eq!(solver(input), output)
    }
}
