fn main() {
    let data = include_str!("../input.txt");
    println!("Part 1: {}", part1(data));
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
    0
}
