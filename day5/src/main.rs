
use std::{cell::RefCell, iter::from_fn};

pub fn main() {
    let data = include_str!("../input.txt");
    println!("Day 5:");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
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



fn part2(data: &str) -> u64 {
    let mut map_lines = data.lines().skip(2);
    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..7)
        .map(|_| {
            let mut map = (&mut map_lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let range_data = line
                        .split_whitespace()
                        .map(|word| word.parse().unwrap())
                        .collect::<Vec<u64>>();
                    (range_data[1]..range_data[1] + range_data[2], range_data[0])
                })
                .collect::<Vec<_>>();
            map.sort_by_key(|(range, _)| range.start);
            map
        })
        .collect();

    let mut seeds = data
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, seeds)| seeds)
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap());

    return from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(start, len)| start..start + len)
        .flat_map(|seed_range| {
            maps.iter().fold(vec![seed_range], |seed_ranges, map| {
                seed_ranges.into_iter()
                .flat_map(|seed_range| {
                    let seed_cell = RefCell::new(seed_range);
                    map.iter()
                        .take_while(|_| !seed_cell.borrow().is_empty())
                        .fold(vec![], |mut mapped_ranges, (to, dest_start)| {
                            let mut seed_range = seed_cell.borrow_mut();

                            if seed_range.start < to.end {
                                let len = seed_range.end.min(to.end) - seed_range.start;
                                let mapped_to = dest_start + seed_range.start - to.start;
                                mapped_ranges.push(mapped_to..mapped_to + len);
                                seed_range.start += len;
                            }
                            mapped_ranges
                        })
                })
                .collect()
            })

        })
        .map(|range| range.start)
        .min()
        .unwrap();

}
