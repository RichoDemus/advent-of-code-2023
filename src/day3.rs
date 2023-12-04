use crate::day3::Cell::Number;
use crate::day3::Cell::Symbol;
use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut board = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        board.push(row);
    }
    board
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let board = parse(input);

    let mut result = 0;
    let mut current_number = 0;
    let mut has_symbol = false;
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_numeric() {
                let number = cell.to_digit(10).unwrap();
                current_number *= 10;
                current_number += number;
                if get_eight_neighbours(x, y)
                    .into_iter()
                    .map(|(x, y)| board.get(y).and_then(|row| row.get(x)).unwrap_or(&'.'))
                    .copied()
                    .any(is_symbol)
                {
                    has_symbol = true;
                }
            } else {
                if has_symbol {
                    result += current_number;
                }
                has_symbol = false;
                current_number = 0;
            }
        }
    }
    result
}

fn is_symbol(c: char) -> bool {
    !(c.is_numeric() || c == '.')
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Symbol(char),
    Number(u32),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol(c) => write!(f, "{c}"),
            Number(n) => write!(f, "{n}"),
        }
    }
}

#[allow(clippy::too_many_lines)]
#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let board = parse(input);
    let columns = board[0].len();

    let mut new_board: Vec<Vec<Cell>> = vec![];
    let mut current_number = 0;
    let mut length = 0;
    for (y, row) in board.iter().enumerate() {
        new_board.push(vec![Symbol('.'); columns]);
        for (x, cell) in row.iter().enumerate() {
            if cell.is_numeric() {
                let number = cell.to_digit(10).unwrap();
                current_number *= 10;
                current_number += number;
                length += 1;
            } else {
                if current_number > 0 {
                    for i in (x.saturating_sub(length))..x {
                        new_board[y][i] = Number(current_number);
                    }
                }
                new_board[y][x] = Symbol(*cell);

                current_number = 0;
                length = 0;
            }
        }
        if current_number > 0 {
            for i in (columns - length)..columns {
                new_board[y][i] = Number(current_number);
            }
        }
    }

    let mut result = 0;

    for (y, row) in new_board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Symbol(c) = *cell {
                if c == '*' {
                    let top_left = new_board
                        .get(y - 1)
                        .and_then(|row| row.get(x - 1))
                        .unwrap_or(&Symbol('.'));
                    let above = new_board
                        .get(y - 1)
                        .and_then(|row| row.get(x))
                        .unwrap_or(&Symbol('.'));
                    let top_right = new_board
                        .get(y - 1)
                        .and_then(|row| row.get(x + 1))
                        .unwrap_or(&Symbol('.'));
                    let left = new_board
                        .get(y)
                        .and_then(|row| row.get(x - 1))
                        .unwrap_or(&Symbol('.'));
                    let right = new_board
                        .get(y)
                        .and_then(|row| row.get(x + 1))
                        .unwrap_or(&Symbol('.'));
                    let bottom_left = new_board
                        .get(y + 1)
                        .and_then(|row| row.get(x - 1))
                        .unwrap_or(&Symbol('.'));
                    let below = new_board
                        .get(y + 1)
                        .and_then(|row| row.get(x))
                        .unwrap_or(&Symbol('.'));
                    let bottom_right = new_board
                        .get(y + 1)
                        .and_then(|row| row.get(x + 1))
                        .unwrap_or(&Symbol('.'));

                    // println!("{} {} {}", top_left, above, top_right);
                    // println!("{} {} {}", left, cell, right);
                    // println!("{} {} {}", bottom_left, below, bottom_right);

                    let mut numeric_neighbours = 0;
                    #[allow(clippy::match_same_arms)]
                    {
                        match (top_left, above, top_right) {
                            (Number(_), Number(_), Number(_)) => numeric_neighbours += 1,
                            (Number(_), Symbol(_), Number(_)) => numeric_neighbours += 2,
                            (Number(_), Symbol(_), Symbol(_)) => numeric_neighbours += 1,
                            (Number(_), Number(_), Symbol(_)) => numeric_neighbours += 1,
                            (Symbol(_), Symbol(_), Number(_)) => numeric_neighbours += 1,
                            (Symbol(_), Number(_), Number(_)) => numeric_neighbours += 1,
                            (Symbol(_), Number(_), Symbol(_)) => numeric_neighbours += 1,
                            _ => (),
                        }
                        match (left, right) {
                            (Number(_), Number(_)) => numeric_neighbours += 2,
                            (Number(_), Symbol(_)) => numeric_neighbours += 1,
                            (Symbol(_), Number(_)) => numeric_neighbours += 1,
                            _ => (),
                        }
                        match (bottom_left, below, bottom_right) {
                            (Number(_), Number(_), Number(_)) => numeric_neighbours += 1,
                            (Number(_), Symbol(_), Number(_)) => numeric_neighbours += 2,
                            (Number(_), Symbol(_), Symbol(_)) => numeric_neighbours += 1,
                            (Number(_), Number(_), Symbol(_)) => numeric_neighbours += 1,
                            (Symbol(_), Symbol(_), Number(_)) => numeric_neighbours += 1,
                            (Symbol(_), Number(_), Number(_)) => numeric_neighbours += 1,
                            (Symbol(_), Number(_), Symbol(_)) => numeric_neighbours += 1,
                            _ => (),
                        }
                    }
                    // println!("{cell} at ({x},{y}) has {numeric_neighbours} numeric neighbours");
                    if numeric_neighbours == 2 {
                        let neighbours = get_eight_neighbours(x, y)
                            .iter()
                            .map(|(x, y)| {
                                new_board
                                    .get(*y)
                                    .and_then(|row| row.get(*x))
                                    .unwrap_or(&Symbol('.'))
                            })
                            .filter_map(|c| if let Number(n) = c { Some(n) } else { None })
                            .collect::<HashSet<_>>();
                        // println!("\t{neighbours:?}");
                        if neighbours.len() == 1 {
                            // println!("\t\t{neighbours:?} * 2");
                            result += (neighbours.clone().into_iter().sum::<u32>()
                                * neighbours.into_iter().sum::<u32>())
                                as usize;
                        } else if neighbours.len() == 2 {
                            // println!("\t\t {}", neighbours.iter().map(|d|**d).product::<u32>());
                            result += neighbours.iter().map(|d| **d).product::<u32>() as usize;
                        } else {
                            panic!()
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn get_eight_neighbours(x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    result.insert((x.saturating_sub(1), y.saturating_sub(1)));
    result.insert((x, y.saturating_sub(1)));
    result.insert((x + 1, y.saturating_sub(1)));
    result.insert((x.saturating_sub(1), y));
    result.insert((x + 1, y));
    result.insert((x.saturating_sub(1), y + 1));
    result.insert((x, y + 1));
    result.insert((x + 1, y + 1));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day3.txt");
        assert_eq!(part1(input), 526404);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day3.txt");
        assert_eq!(part2(input), 84399773);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        );

        assert_eq!(result, 4361)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        );

        assert_eq!(result, 467835)
    }

    #[test]
    fn part2_digits_at_end() {
        let result = part2(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592...12
......755.
...$.*....
.664.598.."#,
        );

        assert_eq!(result, 467835)
    }

    #[test]
    fn part2_self_made_example1() {
        let result = part2(
            r#"..........
....*.....
...1.1....
..........
...2*2....
..........
...3.3....
....*.....
.........."#,
        );

        assert_eq!(result, 14)
    }

    #[test]
    fn part2_self_made_example2() {
        let result = part2(
            r#"..........
....3.....
....*.....
....4.....
..........
..........
..........
..........
.........."#,
        );

        assert_eq!(result, 12)
    }

    #[test]
    fn part2_self_made_example3() {
        let result = part2(
            r#".....
...23
....*
...34
.....
.....
.....
.....
....."#,
        );

        assert_eq!(result, 782)
    }
}
