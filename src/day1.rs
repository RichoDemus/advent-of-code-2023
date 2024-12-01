use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|l| {
        let (l, r) = l.split_ascii_whitespace().collect_tuple().unwrap();
        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    debug_assert_eq!(left.len(), right.len());

    left.into_iter().zip(right).map(|(l,r)|if l > r { l -r} else {r - l}).sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|l| {
        let (l, r) = l.split_ascii_whitespace().collect_tuple().unwrap();
        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    });

    let mut score = 0;

    for x in left {
        let count_similar = right.iter().filter(|&i| *i == x).count();
        score += x * count_similar;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2024/day1.txt");
        assert_eq!(part1(input), 765748);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2024/day1.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_provided_example() {
        let _ = env_logger::builder()
            .filter_module("advent_of_code_2024", log::LevelFilter::Info)
            .try_init();
        let result = part1(
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        );

        assert_eq!(result, 11)
    }

    #[test]
    fn part2_provided_example() {
    let _ = env_logger::builder()
    .filter_module("advent_of_code_2024", log::LevelFilter::Info)
    .try_init();
        let result = part2(
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        );

        assert_eq!(result, 31)
    }
}
