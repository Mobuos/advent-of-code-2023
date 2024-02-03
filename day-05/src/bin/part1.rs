use regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = part1(input);
    dbg!(output);
}

struct MapLine {
    dest: i64,
    source: i64,
    length: i64,
}

fn mapper(source: &i64, map: &Vec<MapLine>) -> i64 {
    for line in map {
        let source_range = line.source..=line.source + line.length;
        if source_range.contains(&source) {
            // Calculate conversion source -> dest by doing dest - source
            return source + (line.dest - line.source);
        }
    }
    source.clone()
}

fn make_vec_mapline(input: &str) -> Vec<MapLine> {
    input
        .lines()
        .map(|lines| {
            let mut values = lines.trim().split_whitespace();
            let dest = values
                .next()
                .expect("Missing destination")
                .parse()
                .expect("Invalid destination");
            let source = values
                .next()
                .expect("Missing source")
                .parse()
                .expect("Invalid source");
            let length = values
                .next()
                .expect("Missing length")
                .parse()
                .expect("Invalid length");
            MapLine {
                dest,
                source,
                length,
            }
        })
        .collect::<Vec<MapLine>>()
}

fn part1(input: &str) -> i64 {
    // Separate Maps
    let re = regex::Regex::new(
        r"seeds:|seed-to-soil map:|soil-to-fertilizer map:|fertilizer-to-water map:|fertilizer-to-water map:|water-to-light map:|light-to-temperature map:|temperature-to-humidity map:|humidity-to-location map:",
    ).unwrap();

    let mut inputs = re
        .split(input)
        .map(|x| x.trim())
        .skip_while(|x| x.is_empty());

    // Get seeds
    let mut seed_map: HashMap<i64, Vec<i64>> = HashMap::new();
    let seeds = inputs.next().unwrap();
    for seed in seeds.split(' ') {
        seed_map.insert(seed.parse().unwrap(), Vec::new());
    }

    let maps: Vec<Vec<MapLine>> = inputs.map(|m| make_vec_mapline(m)).collect();

    for (seed, categories) in &mut seed_map {
        // Add seed to the start of its category vector
        categories.push(seed.clone());

        // Map all categories
        for map in &maps {
            let last = categories.last().unwrap();
            categories.push(mapper(last, map));
        }
    }

    // Find lowest location
    seed_map
        .values()
        .map(|x| x.last().unwrap())
        .min()
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = include_str!("../input/test1.txt");
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn mapper_1() {
        let seed = 14;
        let map_to_soil = make_vec_mapline("50 98 2\n52 50 48");
        let result = mapper(&seed, &map_to_soil);
        assert_eq!(result, 14);
    }

    #[test]
    fn mapper_2() {
        let seed = 79;
        let map_to_soil: Vec<MapLine> = make_vec_mapline("50 98 2\n52 50 48");
        let result = mapper(&seed, &map_to_soil);
        assert_eq!(result, 81);
    }
}
