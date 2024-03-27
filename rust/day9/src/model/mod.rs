mod test;

use regex::Regex;
use std::str::FromStr;

pub type History = Vec<isize>;

#[derive(Debug, PartialEq, Eq)]
pub struct OasisSensor {
    histories: Vec<History>,
}

#[derive(Clone, Copy)]
enum ExtrapolateDirection {
    Forward,
    Backwards,
}

impl OasisSensor {
    pub fn parse(input: &str) -> Option<Self> {
        let lines = input.lines();
        let re = Regex::new(r"\s+").unwrap();

        let histories = lines
            .into_iter()
            .map(|line| {
                re.split(line)
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<isize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        Some(Self { histories })
    }

    pub fn new(histories: Vec<History>) -> Self {
        Self { histories }
    }

    fn predict(history: &History, extrapolate: ExtrapolateDirection) -> Option<isize> {
        if history.iter().all(|e| *e == 0) {
            return Some(0);
        }

        let differences: Vec<_> = history.windows(2).map(|s| s[1] - s[0]).collect();

        if differences.len() > 1 {
            Self::predict(&differences, extrapolate).and_then(|e| match extrapolate {
                ExtrapolateDirection::Forward => history.last().map(|t| e + t),
                ExtrapolateDirection::Backwards => history.first().map(|t| t - e),
            })
        } else {
            None
        }
    }

    pub fn part1(&self) -> Option<isize> {
        self.histories.iter().try_fold(0, |ac, e| {
            Self::predict(e, ExtrapolateDirection::Forward).map(|n| ac + n)
        })
    }

    pub fn part2(&self) -> Option<isize> {
        self.histories.iter().try_fold(0, |ac, e| {
            Self::predict(e, ExtrapolateDirection::Backwards).map(|n| ac + n)
        })
    }
}

impl FromStr for OasisSensor {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Self::parse(input) {
            Some(val) => Ok(val),
            _ => Err(()),
        }
    }
}
