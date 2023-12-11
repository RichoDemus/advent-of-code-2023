use crate::day10::Pipe::{Ground, Start, EW, NE, NS, NW, SE, SW};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let pipes: Vec<Vec<Pipe>> = parse(input);

    info!("pipes:");
    for pipe in &pipes {
        info!("{}", pipe.iter().map(|p| format!("{p}")).join(""));
    }

    let (start_row, start_col) = find_start(&pipes);
    info!(
        "start is at {start_row} {start_col} => {}",
        pipes[start_row][start_col]
    );

    let mut visited_nodes = vec![vec![(start_row, start_col)], vec![(start_row, start_col)]];

    let mut coordinates = figure_out_coords_of_connecting_pipes(&pipes, start_row, start_col);

    let mut steps = 1;
    loop {
        steps += 1;
        for (i, (row, col)) in coordinates.iter_mut().enumerate() {
            visited_nodes.get_mut(i).unwrap().push((*row, *col));
            let current_node = &pipes[*row][*col];
            let next_coordinates = match current_node {
                NS => vec![(*row - 1, *col), (*row + 1, *col)],
                EW => vec![(*row, *col - 1), (*row, *col + 1)],
                NE => vec![(*row - 1, *col), (*row, *col + 1)],
                NW => vec![(*row - 1, *col), (*row, *col - 1)],
                SW => vec![(*row + 1, *col), (*row, *col - 1)],
                SE => vec![(*row + 1, *col), (*row, *col + 1)],
                Start => panic!("we're not at start anymore"),
                Ground => panic!("were on ground :S"),
            };
            let new_nodes = next_coordinates
                .iter()
                .filter(|coord| !visited_nodes[i].contains(coord))
                .collect_vec();
            info!("{i}: {row} {col}: {current_node} next: {next_coordinates:?} - {:?} => {new_nodes:?}", visited_nodes[i]);
            let (new_row, new_col) = new_nodes[0];
            *row = *new_row;
            *col = *new_col;
        }
        let rows = coordinates.iter().map(|(row, _)| row).unique().count();
        let cols = coordinates.iter().map(|(_, col)| col).unique().count();
        if rows == 1 && cols == 1 {
            return steps;
        }
    }
}

fn figure_out_coords_of_connecting_pipes(
    pipes: &[Vec<Pipe>],
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {
    let north = &pipes[row - 1][col];
    let south = &pipes[row + 1][col];
    let west = &pipes[row][col - 1];
    let east = &pipes[row][col + 1];

    let mut result = vec![];
    if *north == NS || *north == SW || *north == SE {
        result.push((row - 1, col));
    }
    if *south == NS || *south == NE || *south == NW {
        result.push((row + 1, col));
    }
    if *west == EW || *west == SE || *west == NE {
        result.push((row, col - 1));
    }
    if *east == EW || *east == SW || *east == NW {
        result.push((row, col + 1));
    }
    result
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    let mut pipes = input
        .lines()
        .map(|line| line.chars().map(Pipe::parse).collect_vec())
        .collect_vec();

    let length = pipes.first().unwrap().len();
    pipes.insert(0, vec![Ground; length]);
    pipes.push(vec![Ground; length]);

    for row in &mut pipes {
        row.insert(0, Ground);
        row.push(Ground);
    }

    pipes
}

fn find_start(pipes: &[Vec<Pipe>]) -> (usize, usize) {
    for (row_index, row) in pipes.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if *cell == Start {
                return (row_index, col_index);
            }
        }
    }
    panic!()
}

#[derive(Clone, Eq, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NS => write!(f, "│"),
            EW => write!(f, "─"),
            NE => write!(f, "└"),
            NW => write!(f, "┘"),
            SW => write!(f, "┐"),
            SE => write!(f, "┌"),
            Ground => write!(f, "."),
            Start => write!(f, "S"),
        }
    }
}
impl Pipe {
    fn parse(c: char) -> Self {
        match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => panic!(),
        }
    }
    fn to_ascii(&self, row_offset: usize, col_offset: usize) -> bool {
        match self {
            NS => " . \n . \n . ",
            EW => "   \n...\n   ",
            NE => " . \n ..\n    ",
            NW => " . \n.. \n   ",
            SW => "   \n.. \n . ",
            SE => "   \n ..\n . ",
            Ground | Start => "   \n   \n   ",
        }
        .lines()
        .map(|line| line.chars().map(|c| c != ' ').collect_vec())
        .collect_vec()[row_offset][col_offset]
    }
}

#[allow(clippy::cognitive_complexity)]
#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut pipes: Vec<Vec<Pipe>> = parse(input);

    info!("pipes:");
    for pipe in &pipes {
        info!("{}", pipe.iter().map(|p| format!("{p}")).join(""));
    }

    let pipe_coords = find_all_pipe_coordinates(&pipes);
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    {
        let (start_row, start_col) = find_start(&pipes);
        let connections = figure_out_coords_of_connecting_pipes(&pipes, start_row, start_col);
        info!("connections: {connections:?}");
        let connections = connections
            .into_iter()
            .map(|(r, c)| {
                (
                    ((r as i32 - start_row as i32) as i8),
                    (c as i32 - start_col as i32) as i8,
                )
            })
            .collect_vec();
        info!("connections: {connections:?}");
        let start_node = figure_out_missing_node(&connections);
        *pipes
            .get_mut(start_row)
            .unwrap()
            .get_mut(start_col)
            .unwrap() = start_node;
    }
    //replace start with proper pipe
    info!("Pipe path:");
    for row in 0..pipes.len() {
        let mut s = String::new();
        for col in 0..pipes[0].len() {
            if pipe_coords.contains(&(row, col)) {
                let current_node = &pipes[row][col];
                s += format!("{current_node}").as_str();
            } else {
                s += " ";
            }
        }
        info!("{s}");
    }

    let mut big_board: Vec<Vec<bool>> = vec![];
    for row in 0..pipes.len() * 3 {
        let mut next = vec![];
        for col in 0..pipes[0].len() * 3 {
            next.push(calc_node(&pipes, &pipe_coords, row, col));
            // print!("{}", match pipe_coords.contains(&(row / 3, col / 3)) {
            //     true => format!("{}",pipes[row/3][col/3]),
            //     false => " ".to_string(),
            // })
        }
        // println!();
        big_board.push(next);
    }

    for row in &big_board {
        let mut s = String::new();
        for cell in row {
            s += if *cell { "+" } else { " " }
        }
        info!("{s}");
    }

    let big_board = fill_iter(big_board);
    for row in &big_board {
        let mut s = String::new();
        for cell in row {
            s += if *cell { "+" } else { " " }
        }
        info!("{s}");
    }

    let mut shrunk = vec![];
    for row in 0..(big_board.len() / 3) {
        let row2 = row * 3;
        let mut n = vec![];
        'next: for col in 0..(big_board[0].len() / 3) {
            let col2 = col * 3;
            // info!("checking {row} {col}");
            for dr in 0..3 {
                for dc in 0..3 {
                    if big_board[row2 + dr][col2 + dc] {
                        // info!("\tblock at {} {}", row + dr,col + dc );
                        n.push('+');
                        continue 'next;
                    }
                }
            }
            // info!("found a whole nine at {row} {col}");
            n.push(' ');
        }
        shrunk.push(n);
    }

    info!("pipes:");
    for pipe in &shrunk {
        info!("{}", pipe.iter().map(|p| format!("{p}")).join(""));
    }

    shrunk
        .into_iter()
        .flat_map(IntoIterator::into_iter)
        .filter(|c| *c == ' ')
        .count()
}

#[allow(clippy::cast_sign_loss)]
fn fill_iter(mut board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let r_max: i32 = board.len().try_into().unwrap();
    let c_max: i32 = board[0].len().try_into().unwrap();
    let mut stack = VecDeque::new();
    stack.push_front((0, 0));
    while let Some((row, col)) = stack.pop_front() {
        board[row as usize][col as usize] = true;
        let neighbours = vec![
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|(row, col)| {
            if *row < 0
                || *row >= r_max
                || *col < 0
                || *col >= c_max
                || stack.contains(&(*row, *col))
            {
                false
            } else {
                !board[*row as usize][*col as usize]
            }
        })
        .collect_vec();
        // info!("we're at {row} {col}, neighbours: {neighbours:?}");
        for n in &neighbours {
            stack.push_back(*n);
        }
    }

    board
}

fn figure_out_missing_node(nodes: &[(i8, i8)]) -> Pipe {
    assert_eq!(nodes.len(), 2);

    match (nodes[0], nodes[1]) {
        ((0, -1), (0, 1)) => EW,
        ((1, 0), (0, 1)) => SE,
        ((1, 0), (0, -1)) => SW,
        ((a, b), (c, d)) => panic!("figure_out_missing_node not handling (({a},{b}),( {c},{d}))"),
    }
}

fn find_all_pipe_coordinates(pipes: &[Vec<Pipe>]) -> Vec<(usize, usize)> {
    let (start_row, start_col) = find_start(pipes);
    info!(
        "start is at {start_row} {start_col} => {}",
        pipes[start_row][start_col]
    );

    let mut visited_nodes = vec![(start_row, start_col)];

    let (mut row, mut col) = figure_out_coords_of_connecting_pipes(pipes, start_row, start_col)[0];

    loop {
        visited_nodes.push((row, col));
        let current_node = &pipes[row][col];
        let next_coordinates = match current_node {
            NS => vec![(row - 1, col), (row + 1, col)],
            EW => vec![(row, col - 1), (row, col + 1)],
            NE => vec![(row - 1, col), (row, col + 1)],
            NW => vec![(row - 1, col), (row, col - 1)],
            SW => vec![(row + 1, col), (row, col - 1)],
            SE => vec![(row + 1, col), (row, col + 1)],
            Start => figure_out_coords_of_connecting_pipes(pipes, row, col),
            Ground => panic!("were on ground :S"),
        };
        let new_nodes = next_coordinates
            .iter()
            .filter(|coord| !visited_nodes.contains(coord))
            .collect_vec();
        // info!("{row} {col}: {current_node} next: {next_coordinates:?} - {:?} => {new_nodes:?}", visited_nodes);
        if new_nodes.is_empty() && next_coordinates.contains(&(start_row, start_col)) {
            return visited_nodes;
        }
        let (new_row, new_col) = new_nodes[0];
        row = *new_row;
        col = *new_col;
    }
}

fn calc_node(pipes: &[Vec<Pipe>], pipe_coords: &[(usize, usize)], row: usize, col: usize) -> bool {
    if pipe_coords.contains(&(row / 3, col / 3)) {
        let node = &pipes[row / 3][col / 3];
        //
        // if row == 5 && (col == 11 || col == 12 || col == 13) {
        //     info!("{node} at {} {}, row mod {} col mod {}", row/3, col/3, row%3, col%3);
        // }
        // if *node == Ground {
        //     info!("{node} at {} {}, row mod {} col mod {}. bool: {}", row/3, col/3, row%3, col%3, node.to_ascii(row%3, col%3));
        // }

        node.to_ascii(row % 3, col % 3)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day10.txt");
        assert_eq!(part1(input), 7005);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day10.txt");
        let res = part2(input);
        assert_eq!(res, 417);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#".....
.S-7.
.|.|.
.L-J.
....."#,
        );

        assert_eq!(result, 4)
    }

    #[test]
    fn part2_provided_example1() {
        let result = part2(
            r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#,
        );

        assert_eq!(result, 4)
    }

    #[test]
    fn part2_provided_example2() {
        let result = part2(
            r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#,
        );

        assert_eq!(result, 8)
    }

    #[test]
    fn part2_provided_example3() {
        let result = part2(
            r#"...........
.S------7..
.|F----7|..
.||....||..
.||....||..
.|L-7F-J|..
.|..||..|..
.L--JL--J..
..........."#,
        );

        assert_eq!(result, 4)
    }
    #[test]
    fn part2_provided_example4() {
        let result = part2(
            r#"S-7F7
L7LJ|
FJ.FJ
|F7L7
LJL-J"#,
        );

        assert_eq!(result, 1)
    }
    #[test]
    fn part2_provided_example5() {
        let result = part2(
            r#"...............
...F7.....F7...
...||.....||...
...|L--S--J|...
.F-J.F---7.L-7.
.L--7|...|F--J.
....||...||....
....LJ...LJ....
..............."#,
        );

        assert_eq!(result, 2)
    }
    #[test]
    fn part2_provided_example6() {
        let result = part2(
            r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#,
        );

        assert_eq!(result, 10)
    }
}
