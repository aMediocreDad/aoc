/*
Day 2: Cube Conundrum (Part 1)
https://adventofcode.com/2023/day/2
*/

const MAX: &'static Set = &Set([
    Cube {
        color: Color::Red,
        count: 12,
    },
    Cube {
        color: Color::Green,
        count: 13,
    },
    Cube {
        color: Color::Blue,
        count: 14,
    },
]);

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: Color,
    count: u8,
}

impl Cube {
    fn add_cube(&mut self, count: u8) {
        self.count += count;
    }
}

struct Set([Cube; 3]);

impl Set {
    fn add(&mut self, string: &str) {
        let iter = string.split(", ");
        for s in iter {
            self.add_cube(s);
        }
    }

    fn add_cube(&mut self, string: &str) {
        let mut iter = string.split(" ");
        let count = iter.next().unwrap().parse::<u8>().unwrap();
        let color = match iter.next().unwrap() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => unreachable!(),
        };
        self.0
            .iter_mut()
            .find(|cube| cube.color == color)
            .unwrap()
            .add_cube(count);
    }

    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .all(|cube| cube.count <= MAX.0.iter().find(|c| c.color == cube.color).unwrap().count)
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
        .filter_map(|line| {
            let mut iter = line.split(": ");
            let game = iter.next()?.split(" ").last()?;
            let sets = iter.next()?.split("; ");
            for set in sets {
                let mut parsed = Set([
                    Cube {
                        color: Color::Red,
                        count: 0,
                    },
                    Cube {
                        color: Color::Green,
                        count: 0,
                    },
                    Cube {
                        color: Color::Blue,
                        count: 0,
                    },
                ]);
                parsed.add(set);
                if !parsed.is_valid() {
                    return None;
                }
            }
            Some(game.parse::<u32>().unwrap())
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
        assert_eq!(output, 8);
    }
}
