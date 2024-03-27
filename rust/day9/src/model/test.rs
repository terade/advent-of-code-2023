use super::*;

#[test]
fn parse_test() {
    let test = "1 2 3 4 5
        2 3 4 5 6";

    assert_eq!(
        test.parse::<OasisSensor>(),
        Ok(OasisSensor::new(vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6]
        ]))
    );
}

#[test]
fn part1_test() {
    let test = "0   3   6   9  12  15";
    let oasis = test.parse::<OasisSensor>().unwrap();

    let test1 = "1   3   6  10  15  21";
    let oasis1 = test1.parse::<OasisSensor>().unwrap();

    let test2 = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    let oasis2 = test2.parse::<OasisSensor>().unwrap();

    assert_eq!(oasis.part1(), Some(18));
    assert_eq!(oasis1.part1(), Some(28));
    assert_eq!(oasis2.part1(), Some(114));
}

#[test]
fn part2_test() {
    let test = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    let oasis = test.parse::<OasisSensor>().unwrap();
    let test1 = "10  13  16  21  30  45";
    let oasis1 = test1.parse::<OasisSensor>().unwrap();

    assert_eq!(oasis1.part2(), Some(5));
    assert_eq!(oasis.part2(), Some(2));
}
