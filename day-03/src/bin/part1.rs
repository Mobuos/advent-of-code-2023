use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = solver(input);
    dbg!(output);
}

fn solver(input: &str) -> i32 {
    // Store positions of each number and symbol on HashMaps
    let mut numbers: HashMap<(i32, i32), i32> = HashMap::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        let mut num: i32 = 0;

        for (x, cell) in row.chars().enumerate() {
            match cell {
                digit if digit.is_ascii_digit() => {
                    num *= 10;
                    num += digit.to_digit(10).unwrap() as i32;
                }
                _ => {
                    if num != 0 {
                        // Log of a number rounded up is it's length :D
                        insert_num(num, &mut numbers, (x as i32 - 1, y as i32));
                        num = 0;
                    }
                    if cell != '.' {
                        symbols.insert((x as i32, y as i32), cell);
                    }
                }
            }
        }
        // After checking the row we should add any remaining number
        if num != 0 {
            insert_num(num, &mut numbers, (row.len() as i32 - 1 as i32, y as i32));
        }
    }
    let numbers = dbg!(numbers);

    // Unnecessary, there are repeated part numbers, but no part number is connected to more
    // than one symbol at a time (ok???)
    // let mut used_parts: HashMap<i32, ()> = HashMap::new();
    let mut sum = 0;

    // For every symbol, check for part numbers
    for (symbol_position, _) in symbols {
        for num in get_neighbours(symbol_position, &numbers).into_iter() {
            println!("Adding {}", num);
            sum += num;
        }
    }

    sum
}

fn get_neighbours((x, y): (i32, i32), map: &HashMap<(i32, i32), i32>) -> Vec<i32> {
    let mut neighbours: Vec<i32> = Vec::new();
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if let Some(num) = map.get(&(x + dx, y + dy)) {
                neighbours.push(num.clone());
            }
        }
    }
    neighbours.dedup(); // Because we're adding numbers row by row, this should work
    neighbours
}

fn insert_num(num: i32, map: &mut HashMap<(i32, i32), i32>, (x, y): (i32, i32)) {
    for i in 0..(num.ilog10() as i32 + 1) {
        map.insert((x - i, y), num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../input/example.txt");
        let output = 4361;
        assert_eq!(solver(input), output);
    }
}

// Liberal usage of "as" because the puzzle won't ever exceed i32 anyways
// Ideally use TryFrom
