use std::cmp;
use std::fs;

const INPUT_FILE: &str = "input.sf"; // sf some format
const BAG: (usize, usize, usize) = (12, 13, 14); // red green blue

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round(usize, usize, usize); //red green blue

#[derive(Debug, PartialEq, Eq)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Round {
    fn insert(&self, cube: Cube) -> Round {
        let Round(mut red, mut green, mut blue) = self;
        if let Cube::Red(count) = cube {
            red = count;
        }
        if let Cube::Green(count) = cube {
            green = count;
        }
        if let Cube::Blue(count) = cube {
            blue = count;
        }

        Round(red, green, blue)
    }
}

fn game(mut line: &str) -> Option<(usize, Vec<Round>)> {
    let mut rounds = Vec::new();
    expect(&mut line, "Game ")?;
    let game_id = number(&mut line)?;
    expect(&mut line, ":")?;
    loop {
        let round = round(&mut line)?;
        rounds.push(round);
        if let None = expect(&mut line, ";") {
            break;
        }
    }

    Some((game_id, rounds))
}

fn expect(input: &mut &str, string: &str) -> Option<()> {
    if input.starts_with(string) {
        *input = &input[string.len()..];
        return Some(());
    }
    None
}

fn number(input: &mut &str) -> Option<usize> {
    let mut num = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            break;
        }
    }

    if !num.is_empty() {
        *input = &input[num.len()..];
        return Some(num.parse::<usize>().unwrap());
    }

    None
}

fn skip_whitespace(input: &mut &str) {
    while let Some(_) = expect(input, " ") {}
}

fn cube(input: &mut &str) -> Option<Cube> {
    let mut cube = None;
    let count = number(input)?;
    skip_whitespace(input);

    if let Some(_) = input.strip_prefix("blue") {
        *input = &input[4..];
        cube = Some(Cube::Blue(count));
    } else if let Some(_) = input.strip_prefix("red") {
        *input = &input[3..];
        cube = Some(Cube::Red(count));
    } else if let Some(_) = input.strip_prefix("green") {
        *input = &input[5..];
        cube = Some(Cube::Green(count));
    }

    cube
}

fn round(input: &mut &str) -> Option<Round> {
    let mut round = Some(Round(0, 0, 0));
    loop {
        skip_whitespace(input);
        let cube = cube(input)?;
        round = round.map(|r| r.insert(cube));
        skip_whitespace(input);

        if let None = expect(input, ",") {
            break;
        }
    }

    round
}

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
