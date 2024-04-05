fn main() {
    let data = include_bytes!("../input.txt");
    println!("Day 3:");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}

fn part1(data: &[u8]) -> u32 {
    let width = (data.iter().position(|&c| c == b'\n').unwrap() + 1) as isize;

    return (0..data.len() - 2)
        // Find the index of the first digit in each number
        .filter(|i| {
            data[*i].is_ascii_digit()
                && !data
                    .get(i.wrapping_sub(1))
                    .map_or_else(|| false, |&c| c.is_ascii_digit())
        })
        // Get the full number at each index and number of digits
        .map(|i| {
            let mut num = 0;
            let mut j = i;
            let mut num_of_digits = 0;
            while data[j].is_ascii_digit() {
                num = num * 10 + (data[j] - b'0') as u32;
                num_of_digits += 1;
                j += 1;
            }
            (i, num, num_of_digits)
        })
        // Filter out numbers that are not surrounded by a symbol(not a valid part)
        .filter(|(i, _num, num_of_digits)| {
            (-width - 1..-width + *num_of_digits + 1)
                .chain([-1, *num_of_digits])
                .chain(width - 1..width + *num_of_digits + 1)
                .any(|j| {
                    data.get((*i as isize + j) as usize)
                        .map_or_else(|| false, |&c| c != b'.' && c.is_ascii_punctuation())
                })
        })
        // Sum the valid parts
        .map(|(_, num, _)| num)
        .sum::<u32>();
}


fn part2(data: &[u8]) -> u32 {
    let width = (data.iter().position(|&c| c == b'\n').unwrap() + 1) as isize;

    return (0..data.len())
        .filter(|i| data[*i] == b'*')
        .map(|i| {
            let mut nums = Vec::new();

            (-width - 1..-width + 2)
                .chain([-1, 1])
                .chain(width - 1..width + 2)
                .for_each(|j| {
                    if data.get((i as isize + j) as usize).map_or_else(|| false, |&c| c.is_ascii_digit()) {
                        let (num, start) = get_num(data, (i as isize + j) as usize);
                        nums.push((num, start));

                    }
                });

            nums.dedup();
            if nums.len() == 2 {
                return nums[0].0 * nums[1].0;
            }
            return 0;
        }).sum();

}



fn get_num(data: &[u8], i: usize) -> (u32, usize) {
    let mut num = 0;
    let mut j = i;

    while data[j - 1].is_ascii_digit() {
        j -= 1;
    }
    let start = j;
    while data[j].is_ascii_digit() {
        num = num * 10 + (data[j] - b'0') as u32;
        j += 1;
    }

    return (num, start);
}
