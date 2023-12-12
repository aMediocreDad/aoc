/*
Day 7: Camel Cards (Part 1)
https://adventofcode.com/2023/day/7
*/

fn main() {
    let input = include_str!("./day7.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Card {
    value: u8,
}

impl Card {
    fn new(val: char) -> Self {
        let value = match val {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => val.to_digit(10).expect("Card value should be a number") as u8,
        };
        Self { value }
    }
}

type Hand = Vec<Card>;

trait HandRank {
    fn rank(self) -> (u32, Vec<u8>);
}

impl HandRank for Hand {
    fn rank(mut self) -> (u32, Vec<u8>) {
        // Sum the indexed values of the cards in the hand
        let card_sum = self
            .iter()
            .map(|card| card.value)
            .collect::<Vec<u8>>();
        // Sort in order to check for patterns
        let rank = match self.sort_unstable() {
            _ if is_flush(&self) => 7,
            _ if is_n_of_a_kind(&self, 4) => 6,
            _ if is_full_house(&self) => 5,
            _ if is_n_of_a_kind(&self, 3) => 4,
            _ if is_double_pair(&self) => 3,
            _ if is_n_of_a_kind(&self, 2) => 2,
            _ => 1,
        };
        // Return the rank + the sum of the cards
        (rank, card_sum)
    }
}

fn is_flush(hand: &Hand) -> bool {
    hand.iter().all(|card| card.value == hand[0].value)
}

fn get_n_of_a_kind(hand: &Hand, n: u8) -> Option<u8> {
    let mut count = 0;
    let mut prev = 0;
    for card in hand {
        if card.value == prev {
            count += 1;
        } else {
            count = 1;
        }
        prev = card.value;
        if count == n {
            return Some(card.value);
        }
    }
    None
}

fn is_n_of_a_kind(hand: &Hand, n: u8) -> bool {
    get_n_of_a_kind(hand, n).is_some()
}

fn is_full_house(hand: &Hand) -> bool {
    if let Some(first_pair) = get_n_of_a_kind(hand, 3) {
        let remainder = hand
            .iter()
            .cloned()
            .filter(|card| card.value != first_pair)
            .collect::<Hand>();
        is_n_of_a_kind(&remainder, 2)
    } else {
        false
    }
}

fn is_double_pair(hand: &Hand) -> bool {
    if let Some(first_pair) = get_n_of_a_kind(hand, 2) {
        let remainder = hand
            .iter()
            .cloned()
            .filter(|card| card.value != first_pair)
            .collect::<Hand>();
        is_n_of_a_kind(&remainder, 2)
    } else {
        false
    }
}

fn solution(input: &str) -> u32 {
    let mut inp = input
        .lines()
        .map(|line| {
            let [cards, bet] = line.split_whitespace().collect::<Vec<_>>()[..2] else {
                panic!("Invalid input");
            };
            (
                cards.chars().map(Card::new).collect::<Hand>().rank(),
                bet.parse::<u32>().expect("Bet should be a number"),
            )
        })
        .collect::<Vec<((u32, Vec<u8>), u32)>>();
    inp.sort_unstable_by(|a, b| {
        let compared = a.0.0.cmp(&b.0.0);
        match compared {
            std::cmp::Ordering::Equal => return a.0.1.cmp(&b.0.1),
            _ => return compared,
        }
    });
    inp.iter()
        .enumerate()
        .inspect(|(index, ((rank, rank2), bet))| {
            println!("{}: {} - {}:{:?}", index + 1, bet, rank, rank2);
        })
        .map(|(index, (_, bet))| bet * (index + 1) as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    //     const INPUT: &str = r#"AAAAA 1
    // AAAAK 2
    // AAAKK 3
    // AAKK3 4
    // AAKK2 5
    // AAKQQ 6
    // AAKQT 7
    // "#;

    #[test]
    fn test_solution() {
        let output = solution(INPUT);
        assert_eq!(output, 6440);
    }
}
