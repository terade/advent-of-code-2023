mod test;

use nom::{
    branch::alt,
    character::complete::{char, multispace1, satisfy},
    combinator::map,
    error::Error,
    multi::{many0, many1, many_m_n},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    str::FromStr,
};

const START_NODE: Node = ['A'; 3];
const END_NODE: Node = ['Z'; 3];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

type Node = [char; 3];

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    directions: Vec<Direction>,
    network: HashMap<Node, (Node, Node)>,
}

impl Direction {
    fn choose(&self, fork: (Node, Node)) -> Node {
        let (left, right) = fork;
        match self {
            Direction::Left => left,
            Direction::Right => right,
        }
    }
}

impl Map {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut network = HashMap::new();
        let direction = map(alt((char('L'), char('R'))), |c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("fundamental error in the map parser"),
        });
        let directions = many1(direction);
        let a_to_z = || satisfy(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
        let node = || map(many_m_n(3, 3, a_to_z()), |v| v.clone().try_into().unwrap());
        let many_whitespaces_not_newline = || many0(satisfy(|c| c.is_whitespace() && !c.eq(&'\n')));
        let entry = separated_pair(
            node(),
            tuple((
                many_whitespaces_not_newline(),
                char('='),
                many_whitespaces_not_newline(),
            )),
            delimited(
                char('('),
                separated_pair(
                    delimited(
                        many_whitespaces_not_newline(),
                        node(),
                        many_whitespaces_not_newline(),
                    ),
                    char(','),
                    delimited(
                        many_whitespaces_not_newline(),
                        node(),
                        many_whitespaces_not_newline(),
                    ),
                ),
                char(')'),
            ),
        );

        let mut parser = pair(directions, many1(preceded(multispace1, entry)));

        match parser(input) {
            Ok((str, (directions, vec))) => {
                for (key, value) in vec {
                    network.insert(key, value);
                }
                Ok((
                    str,
                    Self {
                        directions,
                        network,
                    },
                ))
            }
            Err(err) => Err(err),
        }
    }

    fn from(directions: Vec<Direction>, network: HashMap<Node, (Node, Node)>) -> Self {
        Self {
            directions,
            network,
        }
    }

    fn steps(&self, begin: Node, ends: HashSet<Node>) -> usize {
        let mut node = begin;
        let mut count = 0;

        loop {
            let direction = self.directions[count % self.directions.len()];
            let fork = self.network.get(&node).unwrap();
            node = direction.choose(*fork);

            count += 1;

            if ends.contains(&node) {
                break;
            }
        }

        count
    }

    pub fn part1(&self) -> usize {
        self.steps(START_NODE, HashSet::from([END_NODE]))
    }

    pub fn part2(&self) -> usize {
        self.network
            .keys()
            .filter(|a| a[2] == 'A')
            .map(|p| {
                self.steps(
                    *p,
                    self.network
                        .keys()
                        .filter_map(|z| if z[2] == 'Z' { Some(*z) } else { None })
                        .collect(),
                )
            })
            .fold(1, lcm)
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

impl FromStr for Map {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Self::parse(input).finish() {
            Ok((_remaining, map)) => Ok(map),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}
