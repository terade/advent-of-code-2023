use anyhow::anyhow;
use nom::{
    bytes::complete::is_a,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res},
    error::Error,
    multi::many1,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::{cmp::Ordering, collections::HashMap, fs, marker::PhantomData, str::FromStr};

const INPUT_FILE: &str = "input.sf";
const JOKER_VALUE: isize = 11; // you cant touch these
const NEW_JOKER_VALUE: isize = 1;

#[derive(Debug, PartialEq, Eq)]
struct Rule1 {}

#[derive(Debug, PartialEq, Eq)]
struct Rule2 {}

trait RuleSet {}
impl RuleSet for Rule1 {}
impl RuleSet for Rule2 {}

#[derive(Debug, PartialEq, Eq)]
struct Bet<T: RuleSet> {
    hand: [isize; 5],
    amount: isize,
    rule: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
struct BetRule2 {
    hand: [isize; 5],
    amount: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct CamelCards<T: RuleSet> {
    bets: Vec<Bet<T>>,
}

impl<T: RuleSet> CamelCards<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut parser = map(many1(preceded(multispace0, Bet::parse)), |bets| Self {
            bets,
        });

        parser(input)
    }

    fn from(bets: Vec<Bet<T>>) -> Self {
        Self { bets }
    }
}

impl<T: RuleSet> FromStr for CamelCards<T> {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Self::parse(input).finish() {
            Ok((_remaining, record)) => Ok(record),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl<T: RuleSet> Bet<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let hand = || is_a("AKQJT98765432");
        let mut parser = map_res(
            separated_pair(hand(), multispace1, map_res(digit1, isize::from_str)),
            |(hand_str, amount)| {
                if hand_str.len() != 5 {
                    return Err(anyhow!("invalid hand"));
                }
                let hand = hand_str
                    .chars()
                    .enumerate()
                    .try_fold([0; 5], |mut acc, (i, c)| {
                        acc[i] = match c {
                            c if c.is_ascii_digit() => c.to_digit(10).unwrap() as isize,
                            'T' => 10,
                            'J' => 11,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => return Err(anyhow!("should never reach")),
                        };

                        Ok(acc)
                    })?;

                Ok(Self {
                    hand,
                    amount,
                    rule: PhantomData::<T>,
                })
            },
        );
        parser(input)
    }

    fn from(hand: [isize; 5], amount: isize) -> Self {
        Self {
            hand,
            amount,
            rule: PhantomData::<T>,
        }
    }
}

impl Ord for Bet<Rule1> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Ord for Bet<Rule2> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Bet<Rule1> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hands: Vec<Vec<_>> = [self, other]
            .into_iter()
            .map(|bet| {
                bet.hand
                    .iter()
                    .fold(HashMap::<&isize, usize>::new(), |mut map, val| {
                        *map.entry(val).or_default() += 1;
                        map
                    })
                    .into_values()
                    .collect::<Vec<usize>>()
            })
            .map(|mut map| {
                map.sort_by(|a, b| b.cmp(a));
                map
            })
            .collect();

        Some(match hands[0].cmp(&hands[1]) {
            Ordering::Equal => self.hand.cmp(&other.hand),
            other => other,
        })
    }
}

impl Bet<Rule2> {
    fn revalue_joker(&self) -> Self {
        let mut hand = self.hand;
        for x in hand.iter_mut() {
            *x = if *x == JOKER_VALUE {
                NEW_JOKER_VALUE
            } else {
                *x
            };
        }
        Self {
            hand,
            amount: self.amount,
            rule: PhantomData::<Rule2>,
        }
    }
}

impl PartialOrd for Bet<Rule2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hands: Vec<Vec<_>> = [self.revalue_joker(), other.revalue_joker()]
            .into_iter()
            .map(|bet| {
                let mut map =
                    bet.hand
                        .iter()
                        .fold(HashMap::<&isize, usize>::new(), |mut map, val| {
                            *map.entry(val).or_default() += 1;
                            map
                        });
                let joker_frequency: usize = *map.get(&NEW_JOKER_VALUE).unwrap_or(&0);
                *map.entry(&NEW_JOKER_VALUE).or_default() = 0;
                let mut max_frequency_card = map
                    .clone()
                    .iter()
                    .map(|(a, b)| (**a, *b))
                    .collect::<Vec<(isize, usize)>>();
                // first is the id of the card second is frequency
                max_frequency_card.sort_by(|(c, a), (d, b)| (b, d).cmp(&(a, c)));

                let (id, _m) = max_frequency_card.first().unwrap();
                *map.entry(id).or_default() += joker_frequency;
                map.into_values().collect::<Vec<usize>>()
            })
            .map(|mut map| {
                map.sort_by(|a, b| b.cmp(a));
                map
            })
            .collect();

        Some(match hands[0].cmp(&hands[1]) {
            Ordering::Equal => self.revalue_joker().hand.cmp(&other.revalue_joker().hand),
            other => other,
        })
    }
}

fn solve<T: RuleSet>(input: &str) -> anyhow::Result<isize>
where
    Bet<T>: Ord,
{
    let CamelCards { mut bets } = input.parse::<CamelCards<T>>()?;
    bets.sort();
    Ok(bets
        .into_iter()
        .enumerate()
        .map(|(i, bet)| (i + 1) as isize * bet.amount)
        .sum())
}

fn part1(input: &str) -> anyhow::Result<isize> {
    solve::<Rule1>(input)
}

fn part2(input: &str) -> anyhow::Result<isize> {
    solve::<Rule2>(input)
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");
    println!("{}", part1(&input).unwrap());
    println!("{}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "32T3K 765
T55J5 684"
                .parse::<CamelCards<Rule1>>()
                .unwrap(),
            CamelCards::from(vec![
                Bet::from([3, 2, 10, 3, 13], 765),
                Bet::from([10, 5, 5, 11, 5], 684)
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )
            .unwrap(),
            6440
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )
            .unwrap(),
            5905
        );
        let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");

        assert_eq!(part2(&input).unwrap(), 254083736);
    }

    #[test]
    fn test_bet_cmp() {
        let bet1 = Bet::<Rule2>::from([11, 11, 11, 11, 11], 4);
        let bet2 = Bet::<Rule2>::from([11, 11, 11, 11, 10], 4);

        assert_eq!(bet1.cmp(&bet2), Ordering::Less);
    }
}
