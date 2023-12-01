use phf::phf_map;

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut res: usize = 0;
            'done: for x in line.chars() {
                if let Some(left) = x.to_digit(10) {
                    for x in line.chars().rev() {
                        if let Some(right) = x.to_digit(10) {
                            res = (10 * left + right) as usize;
                            break 'done;
                        }
                    }
                }
            }
            res
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    input.lines().map(solve).sum()
}

fn solve(line: &str) -> usize {
    10 * read_ltr(line) + read_rtl(line)
}

fn read_ltr(mut line: &str) -> usize {
    loop {
        if let Some(digit) = str_start_to_digit(line) {
            return digit;
        }
        line = &line[1..];
    }
}

fn read_rtl(mut line: &str) -> usize {
    loop {
        if let Some(digit) = str_end_to_digit(line) {
            return digit;
        }
        line = &line[0..line.len() - 1];
    }
}

static DIGITS: phf::Map<&'static str, usize> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn str_start_to_digit(word: &str) -> Option<usize> {
    for digit in DIGITS.keys() {
        if word.starts_with(digit) {
            return DIGITS.get(digit).copied();
        }
    }
    None
}

fn str_end_to_digit(word: &str) -> Option<usize> {
    for digit in DIGITS.keys() {
        if word.ends_with(digit) {
            return DIGITS.get(digit).copied();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day1.txt");
        assert_eq!(part1(input), 55607);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day1.txt");
        assert_eq!(part2(input), 55291);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
        );

        assert_eq!(result, 142)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#,
        );

        assert_eq!(result, 281)
    }
}
