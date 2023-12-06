use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    println!("raw: {input}");
    let (times, distances) = input.split_at(input.find('\n').unwrap());

    let times = times
        .replace("Time:", "")
        .trim()
        .split(' ')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    let distances = distances
        .replace("Distance:", "")
        .trim()
        .split(' ')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    println!("times: {times:?}");
    println!("distances: {distances:?}");

    times.into_iter().zip(distances).map(calc_part1).product()
}

fn calc_part1((race_duration, distance_to_beat): (usize, usize)) -> usize {
    let mut number_of_records_beat = 0;
    for wait_time in 0..=race_duration {
        // println!("checking if waiting {wait_time} of {race_duration} beats {distance_to_beat}");
        let speed = wait_time;
        let runtime = race_duration - wait_time;
        let distance = speed * runtime;
        if distance > distance_to_beat {
            number_of_records_beat += 1;
        }
    }
    number_of_records_beat
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    println!("raw: {input}");
    let (times, distances) = input.split_at(input.find('\n').unwrap());

    let times = times
        .replace("Time:", "")
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let distances = distances
        .replace("Distance:", "")
        .replace([' ', '\n'], "")
        .parse::<usize>()
        .unwrap();
    println!("times: {times:?}");
    println!("distances: {distances:?}");

    calc_part1((times, distances))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day6.txt");
        assert_eq!(part1(input), 608902);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day6.txt");
        assert_eq!(part2(input), 46173809);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"Time:      7  15   30
Distance:  9  40  200"#,
        );

        assert_eq!(result, 288)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"Time:      7  15   30
Distance:  9  40  200"#,
        );

        assert_eq!(result, 71503)
    }
}
