use std::fs;

const INPUT_FILE: &str = "calibration.lsv";

const TRANSLATION: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read input file");

    println!("Starting calibration ...");
    println!("First: {}", first(&input));
    println!("Adjusting calibration ...");
    println!("Second: {}", second(&input, &TRANSLATION));
}

fn first(input: &str) -> usize {
    input.split_whitespace().fold(0, |acc, line| {
        let numbers: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut number = numbers.first().unwrap().to_string();
        number.push(*numbers.last().unwrap());
        acc + number.parse::<usize>().unwrap()
    })
}

fn second(mut input: &str, translations: &[(&str, &str); 9]) -> usize {
    let mut renamed = String::new();
    'outer: while !input.is_empty() {
        for (key, value) in translations {
            if input.starts_with(key) {
                renamed.push_str(value);
                input = &input[1..];
                continue 'outer;
            }
        }
        renamed.push_str(&input[0..1]);
        input = &input[1..];
    }
    first(&renamed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(first(&String::from("two1nine")), 11);
        assert_eq!(first(&String::from("5two1nine")), 51);
    }

    #[test]
    fn test2() {
        assert_eq!(second(&String::from("two1nine"), &TRANSLATION), 29);
        assert_eq!(second(&String::from("eighttwothree"), &TRANSLATION), 83);
        assert_eq!(second(&String::from("7pqrstsixteen"), &TRANSLATION), 76);
        assert_eq!(second(&String::from("xtwone3four"), &TRANSLATION), 24);
        assert_eq!(second(&String::from("eightwo"), &TRANSLATION), 82);
        assert_eq!(
            second(
                &String::from(
                    "two1nine
                     eightwothree
                     abcone2threexyz
                     xtwone3four
                     4nineeightseven2
                     zoneight234
                     7pqrstsixteen"
                ),
                &TRANSLATION
            ),
            281
        );
    }
}
