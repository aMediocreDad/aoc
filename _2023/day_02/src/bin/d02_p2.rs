/*
Day 2: Cube Conundrum (Part 2)
https://adventofcode.com/2023/day/2
*/

struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Set {
    fn get_max(current: u8, new: u8) -> u8 {
        if new > current {
            new
        } else {
            current
        }
    }

    fn add_max(mut self, color: &str, count: u8) -> Self {
        match color {
            "red" => self.red = Self::get_max(self.red, count),
            "green" => self.green = Self::get_max(self.green, count),
            "blue" => self.blue = Self::get_max(self.blue, count),
            _ => unreachable!(),
        }
        self
    }

    fn get_output(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

fn main() {
    let input = include_str!("./day2.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(": ");
            let _ = iter.next();
            let sets = iter.next().expect("We need a string of sets").split("; ");
            return sets.map(|s| s.split(", "))
                .flatten()
                .map(|s| {
                    let mut count_and_color = s.split(" ");
                    let count = count_and_color
                        .next()
                        .expect("We need a count")
                        .parse::<u8>()
                        .expect("Count needs to be a number");
                    let color = count_and_color.next().expect("We need a color");
                    (color, count)
                })
                .fold(
                    Set {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |set, (color, count)| set.add_max(color, count),
                )
                .get_output()
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
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 2286);
    }
}
