fn main() {
    let data = include_str!("../input.txt");
    println!("Day 9:");
    println!("Part 1: {}", part1(data));
    // println!("Part 2: {}", part2(data));
}

fn part1(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|mut history| {
            let mut diffs = Vec::new();
            diffs.push(*history.last().unwrap());
            while !history.iter().all(|n| *n == 0) {
                let new_history = history
                    .iter()
                    .zip(history.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect::<Vec<i32>>();
                diffs.push(*new_history.last().unwrap());
                history = new_history;
            }

            diffs.iter().rev().fold(0, |next, diff| next + diff)
        })
        .sum()
}
