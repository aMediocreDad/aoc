/*
Day 6: Wait for it (Part 2)
https://adventofcode.com/2023/day/6
*/

fn main() {
    let input = include_str!("./day6.txt");
    let output = solution(input);
    dbg!(output);
}

fn parse_values(string: &str) -> usize {
    string
        .trim()
        .split_whitespace()
        .filter_map(|s| {
            if s.parse::<usize>().is_ok() {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("A number")
}

fn solution(input: &str) -> usize {
    let mut iter = input.lines();
    let time = parse_values(iter.next().expect("A string"));
    let distance = parse_values(iter.next().expect("A string"));
    (0..time)
        .filter_map(|n| {
            if (time - n) * n > distance {
                Some(n)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 71503);
    }
}
