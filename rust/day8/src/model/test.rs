#[cfg(test)]
use super::*;

#[test]
fn parse_test() {
    let map_test0 = Map::from(
        vec![Direction::Right, Direction::Left],
        hash_map! {
            ['A'; 3] => (['B'; 3], ['C'; 3]),
            ['B'; 3] => (['D'; 3], ['E'; 3]),
        },
    );
    let map_test1 = Map::from(
        vec![Direction::Right, Direction::Left],
        hash_map! {
            ['A'; 3] => (['B'; 3], ['C'; 3]),
            ['B'; 3] => (['D'; 3], ['E'; 3]),
            ['C'; 3] => (['Z'; 3], ['G'; 3]),
        },
    );
    assert_eq!(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)"
            .parse::<Map>()
            .unwrap(),
        map_test0
    );
    assert_eq!(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)"
            .parse::<Map>()
            .unwrap(),
        map_test1
    );
}

#[test]
fn part1_test() {
    assert_eq!(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .parse::<Map>()
            .unwrap()
            .part1(),
        2
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

"
        .parse::<Map>()
        .unwrap()
        .part2(),
        6
    );
}
