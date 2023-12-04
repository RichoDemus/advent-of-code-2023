use anyhow::{Context, Result};
use aoc_runner_derive::aoc;
use std::collections::VecDeque;
use std::str::FromStr;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .map(Card::part1_points)
        .sum()
}

#[aoc(day4, part2)]
fn part2_old(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    for card in &cards {
        queue.push_back(card.clone());
    }

    let mut processed_cards = 0;

    while let Some(card) = queue.pop_front() {
        let points = card.victories;
        for i in 0..points {
            let card = &cards[card.id + i];
            queue.push_front(card.clone());
        }
        processed_cards += 1;
    }

    processed_cards
}

#[aoc(day4, part2, rec)]
fn part2(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();

    let mut processed_cards = 0;
    for card in &cards {
        processed_cards += calc_rec(card, &cards);
    }
    // processed_cards += calc_rec(&cards[0], &cards, 0, 1);
    processed_cards
}

fn calc_rec(card: &Card, cards: &Vec<Card>) -> usize {
    let points = card.victories;

    let mut processed_cards = 0;
    for i in 1..=points {
        let card = &cards[card.id + i - 1];
        processed_cards += calc_rec(card, cards);
    }

    processed_cards + 1
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    victories: usize,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let split = line.split(':').collect::<Vec<_>>();
        let id: usize = split[0]
            .replace("Card", "")
            .trim()
            .parse()
            .context(format!("failed to parse: {line}"))?;

        let numbers = split[1].split('|').collect::<Vec<_>>();

        let winning_numbers: Vec<i32> = numbers[0]
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            // .map(|x|{
            //     println!("x: {}", x);
            //     x
            // })
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .context("winning")?;

        let my_numbers = numbers[1]
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .context("my")?;

        let victories = my_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();
        Ok(Self {
            id,
            winning_numbers,
            my_numbers,
            victories,
        })
    }
}

impl Card {
    fn part1_points(self) -> usize {
        let victories: u32 = match self
            .my_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
            .try_into()
        {
            Ok(x) => x,
            Err(e) => panic!("Failed to convert: {e}"),
        };
        if victories > 0 {
            let two: usize = 2;
            two.pow(victories - 1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day4.txt");
        assert_eq!(part1(input), 25231);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day4.txt");
        assert_eq!(part2(input), 9721255);
        assert_eq!(part2_old(input), 9721255);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
        );

        assert_eq!(result, 13)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
        );

        assert_eq!(result, 30)
    }
}
