/*
Day 3: Cube Conundrum (Part 2)
https://adventofcode.com/2023/day/3
*/

use std::collections::BTreeMap;

const ROWS: [i32; 3] = [-1, 0, 1];

fn main() {
    let input = include_str!("./day3.txt");
    let output = solution(input);
    dbg!(output);
}

fn insert_symbol(
    symbols: &mut BTreeMap<(usize, usize), Vec<u32>>,
    row: i32,
    index: i32,
    val: &str,
) {
    let cols = (-1..=val.len() as i32).collect::<Vec<i32>>();
    let val = val.parse::<u32>().expect("Should be a number");
    for i in ROWS.iter() {
        for j in cols.iter() {
            let mod_row = row + i;
            let mod_index = index + j;
            if mod_row.is_negative() || mod_index.is_negative() {
                continue;
            }
            if let Some(curr) = symbols.get_mut(&(mod_row as usize, mod_index as usize)) {
                curr.push(val);
            } else {
                symbols.insert((mod_row as usize, mod_index as usize), vec![val]);
            }
        }
    }
}

fn solution(input: &str) -> u32 {
    let mut symbols: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    for (row, line) in input.lines().enumerate() {
        let mut string = String::new();
        let mut first_index = -1;
        for (index, char) in line.chars().enumerate() {
            match char {
                c if c.is_numeric() => {
                    if first_index == -1 {
                        first_index = index as i32;
                    }
                    string.push(c);
                }
                _ => {
                    if !string.is_empty() {
                        insert_symbol(&mut symbols, row as i32, first_index, &string);
                        string.clear();
                        first_index = -1;
                    }
                }
            }
            if index == line.len() - 1 && !string.is_empty() && first_index != -1 {
                insert_symbol(&mut symbols, row as i32, first_index, &string);
                string.clear();
                first_index = -1;
            }
        }
    }
    input
        .lines()
        .enumerate()
        .filter_map(|(row, line)| {
            let mut power = Vec::new();
            for (index, char) in line.chars().enumerate() {
                match char {
                    '*' => {
                        if let Some(numbers) = symbols.get(&(row, index)) {
                            if numbers.len() > 1 {
                                power.push(numbers[0..2].iter().product());
                            }
                        }
                    }
                    _ => continue,
                }
            }
            if power.len() > 0 {
                Some(power.iter().sum::<u32>())
            } else {
                None
            }
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
        assert_eq!(output, 467835);
    }
}
