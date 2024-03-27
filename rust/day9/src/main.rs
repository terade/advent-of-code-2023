mod model;

use model::OasisSensor;
use std::fs;

const INPUT_FILE: &str = "input.sf";

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .unwrap_or_else(|_| panic!("Could not read from file {INPUT_FILE}"));

    println!(
        "part1: {}",
        input
            .parse::<OasisSensor>()
            .unwrap()
            .part1()
            .expect("invalid input")
    );
    println!(
        "part2: {}",
        input
            .parse::<OasisSensor>()
            .unwrap()
            .part2()
            .expect("invalid input")
    );
}
