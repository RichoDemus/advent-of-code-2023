use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;
use crate::day11::Cell::{Empty, Space, Star};

#[allow(clippy::cast_possible_wrap)]
#[aoc(day11, part1)]
fn part1(input: &str) -> i64 {
    let mut picture = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    print_arr(&picture);

    {
        let mut row_index = 0;
        loop {
            let Some(row) = picture.get(row_index) else {
                break;
            };

            if row.iter().all(|star| !star) {
                picture.insert(row_index + 1, vec![false; row.len()]);
                row_index += 1;
            }
            row_index += 1;
        }
    }
    {
        let mut col_index = 0;
        'next_col: loop {
            if picture[0].len() <= col_index {
                break;
            }
            for row in &picture {
                if row[col_index] {
                    col_index += 1;
                    continue 'next_col;
                }
            }
            // this column has no stars, extend it
            for row in 0..picture.len() {
                picture.get_mut(row).unwrap().insert(col_index + 1, false);
            }
            col_index += 2;
        }
    }
    info!("after inserts");
    print_arr(&picture);

    let stars = picture
        .into_iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(|(col_index, star)| {
                    if star {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    info!("after conversion");
    print_vec(&stars);

    stars
        .into_iter()
        .combinations(2)
        .map(|stars| {
            let (r1, c1) = stars[0];
            let (r2, c2) = stars[1];

            let distance = (r1 as i64 - r2 as i64).abs() + (c1 as i64 - c2 as i64).abs();
            info!("Distance between {r1} {c1} and {r2} {c2}: {distance}");
            distance
        })
        .sum::<i64>()
}

fn print_arr(pic: &[Vec<bool>]) {
    for row in pic {
        let mut s = String::new();
        for cell in row {
            s += if *cell { "#" } else { "." }
        }
        info!("{s}");
    }
}

fn print_arr2(pic: &[Vec<Cell>]) {
    for row in pic {
        let mut s = String::new();
        for cell in row {
            s += format!("{cell}").as_str();
        }
        info!("{s}");
    }
}

fn print_vec(stars: &[(usize, usize)]) {
    let (row_max, col_max) = stars
        .iter()
        .copied()
        .reduce(|(row_acc, col_acc), (row_next, col_next)| {
            (row_acc.max(row_next), col_acc.max(col_next))
        })
        .unwrap();

    for row in 0..=row_max {
        let mut s = String::new();
        for col in 0..=col_max {
            s += if stars.contains(&(row, col)) {
                "#"
            } else {
                "."
            }
        }
        info!("{s}");
    }
}

#[aoc(day11, part2)]
fn part2(input: &str) -> i128 {
    solve_part2(input, 1_000_000)
}

fn solve_part2(input:&str, size: i128) -> i128 {
    let mut picture = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '#'{Star} else {Empty}).collect_vec())
        .collect_vec();
    print_arr2(&picture);

    let mut space_index = 1;
    {
        let mut row_index = 0;
        loop {
            let Some(row) = picture.get(row_index) else {
                break;
            };

            if row.iter().all(|cell| *cell == Empty) {
                *picture.get_mut(row_index).unwrap() = vec![Space(vec![space_index]); row.len()];
                space_index += 1;
            }
            row_index += 1;
        }
    }
    {
        let mut col_index = 0;
        'next_col: loop {
            if picture[0].len() <= col_index {
                break;
            }
            for row in &picture {
                if row[col_index] == Star {
                    col_index += 1;
                    continue 'next_col;
                }
            }
            // this column has no stars, extend it
            for row in 0..picture.len() {
                if let Space(i) = picture[row][col_index].clone() {
                    *picture.get_mut(row).unwrap().get_mut(col_index).unwrap() = Space(vec![i[0], space_index]);
                } else {
                    *picture.get_mut(row).unwrap().get_mut(col_index).unwrap() = Space(vec![space_index]);
                }
            }
            space_index += 1;
            col_index += 2;
        }
    }
    info!("after inserts");
    print_arr2(&picture);

    let stars = picture
        .iter()
        .cloned()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(|(col_index, star)| {
                    if star ==Star {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    stars
        .into_iter()
        .combinations(2)
        .map(|stars| {
            let (r1, c1) = stars[0];
            let (r2, c2) = stars[1];

            let distance = calc_distance(&picture, r1 as i128,c1 as i128,r2 as i128,c2 as i128, size);
            info!("Distance between {r1} {c1} and {r2} {c2}: {distance}");
            distance
        })
        .sum::<i128>()
}

#[allow(clippy::cast_sign_loss)]
fn calc_distance(space: &[Vec<Cell>], r1: i128, c1: i128, r2: i128, c2: i128, size:  i128) -> i128 {
    // if r1 != 6 || c1 != 1 || r2 != 11 || c2 != 5 {
    //     return 1
    // }
    // info!("Calculating istance between {r1} {c1} and {r2} {c2}");
    // let vertical_steps = r1.max(r2) - r1.min(r2);
    // let horizontal_steps = c1.max(c2) - c1.min(c2);
    // info!("\t {vertical_steps} vertical steps and {horizontal_steps} hortizontal steps");

    let mut steps:Vec<Cell> = vec![];
    let mut col = c1.min(c2);
    let mut row = r1.min(r2);
    let row_end = r1.max(r2);
    if row != row_end {
        assert!(row < row_end);
        row += 1;
        loop {
            // info!("\t\tstepping over {row} {col}: {}", space[row as usize][col as usize]);
            steps.push(space[row as usize][col as usize].clone());

            if row == row_end {
                break
            }
            row += 1;
        }
    }

    let col_end = c1.max(c2);
    if col != col_end {
        assert!(col < col_end);
        col += 1;
        loop {
            // info!("\t\tstepping over {row} {col}: {}", space[row as usize][col as usize]);
            steps.push(space[row as usize][col as usize].clone());
            if col == col_end {
                break
            }
            col += 1;
        }
    }
    // info!("Stepped over {steps:?}");

    let mut result = 0;
    let mut visited_spaces = HashSet::new();
    for step in steps {
        match step {
            Star|Empty => {
                result += 1;
            }
            Space(id_vec) => {
                if id_vec.len() == 1 {
                    let id = id_vec[0];
                    visited_spaces.insert(id);
                    result += size;
                } else {
                    let id1 = id_vec[0];
                    let id2 = id_vec[1];
                    if visited_spaces.contains(&id1) && visited_spaces.contains(&id2){
                        //noop
                    } else if visited_spaces.contains(&id1) {
                        visited_spaces.insert(id1);
                        result += size;
                    } else if visited_spaces.contains(&id2) {
                        visited_spaces.insert(id2);
                        result += size;
                    } else {
                        panic!("can't happen? :S")
                    }
                }
            }
        }
    }

    result
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Cell {
    Star,Space(Vec<usize>),Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Star => write!(f,"#"),
            Space(s) => write!(f,"{}", s.first().unwrap()),
            Empty => write!(f,"."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day11.txt");
        assert_eq!(part1(input), 9795148);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day11.txt");
        assert_eq!(part2(input), 650672493820);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#,
        );

        assert_eq!(result, 374)
    }

    #[test]
    fn part2_provided_example() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        assert_eq!(solve_part2(input, 10), 1030);
        assert_eq!(solve_part2(input, 100), 8410);
    }
}
