pub fn main() {
    println!("Day 6:");
    let data = include_str!("../input.txt");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}

fn part1(data: &str) -> u32 {
    let times = data.lines()
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, time)| time)
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let records = data.lines()
        .skip(1)
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, records)| records)
        .unwrap()
        .split_whitespace()
        .map(|record| record.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return times.iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            (0..time).fold(0, |wins, seconds_held| {
                let distance = (time - seconds_held) * seconds_held;
                if distance > record {
                    wins + 1
                } else {
                    wins
                }
            })
        })
        .fold(1, |acc, wins| acc * wins);

}


fn part2(data: &str) -> u32 {
    let time = data.lines()
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, time)| time)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();




    let record = data.lines()
        .skip(1)
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, records)| records)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();


    // full iteration is too slow, try binary search to find the start and end of the winning range
    let mid = time / 2;

    let mut l = 0;
    let mut r = mid;

    while l <= r {
        let m = (l + r) / 2;
        let distance = (time - m) * m;
        if distance > record {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    let start_winning = l;


    l = mid;
    r = time;

    while l <= r {
        let m = (l + r) / 2;
        let distance = (time - m) * m;
        if distance > record {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    let end_winning = r;

    return (end_winning - start_winning + 1) as u32;

}
