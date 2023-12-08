use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, multispace1, newline, satisfy},
    combinator::{map, map_res, opt},
    error::Error,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub source: String,
    pub destination: String,
    pub map: Vec<(isize, isize, isize)>, //dest start, dest start + range, offset
}

impl Map {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let source_destination = || {
            map(
                terminated(
                    separated_pair(preceded(multispace0, alpha1), tag("-to-"), alpha1),
                    pair(multispace1, tag("map:")),
                ),
                |(source, destination): (&str, &str)| (source.to_string(), destination.to_string()),
            )
        };
        let number = || map_res(digit1, isize::from_str);
        let value_parser = || {
            map(
                tuple((
                    preceded(multispace0, number()),
                    preceded(multispace1, number()),
                    preceded(multispace1, number()),
                )),
                |(dest_start, source_start, range)| {
                    (
                        source_start,
                        source_start + range - 1,
                        dest_start - source_start,
                    )
                },
            )
        };
        let many_whitespaces_not_newline = || many0(satisfy(|c| c.is_whitespace() && !c.eq(&'\n')));

        let tuples_parser = || {
            many1(terminated(
                value_parser(),
                pair(many_whitespaces_not_newline(), opt(newline)),
            ))
        };
        let mut map_parser = map(
            tuple((
                terminated(
                    source_destination(),
                    pair(many_whitespaces_not_newline(), newline),
                ),
                tuples_parser(),
            )),
            |((source, destination), map)| Self {
                source,
                destination,
                map,
            },
        );

        map_parser(input)
    }

    pub fn translate(&self, id: isize) -> isize {
        for (source_start, source_end, shift) in self.map.iter() {
            if *source_start <= id && id <= *source_end {
                return id + *shift;
            }
        }
        id
    }

    pub fn translate_range(&self, range: (isize, isize)) -> Vec<(isize, isize)> {
        let mut new_ranges = Vec::new();
        let mut unprocessed_ranges = vec![range];
        let mut new_unprocessed_ranges;

        for (source_start, source_end, offset) in self.map.iter() {
            new_unprocessed_ranges = Vec::new();

            for (start, end) in unprocessed_ranges.iter() {
                let (new_start, new_end) =
                    (cmp::max(*start, *source_start), cmp::min(*end, *source_end)); //overlap

                if new_start <= new_end {
                    new_ranges.push((new_start + *offset, new_end + *offset));

                    new_unprocessed_ranges.append(
                        &mut vec![(*start, new_start - 1), (new_end + 1, *end)]
                            .into_iter()
                            .filter(|(start, end)| start <= end)
                            .collect(),
                    );
                } else {
                    new_unprocessed_ranges.push((*start, *end));
                }
            }

            unprocessed_ranges = new_unprocessed_ranges;
        }

        new_ranges.append(&mut unprocessed_ranges);
        new_ranges
    }
}

impl FromStr for Map {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Map::parse(input).finish() {
            Ok((_remaining, map)) => Ok(map),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<isize>,
    pub maps: HashMap<String, Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let number = || map_res(digit1, isize::from_str);
        let seeds = delimited(
            tag("seeds:"),
            many0(preceded(multispace1, number())),
            newline,
        );
        let maps = many0(delimited(multispace0, Map::parse, multispace0));
        let mut almanac_parser = map(pair(seeds, maps), |(seeds, maps)| {
            let mut maps_map = HashMap::new();

            for map in maps.into_iter() {
                let key = map.source.clone();
                maps_map.insert(key, map);
            }

            Self {
                seeds,
                maps: maps_map,
            }
        });
        almanac_parser(input)
    }

    pub fn get_seed_ranges(&self) -> Option<Vec<(isize, isize)>> {
        if self.seeds.len() % 2 != 0 {
            return None;
        }

        Some(
            self.seeds
                .chunks(2)
                .map(|vec| (vec[0], vec[0] + vec[1] - 1))
                .collect(),
        )
    }
}

impl FromStr for Almanac {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Almanac::parse(input).finish() {
            Ok((_remaining, almanac)) => Ok(almanac),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
