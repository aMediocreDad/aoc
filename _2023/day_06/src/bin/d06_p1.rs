/*
Day 6: Wait for it (Part 1)
https://adventofcode.com/2023/day/6
*/

fn main() {
    let input = include_str!("./day6.txt");
    let output = solution(input);
    dbg!(output);
}

fn parse_values(string: &str) -> Vec<usize> {
    string
        .trim()
        .split_whitespace()
        .filter_map(|s| {
            if let Ok(s) = s.parse::<usize>() {
                Some(s)
            } else {
                None
            }
        })
        .collect()
}

fn solution(input: &str) -> usize {
    let mut iter = input.lines();
    let times = parse_values(iter.next().expect("A string"));
    let distances = parse_values(iter.next().expect("A string"));
    times
        .iter()
        .enumerate()
        .map(|(index, time)| {
            let distance = distances[index];
            (0..*time)
                .filter_map(|n| {
                    if (time - n) * n > distance {
                        Some(n)
                    } else {
                        None
                    }
                })
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 288);
    }
}
