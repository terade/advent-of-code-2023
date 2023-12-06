mod parser;

use crate::parser::{game, Round};
use std::cmp;
use std::fs;

const INPUT_FILE: &str = "input.sf"; // sf some format
const BAG: (usize, usize, usize) = (12, 13, 14); // red green blue

fn get_games(input: &String) -> Option<Vec<(usize, Round)>> {
    let mut games = Vec::new();
    for line in input.lines() {
        let (id, game) = game(line)?;
        let max = game
            .into_iter()
            .reduce(|Round(red1, green1, blue1), Round(red2, green2, blue2)| {
                Round(
                    cmp::max(red1, red2),
                    cmp::max(green1, green2),
                    cmp::max(blue1, blue2),
                )
            })
            .unwrap();
        games.push((id, max));
    }
    Some(games)
}

fn part1(input: &String, bag: (usize, usize, usize)) -> Option<usize> {
    let games = get_games(input)?;
    let (bred, bgreen, bblue) = bag;
    let result = games
        .into_iter()
        .filter(|(_, max)| {
            let Round(red, green, blue) = max;
            *red <= bred && *green <= bgreen && *blue <= bblue
        })
        .fold(0, |acc, (id, _)| acc + id);

    Some(result)
}

fn part2(input: &String) -> Option<usize> {
    let games = get_games(input)?;
    let result = games
        .into_iter()
        .map(|(_, min)| {
            let Round(red, green, blue) = min;
            red * green * blue
        })
        .sum();

    Some(result)
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");
    println!("part1: {}", part1(&input, BAG).unwrap_or(0));
    println!("part2: {}", part2(&input).unwrap_or(0));
}
