use std::cmp;
use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "engine.sf";

#[derive(Debug, PartialEq, Eq)]
struct Engine {
    part_numbers: Vec<Vec<(usize, usize)>>, // index, value
    parts: Vec<(usize, usize)>,             // line offset
}

impl Engine {
    fn from(part_numbers: Vec<Vec<(usize, usize)>>, parts: Vec<(usize, usize)>) -> Self {
        Self {
            part_numbers,
            parts,
        }
    }
}

fn parse(input: &str) -> Engine {
    let mut part_numbers = Vec::new();
    let mut parts = Vec::new();
    for (line_count, line) in input.lines().enumerate() {
        let mut line_part_numbers: Vec<(usize, usize)> = Vec::new();

        let mut num = String::new();
        let mut column_count = 0;
        for (offset, c) in line.char_indices() {
            column_count = offset;
            if c.is_ascii_digit() {
                num.push(c);
            } else if !num.is_empty() {
                let start = offset - num.len();
                line_part_numbers.push((start, num.parse().unwrap()));
                num = String::new();
            }
            if !c.eq(&'.') && !c.is_ascii_digit() {
                parts.push((line_count, offset));
            }
        }
        if !num.is_empty() {
            let start = column_count - num.len();
            line_part_numbers.push((start, num.parse().unwrap()));
        }

        part_numbers.push(line_part_numbers);
    }

    Engine::from(part_numbers, parts)
}

fn adjacent_part(position: (usize, usize)) -> HashSet<(usize, usize)> {
    let (line, offset) = position;
    let mut set = HashSet::new();

    for y in line.saturating_sub(1)..=line + 1 {
        for x in offset.saturating_sub(1)..=offset + 1 {
            set.insert((y, x));
        }
    }

    set
}

fn place_part_number(part_number: (usize, usize), line: usize) -> HashSet<(usize, usize)> {
    let (offset, value) = part_number;
    let mut set = HashSet::new();
    let length = value.to_string().len();

    for x in offset..offset + length {
        set.insert((line, x));
    }

    set
}

fn part1(input: &str) -> Option<usize> {
    let engine = parse(input);
    let last_line = engine.part_numbers.len();
    let mut valid_numbers = HashSet::new();

    for (line, offset) in engine.parts {
        for i in line.saturating_sub(1)..=cmp::min(last_line - 1, line + 1) {
            let part_numbers = engine.part_numbers.get(i)?;

            for (offset_number, value) in part_numbers {
                let adjacent_part = adjacent_part((line, offset));
                let adjacent_number = place_part_number((*offset_number, *value), i);

                if !adjacent_part.is_disjoint(&adjacent_number) {
                    valid_numbers.insert((i, offset_number, value));
                }
            }
        }
    }
    Some(
        valid_numbers
            .into_iter()
            .fold(0, |acc, (_, _, value)| acc + value),
    )
}

fn part2(input: &str) -> Option<usize> {
    let engine = parse(input);
    let last_line = engine.part_numbers.len();
    let mut valid_numbers = HashSet::new();
    let mut sum = 0;

    for (line, offset) in engine.parts {
        for i in line.saturating_sub(1)..=cmp::min(last_line - 1, line + 1) {
            let part_numbers = engine.part_numbers.get(i)?;

            for (offset_number, value) in part_numbers {
                let adjacent_part = adjacent_part((line, offset));
                let adjacent_number = place_part_number((*offset_number, *value), i);

                if !adjacent_part.is_disjoint(&adjacent_number) {
                    valid_numbers.insert((i, offset_number, value));
                }
            }
        }

        if valid_numbers.len() == 2 {
            sum += valid_numbers
                .iter()
                .fold(1, |acc, (_, _, value)| acc * **value);
        }
        valid_numbers.clear();
    }
    Some(sum)
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");

    println!("part1 {}", part1(&input).unwrap_or(0));
    println!("part2 {}", part2(&input).unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test() {
        assert_eq!(
            parse(
                "467..114..
...*......
..35..633."
            ),
            Engine::from(
                vec![vec![(0, 467), (5, 114)], vec![], vec![(2, 35), (6, 633)]],
                vec![(1, 3)]
            ),
        );

        assert_eq!(
            part1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .unwrap(),
            4361
        );

        assert_eq!(
            part2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .unwrap(),
            467835
        );
    }
}
