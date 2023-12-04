use core::panic;

struct Bag {
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    qtd_red: u32,
    qtd_blue: u32,
    qtd_green: u32,
}

impl Game {
    // Check if a game is valid for a given bag
    fn valid_for(&self, bag: &Bag) -> bool {
        self.qtd_blue <= bag.max_blue
            && self.qtd_green <= bag.max_green
            && self.qtd_red <= bag.max_red
    }

    fn max_from(input: &str) -> Game {
        // todo: Ideally this function would return a Result, instead of doing unwraps everywhere
        let (game_id, handfuls) = input.split_once(':').unwrap();
        let id = game_id
            .split_whitespace()
            .last()
            .unwrap()
            // .trim_end_matches(':')
            .parse::<u32>()
            .unwrap();

        let mut max_game = Game {
            id,
            qtd_red: 0,
            qtd_blue: 0,
            qtd_green: 0,
        };

        for handful in handfuls.split(';') {
            for part in handful.split(',').map(|p| p.trim()) {
                let (num, color) = part.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();
                match color {
                    "red" => max_game.qtd_red = max_game.qtd_red.max(num),
                    "blue" => max_game.qtd_blue = max_game.qtd_blue.max(num),
                    "green" => max_game.qtd_green = max_game.qtd_green.max(num),
                    _ => panic!("Something went wrong, what color is {}?", color),
                }
            }
        }

        max_game
    }
}

fn main() {
    let bag = Bag {
        max_red: 12,
        max_blue: 14,
        max_green: 13,
    };

    let input = include_str!("../input/puzzle_input.txt");
    let output = solver(&bag, input);
    dbg!(output);
}

fn solver(bag: &Bag, input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::max_from(line);
        if game.valid_for(bag) {
            sum += game.id;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            qtd_red: 4,
            qtd_blue: 6,
            qtd_green: 2,
        };
        assert_eq!(Game::max_from(input), expected);
    }

    #[test]
    fn valid_game_1() {
        let input = Game {
            id: 1,
            qtd_red: 4,
            qtd_blue: 6,
            qtd_green: 2,
        };
        let bag = Bag {
            max_red: 12,
            max_blue: 14,
            max_green: 13,
        };
        assert_eq!(input.valid_for(&bag), true);
    }

    #[test]
    fn valid_game_2() {
        let input = Game {
            id: 1,
            qtd_red: 13,
            qtd_blue: 6,
            qtd_green: 2,
        };
        let bag = Bag {
            max_red: 12,
            max_blue: 14,
            max_green: 13,
        };
        assert_eq!(input.valid_for(&bag), false);
    }

    #[test]
    fn solver_1() {
        let bag = Bag {
            max_red: 12,
            max_blue: 14,
            max_green: 13,
        };
        let input = include_str!("../input/example.txt");
        let output = 8;
        assert_eq!(solver(&bag, input), output);
    }
}
