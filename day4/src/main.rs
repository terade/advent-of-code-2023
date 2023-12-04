use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, map_res},
    multi::many1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};
use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

const INPUT_FILE: &str = "input.sf";

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scratchcard {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: Vec<usize>,
}

impl Scratchcard {
    fn parse(input: &str) -> IResult<&str, Self> {
        let id = preceded(
            pair(tag("Card"), multispace1),
            terminated(digit1, char(':')),
        );
        let scratchcard_id = map_res(id, usize::from_str);
        let number = || map_res(digit1, usize::from_str);
        let number_list = || many1(preceded(multispace1, number()));
        let value_parser =
            separated_pair(number_list(), pair(multispace0, char('|')), number_list());
        let mut scratchcard_parser = map(
            pair(scratchcard_id, value_parser),
            |(id, (winning_numbers, numbers))| Self {
                id,
                winning_numbers: winning_numbers.into_iter().collect(),
                numbers,
            },
        );
        scratchcard_parser(input)
    }

    fn from(id: usize, winning_numbers: HashSet<usize>, numbers: Vec<usize>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn calculate_wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|elem| self.winning_numbers.contains(elem))
            .collect::<Vec<_>>()
            .len()
    }
}

fn get_scratchcards(input: &str) -> Option<Vec<Scratchcard>> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let scratchcard = match Scratchcard::parse(line) {
            Ok((_, card)) => card,
            Err(_) => return None,
        };
        cards.push(scratchcard);
    }

    Some(cards)
}

fn part1(input: &str) -> Option<usize> {
    let cards = get_scratchcards(input)?;

    let sum = cards.into_iter().fold(0, |acc, card| {
        let wins = card.calculate_wins();
        (if wins == 0 {
            0
        } else {
            (2_usize).pow((wins - 1) as u32)
        }) + acc
    });

    Some(sum as usize)
}

fn part2(input: &str) -> Option<usize> {
    let cards = get_scratchcards(input)?;

    let repeats = cards
        .iter()
        .map(|card| card.calculate_wins())
        .collect::<Vec<_>>();

    let mut copies = vec![1; repeats.len()];

    Some(repeats.into_iter().enumerate().fold(0, |acc, (i, wins)| {
        for j in i + 1..cmp::min(i + wins + 1, copies.len()) {
            copies[j] += copies[i];
        }
        acc + copies[i]
    }))
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");

    println!("part1: {}", part1(&input).unwrap());
    println!("part2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        assert_eq!(
            Scratchcard::parse("Card 2: 10 20 30 | 10 7"),
            Ok((
                "",
                Scratchcard::from(2, vec![10, 20, 30].into_iter().collect(), vec![10, 7])
            ))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            Some(13)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            Some(30)
        );
    }
}
