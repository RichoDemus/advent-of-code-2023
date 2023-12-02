use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let reds = 12;
    let greens = 13;
    let blues = 14;
    input
        .lines()
        .map(|line| line.try_into().unwrap())
        .filter(|game: &Game| {
            game.biggest_blue() <= blues
                && game.biggest_green() <= greens
                && game.biggest_red() <= reds
        })
        .map(|g| g.id)
        .sum()
}

#[derive(Debug)]
struct Game {
    id: i32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn biggest_red(&self) -> i32 {
        self.reveals.iter().map(|r| r.red).max().unwrap_or(0)
    }
    fn biggest_green(&self) -> i32 {
        self.reveals.iter().map(|r| r.green).max().unwrap_or(0)
    }
    fn biggest_blue(&self) -> i32 {
        self.reveals.iter().map(|r| r.blue).max().unwrap_or(0)
    }
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let split = line.split(':').collect::<Vec<_>>();
        let id: i32 = split[0].replace("Game ", "").parse()?;

        let mut reveals = vec![];
        for reveal in split[1].split(';') {
            let split = reveal.split(',').collect::<Vec<_>>();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cubes in split {
                let cubes = cubes.replace(' ', "");
                if cubes.contains("red") {
                    red = cubes.replace("red", "").parse()?;
                } else if cubes.contains("green") {
                    green = cubes.replace("green", "").parse()?;
                } else if cubes.contains("blue") {
                    blue = cubes.replace("blue", "").parse()?;
                } else {
                    panic!()
                }
            }
            reveals.push(Reveal { red, blue, green });
        }
        Ok(Self { id, reveals })
    }
}

#[derive(Debug)]
struct Reveal {
    red: i32,
    blue: i32,
    green: i32,
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.try_into().unwrap())
        .map(|game: Game| game.biggest_red() * game.biggest_green() * game.biggest_blue())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day2.txt");
        assert_eq!(part1(input), 2776);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2023/day2.txt");
        assert_eq!(part2(input), 68638);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        );

        assert_eq!(result, 8)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        );

        assert_eq!(result, 2286)
    }
}
