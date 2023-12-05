use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .iter()
        .map(|seed| calc_location_number(*seed, &maps))
        .min()
        .unwrap()
}

fn calc_location_number(seed: usize, maps: &Vec<Maps>) -> usize {
    let mut value = seed;
    for map in maps {
        let result = map.mappings.iter().find_map(|m| m.do_map(value));
        // println!("{value} mapped to {result:?} by {}", map.name);
        value = result.unwrap_or(value);
    }
    value
}

// #[aoc(day5, part2)]
#[allow(unused)]
fn part2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    println!("{seeds:?}");
    let sum: usize = seeds.chunks(2).map(|chunk| chunk[1]).sum();
    println!("Processing {sum} numbers");

    let mut lowest = usize::MAX;
    for (i, chunk) in seeds.chunks(2).enumerate() {
        let start = chunk[0];
        let end = chunk[1];
        println!(
            "processing chunk {i}: {start}=>{} for a total of {}",
            start + end - 1,
            end
        );
        for seed in start..(start + end - 1) {
            lowest = lowest.min(calc_rec(seed, 0, maps.as_slice()));
        }
    }
    lowest
    // calc_rec(79, 0, maps.as_slice())
}

fn calc_rec(seed: usize, map: usize, mapz: &[Maps]) -> usize {
    // println!("processing seed {} at map {}", seed, map);
    mapz.get(map).map_or(seed, |maps| {
        // println!("\t maps are {:?}", maps.name);
        let mut mapped_value = None;
        for mapping in &maps.mappings {
            if let Some(mapped) = mapping.do_map(seed) {
                // println!("\t\t{} mapped to {}", seed, mapped);
                // if mapped < lowest && mapped < new_lowest {

                let new = calc_rec(mapped, map + 1, mapz);
                mapped_value = match (mapped_value, new) {
                    (None, _) => Some(new),
                    (Some(old), new) if new < old => Some(new),
                    (Some(old), _) => Some(old),
                };
                // }
            }
        }
        mapped_value.unwrap_or_else(|| calc_rec(seed, map + 1, mapz))
    })
}

#[aoc(day5, part2, slow)]
fn part2_old(input: &str) -> usize {
    let (seeds, mut maps) = parse(input);
    println!("{seeds:?}");
    let sum: usize = seeds.chunks(2).map(|chunk| chunk[1]).sum();
    println!("Processing {sum} numbers");

    let mut lowest = usize::MAX;
    for (i, chunk) in seeds.chunks(2).enumerate() {
        let start = chunk[0];
        let end = chunk[1];
        println!(
            "processing chunk {i}: {start}=>{} for a total of {}",
            start + end - 1,
            end
        );
        for seed in start..(start + end - 1) {
            let mut value = seed;
            for map in &mut maps {
                let result = map
                    .mappings
                    .iter()
                    .map(|m| (m, m.do_map(value)))
                    .filter(|(_, result)| result.is_some())
                    .collect::<Vec<_>>();
                value = if let Some((_m, Some(result))) = result.first().copied() {
                    // if m.destination_range_start > lowest_seen {
                    //     // println!("\t skipping {} because it is greater than {lowest_seen}",m.destination_range_start);
                    //     skips += 1;
                    //     continue 'next_number;
                    // }
                    result
                } else {
                    value
                };
                // let result = map.mappings.iter()
                //     .filter(|m| {
                //         if m.destination_range_start <= lowest_seen {
                //             true
                //         } else {
                //             skips += 1;
                //             // println!("\t skipping {} because it is greater than {lowest_seen}",m.destination_range_start);
                //             false
                //         }
                //     })
                //     .filter_map(|m| m.do_map(value)).next();
                // println!("{value} mapped to {result:?} by {}", map.name);
                // value = result.unwrap_or(value);
                map.lowest_destination_we_have_seen = value;
            }
            // println!("skipped {skips} for {seed}");
            lowest = lowest.min(value);
        }
    }
    lowest
}

fn parse(input: &str) -> (Vec<usize>, Vec<Maps>) {
    let (seed, mappings) = input.split_at(input.find("\n\n").unwrap());
    let seeds: Vec<usize> = seed
        .replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mappings = mappings
        .trim()
        .split("\n\n")
        .map(|line| {
            let (name, values) = line.split_at(line.find(':').unwrap());
            let name = name.replace(" map", "");
            let values = values.replace(":\n", "");
            let mappings = values
                .lines()
                .map(|line| {
                    let mut lines = line.split(' ').map(|x| x.parse().unwrap());
                    Map {
                        destination_range_start: lines.next().unwrap(),
                        source_range_start: lines.next().unwrap(),
                        range_length: lines.next().unwrap(),
                    }
                })
                .collect::<Vec<_>>();
            Maps {
                _name: name,
                mappings,
                lowest_destination_we_have_seen: usize::MAX,
            }
        })
        .collect::<Vec<_>>();

    (seeds, mappings)
}

#[derive(Debug)]
struct Maps {
    _name: String,
    mappings: Vec<Map>,
    lowest_destination_we_have_seen: usize,
}

#[derive(Debug)]
struct Map {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}
impl Map {
    fn do_map(&self, value: usize) -> Option<usize> {
        let source_min = self.source_range_start;
        let source_max = source_min + self.range_length;
        let destination_min = self.destination_range_start;
        //let _destination_max = destination_min + self.range_length;

        if value >= source_max || value < source_min {
            None
        } else {
            let diff = value - source_min;
            let res = diff + destination_min;
            // println!("\t{value} mapped to {res}, diff {diff} by {self:?}", );
            assert!(diff <= self.range_length);
            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2023/day5.txt");
        assert_eq!(part1(input), 323142486);
    }

    // todo room for optimuzing this day
    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2023/day5.txt");
    //     assert_ne!(part2_old(input), 79874952);
    //     assert_ne!(part2_old(input), 952477129);
    //     assert_eq!(part2_old(input), 79874951);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
        );

        assert_eq!(result, 35)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
        );

        assert_eq!(result, 46)
    }
    #[test]
    fn part2_provided_example_old() {
        let result = part2_old(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
        );

        assert_eq!(result, 46)
    }
}
