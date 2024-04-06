pub fn main() {
    let data = include_str!("../input.txt");
    println!("Day 4:");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}

fn part1(data: &str) -> u32 {
    return data
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (winning, have) = nums.split_once('|').unwrap();
            let winning = winning.split_whitespace().collect::<Vec<&str>>();
            have.split_whitespace()
                .filter(|&word| winning.contains(&word))
                .count()
        })
        .map(|mut num_of_winning| {
            let mut val = 0;
            while num_of_winning > 0 {
                if val == 0 {
                    val = 1;
                } else {
                    val *= 2;
                }
                num_of_winning -= 1;
            }
            val
        })
        .sum::<u32>();
}

fn part2(data: &str) -> u32 {
    let num_of_cards = data.lines().count();
    let mut card_amts: Vec<u32> = vec![1; num_of_cards];
    data.lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (winning, have) = nums.split_once('|').unwrap();

            let winning = winning.split_whitespace().collect::<Vec<&str>>();
            have.split_whitespace()
                .filter(|&word| winning.contains(&word))
                .count()
        })
        .enumerate()
        .for_each(|(i, num_of_winning)| {
            for offset in 1..num_of_winning + 1 {
                card_amts[i + offset] += card_amts[i];
            }
        });
    return card_amts.iter().sum();
}
