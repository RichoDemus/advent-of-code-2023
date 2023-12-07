use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;
use std::cmp::Ordering;
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut hands = input.lines().map(parse_hand).collect::<Vec<_>>();
    info!("hands");
    hands.sort_unstable();
    for hand in &hands {
        info!("{hand}: {}", hand.ttype);
    }
    let mut score = 0;
    for (rank, card) in hands.into_iter().enumerate() {
        score += card.bet * (rank + 1);
    }
    score
}

fn parse_hand(hand: &str) -> Hand {
    info!("parse: {hand}");
    let (cards, bet) = hand.split_at(hand.find(' ').unwrap());
    let bet = bet.trim().parse::<usize>().unwrap();

    let cards: Vec<Card> = cards.chars().map(Card::new).collect();
    let ttype = ttype(cards.as_slice());
    let ttype2 = ttype2(cards.as_slice());
    Hand {
        cards,
        bet,
        ttype,
        ttype2,
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
    ttype: Type,
    ttype2: Type,
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ttype == other.ttype {
            for (left, right) in self.cards.iter().zip(other.cards.iter()) {
                if left == right {
                    continue;
                }
                return left.cmp(right);
            }
            panic!("Two completely indentical hands")
        } else {
            self.ttype.cmp(&other.ttype)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{} {}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.bet
        )
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Card {
    value: usize,
}

impl Card {
    fn new(card: char) -> Self {
        let value = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            i if i <= '9' && i > '1' => i.to_digit(10).unwrap().try_into().unwrap(),
            _ => panic!(),
        };
        Self { value }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value {
            14 => write!(f, "A"),
            13 => write!(f, "K"),
            12 => write!(f, "Q"),
            11 => write!(f, "J"),
            10 => write!(f, "T"),
            e => write!(f, "{e}"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Type {
    ttype: usize,
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.ttype {
            7 => write!(f, "Five of a kind"),
            6 => write!(f, "Four of a kind"),
            5 => write!(f, "Full house"),
            4 => write!(f, "Three of a kind"),
            3 => write!(f, "Two pair"),
            2 => write!(f, "One pair"),
            1 => write!(f, "High card"),
            _ => panic!(),
        }
    }
}

impl Type {
    const fn five_of_a_kind() -> Self {
        Self { ttype: 7 }
    }
    const fn four_of_a_kind() -> Self {
        Self { ttype: 6 }
    }
    const fn full_house() -> Self {
        Self { ttype: 5 }
    }
    const fn three_of_a_kind() -> Self {
        Self { ttype: 4 }
    }
    const fn two_pairs() -> Self {
        Self { ttype: 3 }
    }
    const fn pair() -> Self {
        Self { ttype: 2 }
    }
    const fn high_card() -> Self {
        Self { ttype: 1 }
    }
}

fn ttype(cards: &[Card]) -> Type {
    info!(
        "figure out type: {}",
        cards
            .iter()
            .copied()
            .fold(String::new(), |mut output, card| {
                let _ = write!(output, "{card}");
                output
            })
    );

    let cards = cards
        .iter()
        .copied()
        .map(|card| (card, 1))
        .into_group_map()
        .into_iter()
        .map(|(key, value)| (value.iter().sum::<usize>(), key))
        .into_group_map();

    if cards.contains_key(&5) {
        Type::five_of_a_kind()
    } else if cards.contains_key(&4) {
        Type::four_of_a_kind()
    } else if cards.contains_key(&3) && cards.contains_key(&2) {
        Type::full_house()
    } else if cards.contains_key(&3) {
        Type::three_of_a_kind()
    } else if cards.get(&2).unwrap_or(&vec![]).len() == 2 {
        Type::two_pairs()
    } else if cards.contains_key(&2) {
        Type::pair()
    } else {
        Type::high_card()
    }
}

fn ttype2(cards: &[Card]) -> Type {
    if cards.contains(&Card { value: 11 }) {
        info!(
            "figure out type: {}",
            cards
                .iter()
                .copied()
                .fold(String::new(), |mut output, card| {
                    let _ = write!(output, "{card}");
                    output
                })
        );
        let number_of_jokers = cards.iter().filter(|card| card.value == 11).count();
        let cards = cards
            .iter()
            .copied()
            .filter(|card| card.value != 11) //dont count jokers for pairs and stuff
            .map(|card| (card, 1))
            .into_group_map()
            .into_iter()
            .map(|(key, value)| (value.iter().sum::<usize>(), key))
            .into_group_map();

        let ttype = if cards.contains_key(&5) {
            Type::five_of_a_kind()
        } else if cards.contains_key(&4) {
            Type::four_of_a_kind()
        } else if cards.contains_key(&3) && cards.contains_key(&2) {
            Type::full_house()
        } else if cards.contains_key(&3) {
            Type::three_of_a_kind()
        } else if cards.get(&2).unwrap_or(&vec![]).len() == 2 {
            Type::two_pairs()
        } else if cards.contains_key(&2) {
            Type::pair()
        } else {
            Type::high_card()
        };
        info!("\ttype without joker: {ttype}");
        //upgrade type since we have joker

        let mut ttype = ttype.ttype;
        for _ in 0..number_of_jokers {
            ttype = match ttype {
                7 | 6 => Type::five_of_a_kind().ttype,
                5 | 4 => Type::four_of_a_kind().ttype,
                3 => Type::full_house().ttype,
                2 => Type::three_of_a_kind().ttype,
                1 => Type::pair().ttype,
                ttyp => panic!("cant happen? ttype is {ttyp}"),
            };
            info!("\t\tupgraded to {}", Type { ttype });
        }
        Type { ttype }
    } else {
        ttype(cards)
    }
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut hands = input.lines().map(parse_hand).collect::<Vec<_>>();
    info!("hands");
    hands.sort_unstable_by(part2_cmp);
    for hand in &hands {
        info!("{hand}: {}", hand.ttype2);
    }
    let mut score = 0;
    for (rank, card) in hands.into_iter().enumerate() {
        info!(
            "\tCard {card} is rank {} for a score of {}",
            rank + 1,
            card.bet * (rank + 1)
        );
        score += card.bet * (rank + 1);
    }
    score
}

fn part2_cmp(left: &Hand, right: &Hand) -> Ordering {
    if left.ttype2 == right.ttype2 {
        for (left, right) in left.cards.iter().zip(right.cards.iter()) {
            if left == right {
                continue;
            }
            info!("\tcomparing {left} {right}");
            let left = if left.value == 11 { 1 } else { left.value };
            let right = if right.value == 11 { 1 } else { right.value };

            return left.cmp(&right);
        }
        panic!("Two completely indentical hands")
    } else {
        left.ttype2.cmp(&right.ttype2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day7.txt");
        assert_eq!(part1(input), 249726565);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day7.txt");
        assert_eq!(part2(input), 251135960);
    }

    #[test]
    fn part1_provided_example() {
        // let _ = env_logger::builder()
        //     .filter_module("advent_of_code_2023", log::LevelFilter::Info)
        //     .try_init();
        let result = part1(
            r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#,
        );

        assert_eq!(result, 6440)
    }

    #[test]
    fn part2_provided_example() {
        // let _ = env_logger::builder()
        //     .filter_module("advent_of_code_2023", log::LevelFilter::Info)
        //     .try_init();
        let result = part2(
            r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#,
        );

        assert_eq!(result, 5905)
    }

    #[test]
    fn part2_made_up_example() {
        // let _ = env_logger::builder()
        //     .filter_module("advent_of_code_2023", log::LevelFilter::Info)
        //     .try_init();
        let result = part2(
            r#"KKK23 1
KKJ23 10"#,
        );

        assert_eq!(result, 12)
    }

    #[test]
    fn part2_isolated_type() {
        // let _ = env_logger::builder()
        //     .filter_module("advent_of_code_2023", log::LevelFilter::Info)
        //     .try_init();

        assert_eq!(parse_hand("32T3K 1").ttype2, Type::pair());
        assert_eq!(parse_hand("KK677 1").ttype2, Type::two_pairs());
        assert_eq!(parse_hand("T55J5 1").ttype2, Type::four_of_a_kind());
        assert_eq!(parse_hand("KTJJT 1").ttype2, Type::four_of_a_kind());
        assert_eq!(parse_hand("QQQJA 1").ttype2, Type::four_of_a_kind());
        assert_eq!(parse_hand("QJJQ2 1").ttype2, Type::four_of_a_kind());

        assert_eq!(parse_hand("2345J 1").ttype2, Type::pair());
        assert_eq!(parse_hand("234JJ 1").ttype2, Type::three_of_a_kind());
        assert_eq!(parse_hand("23JJJ 1").ttype2, Type::four_of_a_kind());
        assert_eq!(parse_hand("2JJJJ 1").ttype2, Type::five_of_a_kind());
    }
}
