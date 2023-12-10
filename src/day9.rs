use aoc_runner_derive::aoc;
use itertools::Itertools;
use log::info;

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    input.lines().map(extrapolate_one).sum()
}

fn extrapolate_one(line: &str) -> i64 {
    let data = line
        .split(' ')
        .map(|s| match s.parse::<i64>() {
            Ok(u) => u,
            Err(e) => panic!("Failed to parse {s}: {e}"),
        })
        .collect_vec();

    let mut data = vec![data];
    loop {
        let last = data.last().unwrap();
        if last.iter().all(|i| *i == 0) {
            break;
        }
        let mut next = vec![];
        for i in 0..(last.len() - 1) {
            next.push(last[i + 1] - last[i]);
        }
        data.push(next);
    }
    {
        print(&data);
        data.last_mut().unwrap().push(0);
        let length = data.len() - 2;
        let last_elem_of_last_non_zero = *data.get(length).unwrap().last().unwrap();
        data.get_mut(length)
            .unwrap()
            .push(last_elem_of_last_non_zero);
        print(&data);
    }
    for i in (0..(data.len() - 2)).rev() {
        info!("looking at {:?}", data[i]);
        let last_item_of_prev = *data[i + 1].last().unwrap();
        let last = *data[i].last().unwrap();
        data.get_mut(i).unwrap().push(last + last_item_of_prev);
    }
    print(&data);
    *data.first().unwrap().last().unwrap()
}

fn print(data: &[Vec<i64>]) {
    for (i, values) in data.iter().enumerate() {
        let mut out = String::new();
        for _ in 0..i {
            out.push(' ');
        }
        for val in values {
            out += format!(" {val}").as_str();
        }
        info!("{out}");
    }
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    input.lines().map(extrapolate_front).sum()
}

fn extrapolate_front(line: &str) -> i64 {
    let data = line
        .split(' ')
        .map(|s| match s.parse::<i64>() {
            Ok(u) => u,
            Err(e) => panic!("Failed to parse {s}: {e}"),
        })
        .collect_vec();

    let mut data = vec![data];
    loop {
        let last = data.last().unwrap();
        if last.iter().all(|i| *i == 0) {
            break;
        }
        let mut next = vec![];
        for i in 0..(last.len() - 1) {
            next.push(last[i + 1] - last[i]);
        }
        data.push(next);
    }
    {
        print(&data);
        data.last_mut().unwrap().push(0);
        let length = data.len() - 2;
        let first_elem_of_last_non_zero = *data.get(length).unwrap().first().unwrap();
        data.get_mut(length)
            .unwrap()
            .insert(0, first_elem_of_last_non_zero);
        print(&data);
    }
    for i in (0..(data.len() - 2)).rev() {
        info!("looking at {:?}", data[i]);
        let first_item_of_prev = *data[i + 1].first().unwrap();
        let first = *data[i].first().unwrap();
        data.get_mut(i)
            .unwrap()
            .insert(0, first - first_item_of_prev);
    }
    print(&data);
    *data.first().unwrap().first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day9.txt");
        assert_eq!(part1(input), 1772145754);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day9.txt");
        assert_eq!(part2(input), 867);
    }

    #[test]
    fn part1_provided_example() {
        assert_eq!(extrapolate_one("0 3 6 9 12 15"), 18);
        assert_eq!(extrapolate_one("1 3 6 10 15 21"), 28);
        assert_eq!(extrapolate_one("10 13 16 21 30 45"), 68);

        let result = part1(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
        );

        assert_eq!(result, 114);
    }

    #[test]
    fn part2_provided_example() {
        assert_eq!(extrapolate_front("0 3 6 9 12 15"), -3);
        assert_eq!(extrapolate_front("1 3 6 10 15 21"), 0);
        assert_eq!(extrapolate_front("10 13 16 21 30 45"), 5);

        let result = part2(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
        );

        assert_eq!(result, 2);
    }
}
