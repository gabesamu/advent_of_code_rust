pub fn main() {
    let data = include_str!("../input.txt");
    println!("Day 2:");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}

fn part1(data: &str) -> u32 {
    return data
        .lines()
        .map(|line| {
            let (id_str, cubes) = line.split_once(": ").unwrap();

            let id = id_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let valid: bool = cubes.split(&[';', ','][..]).all(|cube| {
                let (cnt, color) = cube.trim().split_once(' ').unwrap();
                let cnt = cnt.parse::<u32>().unwrap();
                match color {
                    "red" => cnt <= 12,
                    "green" => cnt <= 13,
                    "blue" => cnt <= 14,
                    _ => false,
                }
            });

            return if valid { id } else { 0 };
        })
        .sum();
}

fn part2(data: &str) -> u32 {
    return data
        .lines()
        .map(|line| {
            let (_, cubes) = line.split_once(": ").unwrap();

            let mut green = 0;
            let mut blue = 0;
            let mut red = 0;

            cubes.split(&[';', ','][..]).for_each(|cube| {
                let (cnt, color) = cube.trim().split_once(' ').unwrap();
                let cnt = cnt.parse::<u32>().unwrap();
                match color {
                    "red" => red = red.max(cnt),
                    "green" => green = green.max(cnt),
                    "blue" => blue = blue.max(cnt),
                    _ => (),
                }
            });

            return green * blue * red;
        })
        .sum();
}
