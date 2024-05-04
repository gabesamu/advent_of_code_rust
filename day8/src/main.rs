use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt");
    println!("Day 8:");
    println!("{}", part1(data));
    println!("{}", part2(data));
}

pub fn speed_test() {
    let data = include_str!("../input.txt");
    part1(data);
    part2(data);
}

fn part1(data: &str) -> i32 {
    let directions = data.lines().next().unwrap();

    let mut nodes = HashMap::with_capacity(800);

    data.lines().skip(2).for_each(|line| {
        let (node, directions) = line.split_once(" = ").unwrap();
        let (left, right) = directions
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();

        nodes.insert(node, (left, right));
    });

    let mut steps = 0;
    let mut curr = "AAA";

    let dir_len = directions.len();

    while curr != "ZZZ" {
        directions.chars().for_each(|c| {
            let (left, right) = nodes.get(curr).unwrap();
            let next = if c == 'L' { left } else { right };
            curr = next;
        });

        steps += dir_len;
    }

    return steps as i32;
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn part2(data: &str) -> i64 {
    let directions = data.lines().next().unwrap();

    let mut nodes = HashMap::with_capacity(800);
    let mut start_nodes = Vec::new();

    data.lines().skip(2).for_each(|line| {
        let (node, directions) = line.split_once(" = ").unwrap();
        let (left, right) = directions
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();

        nodes.insert(node, (left, right));

        if node.ends_with("A") {
            start_nodes.push(node);
        }
    });

    let dir_len = directions.len();

    let steps = start_nodes
        .iter()
        .map(|start| {
            let mut steps = 0;
            let mut curr = *start;

            while !curr.ends_with("Z") {
                directions.chars().for_each(|c| {
                    let (left, right) = nodes.get(curr).unwrap();
                    let next = if c == 'L' { left } else { right };
                    curr = next;
                });

                steps += dir_len;
            }

            return steps as i64;
        })
        .fold(1, |ans, steps| lcm(ans, steps));

    return steps as i64;
}
