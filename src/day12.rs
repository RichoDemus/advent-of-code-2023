use crate::day12::Spring::{Damaged, Operational, Unknown};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::usize;

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(|line| solve_part2(line, 1)).sum()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => todo!(),
        }
    }
}

fn vec_of_display_to_string<T: Display>(items: &[T]) -> String {
    items.iter().map(|p| format!("{p}")).join("")
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operational => write!(f, "."),
            Damaged => write!(f, "#"),
            Unknown => write!(f, "?"),
        }
    }
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        result += solve_part2(line, 5);
    }
    result
}

fn solve_part2(line: &str, expansions: usize) -> usize {
    let line = expand(line, expansions);

    let (springs, checksum) = line.split_at(line.find(' ').unwrap());
    let springs = springs.chars().map(Spring::from).collect_vec();
    let checksum = checksum
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Tried to parse '{s}'"))
        })
        .collect_vec();
    info!("{} {checksum:?}", vec_of_display_to_string(&springs));

    let mut cache = HashMap::new();
    solve_rec(springs.as_slice(), checksum.as_slice(), &mut cache, 0, 0, 0)
}

#[allow(clippy::option_if_let_else, clippy::cognitive_complexity)]
fn solve_rec(
    springs: &[Spring],
    checksum: &[usize],
    cache: &mut HashMap<(usize, usize, usize), usize>,
    index: usize,
    current_group_size: usize,
    checksum_index: usize,
) -> usize {
    if let Some(cached) = cache.get(&(index, current_group_size, checksum_index)) {
        *cached
    } else {
        let tabs = "\t".repeat(index);
        let maybe_spring = springs.get(index);
        info!("{}i:{index}: + {maybe_spring:?}. current group size: {current_group_size}. checksum: {} ({checksum_index})", tabs, checksum.get(checksum_index).unwrap_or(&1337));
        let result = match maybe_spring {
            None => {
                info!("{}at the end", tabs);
                if let Some(current_checksum) = checksum.get(checksum_index) {
                    if current_group_size == *current_checksum
                        && checksum.len() == checksum_index + 1
                    {
                        info!("{tabs}s: is ok");
                        1
                    } else {
                        info!("{tabs}s: is NOT ok for {checksum:?}");
                        0
                    }
                } else {
                    let mut offset = 0;
                    if current_group_size > 0 {
                        offset += 1;
                    }
                    if checksum.len() == checksum_index + offset {
                        info!("{tabs}: is ok");
                        1
                    } else {
                        info!("{tabs}: is NOT ok for {checksum:?}");
                        0
                    }
                }
            }
            Some(Operational) => {
                if let Some(curr_check) = checksum.get(checksum_index) {
                    if current_group_size > *curr_check {
                        info!("{tabs}: is NOT ok: group size to big");
                        0
                    } else if current_group_size > 0 {
                        if current_group_size == checksum[checksum_index] {
                            solve_rec(springs, checksum, cache, index + 1, 0, checksum_index + 1)
                        } else {
                            info!("{tabs}: is NOT ok: wrong group size");
                            0
                        }
                    } else {
                        solve_rec(springs, checksum, cache, index + 1, 0, checksum_index)
                    }
                } else {
                    solve_rec(springs, checksum, cache, index + 1, 0, checksum_index)
                }
            }
            Some(Damaged) => {
                if checksum_index == checksum.len() {
                    info!("{tabs}: is NOT ok: checksum out of bounds");
                    0
                } else if current_group_size > checksum[checksum_index] {
                    info!("{tabs}: is NOT ok: group size to big");
                    0
                } else {
                    solve_rec(
                        springs,
                        checksum,
                        cache,
                        index + 1,
                        current_group_size + 1,
                        checksum_index,
                    )
                }
            }
            Some(Unknown) => {
                if checksum_index == checksum.len() {
                    if current_group_size == 0 {
                        // we're out of checksums and the group is closed, lets continue with .
                        solve_rec(springs, checksum, cache, index + 1, 0, checksum_index)
                    } else {
                        todo!()
                    }
                } else if current_group_size > checksum[checksum_index] {
                    info!("{tabs}: is NOT ok: group size to big");
                    0
                } else {
                    //treat this unknown as operational
                    info!(
                        "{tabs} can next be operational?: curr: {current_group_size} check: {}",
                        checksum[checksum_index]
                    );
                    if current_group_size > 0 && current_group_size < checksum[checksum_index] {
                        info!("{tabs} no it cant");
                        solve_rec(
                            springs,
                            checksum,
                            cache,
                            index + 1,
                            current_group_size + 1,
                            checksum_index,
                        )
                    } else {
                        let x = if current_group_size > 0 {
                            solve_rec(springs, checksum, cache, index + 1, 0, checksum_index + 1)
                        } else {
                            solve_rec(springs, checksum, cache, index + 1, 0, checksum_index)
                        };
                        // treat this unknown as damaged path
                        x + solve_rec(
                            springs,
                            checksum,
                            cache,
                            index + 1,
                            current_group_size + 1,
                            checksum_index,
                        )
                    }
                }
            }
        };
        cache.insert((index, current_group_size, checksum_index), result);
        result
    }
}

#[allow(clippy::similar_names)]
fn expand(line: &str, expand_size: usize) -> String {
    let (springs, checksum) = line.split_at(line.find(' ').unwrap());
    let mut springs = springs.to_string();
    springs += "?";
    let mut strings = springs.trim().repeat(expand_size);
    strings.remove(strings.len() - 1);
    let mut checksum = checksum.to_string();
    checksum += ",";
    let mut checksum = checksum.trim().repeat(expand_size);
    checksum.remove(checksum.len() - 1);

    format!("{strings} {checksum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day12.txt");
        assert_eq!(part1(input), 8193);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day12.txt");
        assert_eq!(part2(input), 45322533163795);
    }

    #[test]
    fn part1_provided_example() {
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("????.#...#... 4,1,1"), 1);
        assert_eq!(part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(part1("?###???????? 3,2,1"), 10);

        assert_eq!(
            part1(
                r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#,
            ),
            21
        );
    }

    #[test]
    fn part2_provided_example() {
        assert_eq!(
            expand("???.### 1,1,3", 5),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
        );
        assert_eq!(expand("???.### 1,1,3", 1), "???.### 1,1,3");

        // part 1
        assert_eq!(solve_part2("???.### 1,1,3", 1), 1);
        assert_eq!(solve_part2(".??..??...?##. 1,1,3", 1), 4);
        assert_eq!(solve_part2("?#?#?#?#?#?#?#? 1,3,1,6", 1), 1);
        assert_eq!(solve_part2("????.#...#... 4,1,1", 1), 1);
        assert_eq!(solve_part2("????.######..#####. 1,6,5", 1), 4);
        assert_eq!(solve_part2("?###???????? 3,2,1", 1), 10);

        //part 2
        assert_eq!(solve_part2("???.### 1,1,3", 5), 1);
        assert_eq!(solve_part2(".??..??...?##. 1,1,3", 5), 16384);
        assert_eq!(solve_part2("?#?#?#?#?#?#?#? 1,3,1,6", 5), 1);
        assert_eq!(solve_part2("????.#...#... 4,1,1", 5), 16);
        assert_eq!(solve_part2("????.######..#####. 1,6,5", 5), 2500);
        assert_eq!(solve_part2("?###???????? 3,2,1", 5), 506250);

        assert_eq!(
            part2(
                r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
            ),
            525152
        );

        let expected = part1("?#.??????#??#?#?#?#? 1,1,15");
        assert_eq!(solve_part2("?#.??????#??#?#?#?#? 1,1,15", 1), expected);
        assert_eq!(solve_part2("?#.??????#??#?#?#?#? 1,1,15", 5), 81);
    }
}
