use regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/puzzle_input.txt");
    let output = part2(input);
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

fn part2(input: &str) -> i64 {
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
    let seeds: Vec<&str> = inputs.next().unwrap().split_whitespace().collect();

    // todo: This is too many seeds, convert to a representation such as (start, len)

    for slice in seeds.chunks(2) {
        let seed_start: i64 = slice.get(0).unwrap().parse().unwrap();
        let seed_len: i64 = slice
            .get(1)
            .expect("Expected pairs of seeds")
            .parse()
            .unwrap();
        for seed in seed_start..=seed_start + seed_len {
            seed_map.insert(seed, Vec::new());
        }
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
        let result = part2(input);
        assert_eq!(result, 46);
    }
}
