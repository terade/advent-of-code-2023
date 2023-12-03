#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Round(pub usize, pub usize, pub usize); //red green blue

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

pub fn game(mut line: &str) -> Option<(usize, Vec<Round>)> {
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
