/*
Day 1: Trebuchet?! (Part 1)
https://adventofcode.com/2023/day/1
*/

fn main() {
    let input = include_str!("./day1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u16 {
    input
        .lines()
        .filter_map(|line| {
            let f_i = line.find(|c: char| c.is_numeric());
            let s_i = line.rfind(|c: char| c.is_numeric());
            match (f_i, s_i) {
                (Some(f), Some(s)) => {
                    let first = line.chars().nth(f).unwrap().to_string();
                    let second = line.chars().nth(s).unwrap().to_string();
                    Some(first + &second)
                }
                _ => None,
            }
        })
        .map(|s| s.parse::<u16>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 142);
    }
}
