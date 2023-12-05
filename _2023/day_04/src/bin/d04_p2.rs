/*
Day 4: Scratchcards (Part 2)
https://adventofcode.com/2023/day/4
*/
use onig::Regex;
use std::{collections::BTreeMap, str::Split};

fn main() {
    let input = include_str!("./day4.txt");
    let output = solution(input);
    dbg!(output);
}

fn parse_card(input: &mut Split<'_, &str>) -> Vec<u32> {
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

fn solution(input: &str) -> usize {
    let mut card_copies: BTreeMap<usize, usize> = BTreeMap::new();

    input.lines().enumerate().fold(0, |acc, (index, line)| {
        let re = Regex::new(r"Card \d+: ").unwrap();

        let line = re.replace(line, "").to_owned();
        let mut cards = line.trim().split(" | ");

        let winning_numbers = parse_card(&mut cards);
        let numbers_you_have = parse_card(&mut cards);

        let end = *card_copies.get(&index).unwrap_or(&1);

        // Iterate over copies of cards you have and count the number of wins
        acc + (0..end)
            .map(|_| {
                let number_of_wins = numbers_you_have
                    .iter()
                    .filter(|n| winning_numbers.contains(n))
                    .count();
                if number_of_wins > 0 {
                    for i in index + 1..=index + number_of_wins {
                        card_copies.entry(i).and_modify(|e| *e += 1).or_insert(2);
                    }
                }
            })
            .count()
    })
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
        assert_eq!(output, 30);
    }
}
