fn main() {
    let data = include_str!("../input.txt");
    println!("Day 5:");
    println!("Part 1: {}", part1(data));
    // println!("Part 2: {}", part2(data));
}

fn part1(data: &str) -> u64 {
    let mut map_lines = data.lines().skip(2);
    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..7)
        .map(|_| {
            (&mut map_lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let range_data = line
                        .split_whitespace()
                        .map(|word| word.parse().unwrap())
                        .collect::<Vec<u64>>();
                    (range_data[1]..range_data[1] + range_data[2], range_data[0])
                })
                .collect()
        })
        .collect();

    return data
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, seeds)| seeds)
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .map(|seed| {
            maps.iter().fold(seed, |curr, map| {
                map.iter()
                    .find(|(range, _)| range.contains(&curr))
                    .map(|(from, to)| to + curr - from.start)
                    .unwrap_or(curr)
            })
        })
        .min()
        .unwrap();
}
