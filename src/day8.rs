use crate::day8::Direction::{Left, Right};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;
use std::collections::{HashMap, HashSet};

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (instructions, network) = input.split_at(input.find('\n').unwrap());
    let instructions = instructions
        .trim()
        .chars()
        .map(|c| if c == 'L' { Left } else { Right })
        .collect_vec();
    let network = network
        .trim()
        .lines()
        .map(parse_node)
        .collect::<HashMap<_, _>>();

    info!("{instructions:?}");
    info!("{network:?}");

    let mut moves = 0;
    let mut i = 0;
    let mut current_node = "AAA".to_string();
    loop {
        let instruction = &instructions[i];
        let directions = network.get(&current_node).unwrap();
        current_node = if instruction == &Left {
            directions.0.clone()
        } else {
            directions.1.clone()
        };
        moves += 1;
        if current_node == "ZZZ" {
            return moves;
        }
        i += 1;
        if i == instructions.len() {
            i = 0;
        }
    }
}

fn parse_node(line: &str) -> (String, (String, String)) {
    let (name, nodes) = line.split_at(line.find('=').unwrap());
    let name = name.trim().to_string();
    let nodes = nodes.replace(['(', ')', '='], "");
    let (left, right) = nodes.split_at(nodes.find(',').unwrap());
    let left = left.trim().to_string();
    let right = right.replace(',', "");
    let right = right.trim().to_string();

    (name, (left, right))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[aoc(day8, part2, mine)]
fn part2(input: &str) -> usize {
    let (instructions, network) = input.split_at(input.find('\n').unwrap());
    let instructions = instructions
        .trim()
        .chars()
        .map(|c| if c == 'L' { Left } else { Right })
        .collect_vec();
    let network = network
        .trim()
        .lines()
        .map(parse_node)
        .collect::<HashMap<_, _>>();

    assert_eq!(network.len(), network.keys().collect::<HashSet<_>>().len());

    let mut current_nodes: Vec<(String, Vec<String>, bool)> = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .map(|node| (node, vec![], false))
        .collect_vec();

    let mut i = 0;
    loop {
        let instruction = &instructions[i];
        let mut num_done = 0;
        for (node, visited_nodes, done) in &mut current_nodes {
            if *done {
                num_done += 1;
                continue;
            }
            let directions = network.get(node).unwrap();
            *node = if instruction == &Left {
                directions.0.clone()
            } else {
                directions.1.clone()
            };
            visited_nodes.push(node.clone());
            if node.ends_with('Z') {
                *done = true;
            }
        }

        if num_done == current_nodes.len() {
            break;
        }

        i += 1;
        if i == instructions.len() {
            i = 0;
        }
    }

    info!("path: {:?}", current_nodes[0]);

    let lengths = current_nodes
        .iter()
        .map(|(_, visits, _)| visits.len())
        .collect_vec();
    info!("lenghts: {lengths:?}");

    let mut gcd = (lengths[0] * lengths[1]) / calc_gcd(lengths[0], lengths[1]);
    for length in lengths.into_iter().skip(2) {
        info!("calculating gcd for {gcd} and {}", length);
        gcd = (gcd * length) / calc_gcd(gcd, length);
    }
    gcd
}

const fn calc_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day8.txt");
        assert_eq!(part1(input), 19241);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day8.txt");
        assert_eq!(part2(input), 9606140307013);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
        );

        assert_eq!(result, 2)
    }

    #[test]
    fn part1_provided_example2() {
        let result = part1(
            r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
        );

        assert_eq!(result, 6)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#,
        );

        assert_eq!(result, 6)
    }
}
