mod model;
use model::*;
use std::fs;

const INPUT_FILE: &str = "input.sf";

fn part1(input: &str) -> Option<isize> {
    let almanac = match input.parse::<Almanac>() {
        Ok(almanac) => almanac,
        _ => return None,
    };
    let mut seeds = almanac.seeds;
    let mut source = "seed";

    while let Some(map) = almanac.maps.get(source) {
        seeds = seeds.into_iter().map(|seed| map.translate(seed)).collect();
        source = &map.destination;
    }

    seeds.into_iter().min()
}

fn part2(input: &str) -> Option<isize> {
    let almanac = match input.parse::<Almanac>() {
        Ok(almanac) => almanac,
        _ => return None,
    };

    let mut source = "seed";
    let mut seed_ranges = almanac.get_seed_ranges()?;

    while let Some(map) = almanac.maps.get(source) {
        seed_ranges = seed_ranges
            .into_iter()
            .flat_map(|seed_range| map.translate_range(seed_range))
            .collect();
        source = &map.destination;
    }

    seed_ranges.into_iter().map(|(start, _)| start).min()
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("couldnt read from file");
    println!("part1: {}", part1(&input).expect("input is not correct"));
    println!("part2: {}", part2(&input).expect("input is not correct"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            part1(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            Some(35)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4"
            ),
            Some(46)
        );
    }
}
