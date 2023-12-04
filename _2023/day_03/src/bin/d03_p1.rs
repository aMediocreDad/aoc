/*
Day 3: Cube Conundrum (Part 1)
https://adventofcode.com/2023/day/3
*/

use std::collections::BTreeMap;

const INDICES: [i32; 3] = [-1, 0, 1];

fn main() {
    let input = include_str!("./day3.txt");
    let output = solution(input);
    dbg!(output);
}

fn insert_symbol(symbols: &mut BTreeMap<(usize, usize), bool>, row: usize, index: usize) {
    for i in INDICES.iter() {
        for j in INDICES.iter() {
            let row = row as i32 + i;
            let index = index as i32 + j;
            symbols.insert((row.max(0) as usize, index.max(0) as usize), true);
        }
    }
}

fn solution(input: &str) -> u32 {
    let mut symbols: BTreeMap<(usize, usize), bool> = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (index, char) in line.chars().enumerate() {
            match char {
                c if c.is_numeric() || c == '.' => continue,
                _ => {
                    insert_symbol(&mut symbols, row, index);
                }
            }
        }
    }
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut is_parts_number = false;
            let mut string = String::new();
            let mut numbers = Vec::new();
            for (index, char) in line.chars().enumerate() {
                match char {
                    c if c.is_numeric() => {
                        if symbols.contains_key(&(row, index)) {
                            is_parts_number = true;
                        }
                        string.push(c);
                    }
                    _ => {
                        if !string.is_empty() && is_parts_number {
                            numbers.push(string.parse::<u32>().expect("Should be a number"));
                            string.clear();
                            is_parts_number = false;
                        }
                        string.clear();
                    }
                }
                if index == line.len() - 1 && !string.is_empty() && is_parts_number {
                    numbers.push(string.parse::<u32>().expect("Should be a number"));
                    string.clear();
                }
            }
            numbers.iter().sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 4361);
    }
}
