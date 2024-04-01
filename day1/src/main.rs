fn main() {
    let data = include_str!("../input.txt");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}


fn part1(data: &str) -> i32 {
    return data
        .lines()
        .map(|line| {
            line.chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap() as i32
                * 10
                + line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32
        })
        .sum::<i32>();
}

fn part2(data: &str) -> i32 {
    return data
        .lines()
        .map(|line| {
            (0..line.len()).find_map(|i| get_num(line, i)).unwrap() * 10
                + (0..line.len())
                    .rev()
                    .find_map(|i| get_num(line, i))
                    .unwrap()
        })
        .sum::<i32>();
}

#[inline(always)]
fn get_num(line: &str, i: usize) -> Option<i32> {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let byte_index = line.char_indices().nth(i)?.0;

    line[byte_index..]
        .chars()
        .next()
        .and_then(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).map(|d| d as i32)
            } else {
                None
            }
        })
        .or_else(|| {
            NUMS.iter().enumerate().find_map(|(index, num)| {
                if line[byte_index..].starts_with(num) {
                    Some(index as i32 + 1)
                } else {
                    None
                }
            })
        })
}
