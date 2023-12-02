/*
Day 2: Cube Conundrum (Part 1)
https://adventofcode.com/2023/day/1
*/

fn main() {
    let input = include_str!("./day2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u16 {
    /* WIP */
    input
        .lines()
        .filter_map(|line| {
            dbg!(line);
            Some(2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
"#;
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 8);
    }
}
