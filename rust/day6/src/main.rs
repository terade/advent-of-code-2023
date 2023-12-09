use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, satisfy},
    combinator::map_res,
    error::Error,
    multi::many0,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::fs;
use std::iter;
use std::str::FromStr;

const INPUT_FILE: &str = "input.sf";

#[derive(Debug, PartialEq, Eq)]
struct Records {
    time_distance: Vec<(isize, isize)>,
}

impl Records {
    fn parse(input: &str) -> IResult<&str, Self> {
        let number = || map_res(digit1, isize::from_str);
        let many_whitespaces_not_newline = || many0(satisfy(|c| c.is_whitespace() && !c.eq(&'\n')));

        let number_list = || many0(preceded(many_whitespaces_not_newline(), number()));
        let time = preceded(tag("Time:"), number_list());
        let distance = preceded(tag("Distance:"), number_list());

        let time_distance = separated_pair(time, multispace1, distance);

        let mut records_parser = map_res(time_distance, |(time, distance)| {
            if time.len() != distance.len() {
                Err(())
            } else {
                let time_distance = iter::zip(time, distance).collect();
                Ok(Self { time_distance })
            }
        });

        records_parser(input)
    }

    fn from(time_distance: Vec<(isize, isize)>) -> Self {
        Self { time_distance }
    }
}

impl FromStr for Records {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Records::parse(input).finish() {
            Ok((_remaining, record)) => Ok(record),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn abc_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = (b * b - 4_f64 * a * c).sqrt();
    ((-b - d) / (2_f64 * a), ((-b + d) / (2_f64 * a)))
}

fn ways_to_win(records: Vec<(isize, isize)>) -> isize {
    records
        .into_iter()
        .map(|(time, distance)| {
            let e = 0.00001;
            let (first, second) = abc_formula(-1_f64, time as f64, -distance as f64);
            ((first - e).floor() - (second + e).ceil()) as isize + 1
        })
        .product()
}

fn part1(input: &str) -> Result<isize, anyhow::Error> {
    let records: Records = input.parse()?;
    Ok(ways_to_win(records.time_distance))
}

fn part2(input: &str) -> Result<isize, anyhow::Error> {
    let records: Records = input.parse()?;
    let records = vec![records
        .time_distance
        .into_iter()
        .map(|(t, d)| (t.to_string(), d.to_string()))
        .reduce(|(mut t_acc, mut d_acc), (t, d)| {
            t_acc.push_str(&t);
            d_acc.push_str(&d);
            (t_acc, d_acc)
        })
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .ok_or(anyhow!("no races recorded"))?];
    Ok(ways_to_win(records))
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("couldn't read from file");
    println!("{}", part1(&input).expect("error in file format"));
    println!("{}", part2(&input).expect("error in file format"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            "Time:      7  15   30
Distance:  9  40  200"
                .parse::<Records>()
                .unwrap(),
            Records::from(vec![(7, 9), (15, 40), (30, 200)])
        );

        assert_eq!(
            part1(
                "Time:      7  15   30
Distance:  9  40  200
"
            )
            .unwrap(),
            288
        );
    }
}
