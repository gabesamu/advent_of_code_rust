use std::{cmp, collections::HashMap};

pub fn main() {
    println!("Day 7:");
    let data = include_str!("../input.txt");
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}

pub fn speed_test() {
    let data = include_str!("../input.txt");
    part1(data);
    part2(data);
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

fn calculate_hand_type(cards: &Vec<u8>) -> HandType {
    let mut counts = HashMap::with_capacity(5);
    cards.iter()
        .for_each(|card| {
            counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        });

    let mut count_of_counts = [0; 6];

    let joker_count = counts.remove(&1).unwrap_or(0);


    counts.into_values()
        .for_each(|cnt| {
            count_of_counts[(cnt) as usize] += 1;
        });


    match joker_count {
        0 => {
            if count_of_counts[5] == 1 { return HandType::FiveOfAKind; }
            else if count_of_counts[4] == 1 { return HandType::FourOfAKind; }
            else if count_of_counts[3] == 1 && count_of_counts[2] == 1 { return HandType::FullHouse; }
            else if count_of_counts[3] == 1 { return HandType::ThreeOfAKind; }
            else if count_of_counts[2] == 2 { return HandType::TwoPair; }
            else if count_of_counts[2] == 1 { return HandType::OnePair; }
            else { return HandType::HighCard; }
        },
        1 => {
            if count_of_counts[4] == 1 { return HandType::FiveOfAKind; }
            else if count_of_counts[3] == 1 { return HandType::FourOfAKind; }
            else if count_of_counts[2] == 2 { return HandType::FullHouse; }
            else if count_of_counts[2] == 1 { return HandType::ThreeOfAKind; }
            else { return HandType::OnePair; }
        },
        2 => {
            if count_of_counts[3] == 1 { return HandType::FiveOfAKind; }
            else if count_of_counts[2] == 1 { return HandType::FourOfAKind; }
            else { return HandType::ThreeOfAKind; }
        },
        3 => {
            if count_of_counts[2] == 1 { return HandType::FiveOfAKind; }
            else { return HandType::FourOfAKind; }
        },
        4 => {
            return HandType::FiveOfAKind;
        },
        5 => {
            return HandType::FiveOfAKind;
        },
        _ => {
            return HandType::HighCard;
        }
    }
}


fn part1(data: &str) -> i32 {
    let mut hands: Vec<Hand> = data.lines()
        .map(|line| {
            let (cards_str, val) = line.split_once(' ').unwrap();
            let cards: Vec<u8> = cards_str.as_bytes().iter().map(|c| card_value(*c)).collect();

            let hand = Hand {
                hand_type: calculate_hand_type(&cards),
                value: val.parse().unwrap(),
                cards: cards,
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


fn card_value_with_jokers(c: u8) -> u8 {
    match c as char {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c - b'0',
    }
}


fn part2(data: &str) -> i32 {
    let mut hands: Vec<Hand> = data.lines()
        .map(|line| {
            let (cards_str, val) = line.split_once(' ').unwrap();
            let cards = cards_str.as_bytes().iter().map(|c| card_value_with_jokers(*c)).collect();
            let hand = Hand {
                hand_type: calculate_hand_type(&cards),
                value: val.parse().unwrap(),
                cards: cards,
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
