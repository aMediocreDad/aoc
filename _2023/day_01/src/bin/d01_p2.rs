/*
Day 1: Trebuchet?! (Part 2)
https://adventofcode.com/2023/day/1
*/

use onig::Regex;

fn main() {
    let input = include_str!("./day1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u16 {
    let numbers_rx = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    input
        .lines()
        .filter_map(|line| {
            let mut numbers = Vec::new();
            for c in numbers_rx.captures_iter(line) {
                let num_str = c.at(1).unwrap();
                if let Ok(num) = num_str.parse::<u8>() {
                    numbers.push(num);
                } else {
                    let num = match num_str {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => continue,
                    };
                    numbers.push(num);
                }
            }
            let first_last =
                numbers.first().unwrap().to_string() + &numbers.last().unwrap().to_string();
            Some(first_last)
        })
        .map(|s| s.parse::<u16>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Add overlapping numbers to test positive lookahead
    const INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthreexyz
"#;

    #[test]
    fn test_part1() {
        let output = solution(INPUT);
        assert_eq!(output, 364);
    }
}
