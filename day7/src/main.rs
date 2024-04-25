use std::{cmp, collections::HashMap};

pub fn main() {
    println!("Day 7:");
    let data = include_str!("../input.txt");
    println!("Part 1: {}", part1(data));
    // println!("Part 2: {}", part2(data));
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    value: i32,
    cards: Vec<u8>,
}

fn card_value(c: u8) -> u8 {
    match c as char {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c - b'0',
    }

}

fn calculate_hand_type(cards: &[u8]) -> HandType {
    let mut counts = HashMap::with_capacity(5);
    cards.iter()
        .for_each(|card| {
            counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        });

    let mut count_of_counts = [0; 5];

    counts.into_values()
        .for_each(|cnt| {
            count_of_counts[(cnt - 1) as usize] += 1;
        });

    if count_of_counts[4] > 0 {
        return HandType::FiveOfAKind;
    } else if count_of_counts[3] > 0 {
        return HandType::FourOfAKind;
    } else if count_of_counts[2] > 0 {
        if count_of_counts[1] > 0 { return HandType::FullHouse; }
        else { return HandType::ThreeOfAKind; }
    } else if count_of_counts[1] > 0 {
        if count_of_counts[1] == 2 { return HandType::TwoPair; }
        else { return HandType::OnePair; }
    } else {
        return HandType::HighCard;
    }

}


fn part1(data: &str) -> i32 {
    let mut hands: Vec<Hand> = data.lines()
        .map(|line| {
            let (cards, val) = line.split_once(' ').unwrap();

            let hand = Hand {
                hand_type: calculate_hand_type(cards.as_bytes()),
                value: val.parse().unwrap(),
                cards: cards.as_bytes().iter().map(|c| card_value(*c)).collect(),
            };

            hand
        }).collect();


    hands.sort_by(|a, b| {
        a.hand_type.cmp(&b.hand_type)
            .then_with(|| a.cards.cmp(&b.cards))
    });

    hands.into_iter()
        .enumerate()
        .fold(0, |res, (rank, hand)| {
            res + (hand.value * (rank + 1) as i32)
        })
}
