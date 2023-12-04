/*
Day 4: Scratchcards (Part 1)
https://adventofcode.com/2023/day/4
*/
use ::onig::Regex;
use std::str::Split;

fn main() {
    let input = include_str!("./day4.txt");
    let output = solution(input);
    dbg!(output);
}

fn split_card(input: &mut Split<'_, &str>) -> Vec<u32> {
    input
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|n| match n {
            str if str.parse::<u32>().is_ok() => {
                Some(str.parse::<u32>().expect("This should not be possible"))
            }
            _ => None,
        })
        .collect::<Vec<_>>()
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> u32 {
            let re = Regex::new(r"Card \d+: ").unwrap();
            let line = re.replace(line, "").to_owned();
            let mut cards = line.trim().split(" | ");
            let winning_numbers = split_card(&mut cards);
            let scratch = split_card(&mut cards);
            scratch
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 13);
    }
}
