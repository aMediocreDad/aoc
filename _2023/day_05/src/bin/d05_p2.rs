/*
Day 5: If You Give A Seed A Fertilizer (Part 2)
https://adventofcode.com/2023/day/5
*/
use ::onig::Regex;
// Not using rayon for this one means it takes a while to run
use rayon::prelude::*;

fn main() {
    let input = include_str!("./day5.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug)]
struct Rule {
    min: u64,
    max: u64,
    dest_offset: i128,
}

impl Rule {
    fn new(dest: u64, min: u64, range: u64) -> Self {
        let max = min + range - 1;
        let dest_offset = dest as i128 - min as i128;
        Self {
            min,
            max,
            dest_offset,
        }
    }
}

fn solution(input: &str) -> u64 {
    // Regex gets only numbers [0-9] and whitespace/newlines in groups
    let re = Regex::new(r"[\w\- ]+?:(:?\D([\d+\s]+))").unwrap();

    let mut parsed = re.captures_iter(input);

    let mut seeds: Vec<u64> = vec![];

    parsed
        .next()
        .expect("No seeds found")
        .at(1)
        .expect("No seeds found")
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .for_each(|chunk| {
            for i in chunk[0]..=chunk[0] + chunk[1] {
                seeds.push(i);
            }
        });

    // Creates a list of rules for each category using Rule struct
    let categories = parsed
        .map(|cap| {
            cap.at(1)
                .unwrap()
                .trim()
                .lines()
                .map(|line| {
                    let ranges: Vec<u64> = line
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();
                    Rule::new(ranges[0], ranges[1], ranges[2])
                })
                .collect::<Vec<Rule>>()
        })
        .collect::<Vec<Vec<Rule>>>();

    // We can then recursively apply the rules to the seeds
    // and find the smallest result
    seeds
        .into_par_iter()
        .map(|seed| {
            categories.iter().fold(seed, |acc, range| {
                if let Some(rule) = range.iter().find(|rule| rule.min <= acc && acc <= rule.max) {
                    (acc as i128 + rule.dest_offset) as u64
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 46);
    }
}
