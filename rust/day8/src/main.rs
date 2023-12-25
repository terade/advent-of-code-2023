mod model;

use model::*;
use std::fs;

const INPUT_FILE: &str = "input.sf";

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .unwrap_or_else(|_| panic!("could not read from file {}", INPUT_FILE));

    let part1 = input.parse::<Map>().unwrap().part1();
    let part2 = input.parse::<Map>().unwrap().part2();
    println!("part1: {}\npart2: {}", part1, part2);
}
