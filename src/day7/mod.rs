use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

#[derive(Debug, Clone)]
struct HandValue {
    hand: String,
    value: isize,
}

fn hand_to_value(hand: &str) -> Hand {

    let mut handString = hand.to_string();
    handString = handString.replace("A", "E");
    handString = handString.replace("T", "A");
    handString = handString.replace("J", "B");
    handString = handString.replace("Q", "C");
    handString = handString.replace("K", "D");

    let counts: HashMap<char, isize> = handString.to_lowercase().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    if counts.values().any(|&v| v == 5) {
        Hand::FiveOfAKind(handString)
    } else if counts.values().any(|&v| v == 4) {
        Hand::FourOfAKind(handString)
    } else if counts.values().any(|&v| v == 3) && counts.values().any(|&v| v == 2) {
        Hand::FullHouse(handString)
    } else if counts.values().any(|&v| v == 3) {
        Hand::ThreeOfAKind(handString)
    } else if counts.values().filter(|&v| *v == 2).count() == 2 {
        Hand::TwoPair(handString)
    } else if counts.values().any(|&v| v == 2) {
        Hand::OnePair(handString)
    } else {
        Hand::HighCard(handString)
    }
}

fn hand_to_value_joker(hand: &str) -> Hand {
    let mut handString = hand.to_string();
    handString = handString.replace("A", "D");
    handString = handString.replace("J", "1");
    handString = handString.replace("T", "A");
    handString = handString.replace("Q", "B");
    handString = handString.replace("K", "C");

    let mut counts: HashMap<char, isize> = handString.to_lowercase().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let joker_count = counts.get(&'1').unwrap_or(&0).clone();

    // Remove the jokers to not count them
    counts.remove(&'1');

    if counts.values().count() == 0 && joker_count == 5 {
        Hand::FiveOfAKind(handString)
    } else if counts.values().any(|&v| v == 5) || counts.values().any(|&v| v == 5 - joker_count) {
        Hand::FiveOfAKind(handString)
    } else if counts.values().any(|&v| v == 4) || counts.values().any(|&v| v == 4 - joker_count) {
        Hand::FourOfAKind(handString)
    } else if counts.values().any(|&v| v == 3) && counts.values().any(|&v| v == 2) ||
        counts.values().any(|&v| v == 3) && joker_count == 2 ||
        counts.values().filter(|&v| *v == 2).count() == 2 && joker_count == 1   {
        Hand::FullHouse(handString)
    } else if counts.values().any(|&v| v == 3) || counts.values().any(|&v| v == 3 - joker_count){
        Hand::ThreeOfAKind(handString)
    } else if counts.values().filter(|&v| *v == 2).count() == 2
    {
        Hand::TwoPair(handString)
    } else if counts.values().any(|&v| v == 2) || counts.values().any(|&v| v == 2 - joker_count) {
        Hand::OnePair(handString)
    } else {
        Hand::HighCard(handString)
    }
}

fn value_to_order_string(value: &Hand) -> String {
    match value {
        Hand::FiveOfAKind(hand) => {
            let mut val = "7".to_string();
            val.push_str(hand);
            val
        },
        Hand::FourOfAKind(hand) => {
            let mut val = "6".to_string();
            val.push_str(hand);
            val
        },
        Hand::FullHouse(hand) => {
            let mut val = "5".to_string();
            val.push_str(hand);
            val
        },
        Hand::ThreeOfAKind(hand) => {
            let mut val = "4".to_string();
            val.push_str(hand);
            val
        },
        Hand::TwoPair(hand) => {
            let mut val = "3".to_string();
            val.push_str(hand);
            val
        },
        Hand::OnePair(hand) => {
            let mut val = "2".to_string();
            val.push_str(hand);
            val
        },
        Hand::HighCard(hand) => {
            let mut val = "1".to_string();
            val.push_str(hand);
            val
        },
        _ => panic!("Invalid hand value"),
    }
}

#[aoc_generator(day7, part1)]
fn day7_gen(input: &str) -> Vec<HandValue> {
    input.lines().map(|l| {
        let mut split = l.split(" ");
        let hand = split.next().unwrap().trim().to_string();
        let value = split.next().unwrap().trim().parse::<isize>().unwrap();
        let handValue = hand_to_value(hand.as_str());
        HandValue {
            hand: value_to_order_string(&handValue),
            value,
        }
    }).collect()
}

#[aoc(day7, part1)]
fn day7_run(input: &[HandValue]) -> isize {
   let mut res = input.to_vec();
    res.sort_by(|a, b| {
        a.hand.cmp(&b.hand)
    });
    res.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v.value * (i as isize + 1)
    })
}

#[aoc_generator(day7, part2)]
fn day7_gen2(input: &str) -> Vec<HandValue> {
    input.lines().map(|l| {
        let mut split = l.split(" ");
        let hand = split.next().unwrap().trim().to_string();
        let value = split.next().unwrap().trim().parse::<isize>().unwrap();
        let handValue = hand_to_value_joker(hand.as_str());
        HandValue {
            hand: value_to_order_string(&handValue),
            value,
        }
    }).collect()
}

#[aoc(day7, part2)]
fn day7_run2(input: &[HandValue]) -> isize {
   let mut res = input.to_vec();
    res.sort_by(|a, b| {
        a.hand.cmp(&b.hand)
    });
    println!("{:?}", res);
    res.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v.value * (i as isize + 1)
    })
}