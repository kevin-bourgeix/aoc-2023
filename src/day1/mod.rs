use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day1, part1)]
fn day1_gen(input: &str) -> Vec<usize> {
    input.lines().map(|l| {
        let num = l.chars().fold(String::new(), |acc, c| {
            if c.is_numeric() {
                return acc + &c.to_string();
            }
            acc
        });
        if num.len() == 1 {
            let single = num.to_string().parse::<usize>().unwrap();
            return single * 10 + single;
        } else {
            return num.chars().nth(0).unwrap().to_string().parse::<usize>().unwrap() * 10
                + num.chars().nth(num.len() - 1).unwrap().to_string().parse::<usize>().unwrap()
        }
    }).collect()
}

#[aoc(day1, part1)]
fn day1_res(input: &[usize]) -> String {
    // input.iter().for_each(|n| println!("{}", n));
    input.iter().sum::<usize>().to_string()
}

fn words(input: &str) -> Vec<&str> {
    Vec::from(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "eighthree", "sevenine", "twone", "fiveight", "nineight", "oneight", "threeight", "eightwo"])
}

fn words_to_numbers(input: &str) -> usize {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "eighthree" => 83, // lol thx Reddit
        "sevenine" => 79,
        "twone" => 21,
        "fiveight" => 58,
        "nineight" => 98,
        "oneight" => 18,
        "threeight" => 38,
        "eightwo" => 82,
        _ => 0,
    }
}

#[aoc_generator(day1, part2)]
fn day1_gen2(input: &str) -> Vec<usize> {
    let regex = Regex::new("(eightwo|threeight|oneight|fiveight|nineight|eighthree|sevenine|twone|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut modified = String::from(input);
    while regex.is_match(modified.as_ref()) {
        modified = regex.replace(modified.as_ref(), |caps: &regex::Captures| {
            let word = caps.get(0).unwrap().as_str();
            let num = words_to_numbers(word);
            num.to_string()
        }).to_string();
    }
    // let modified = regex.replace(input, |caps: &regex::Captures| {
    //     let word = caps.get(0).unwrap().as_str();
    //     let num = words_to_numbers(word);
    //     num.to_string()
    // });

    modified.lines().for_each(|l| println!("{}", l));
    modified.lines().map(|l| {
        let num = l.chars().fold(String::new(), |acc, c| {
            if c.is_numeric() {
                return acc + &c.to_string();
            }
            acc
        });
        if num.len() == 1 {
            let single = num.to_string().parse::<usize>().unwrap();
            return single * 10 + single;
        } else {
            return num.chars().nth(0).unwrap().to_string().parse::<usize>().unwrap() * 10
                + num.chars().nth(num.len() - 1).unwrap().to_string().parse::<usize>().unwrap()
        }
    }).collect()
}

#[aoc(day1, part2)]
fn day1_res2(input: &[usize]) -> String {
    // input.iter().for_each(|n| println!("{}", n));
    input.iter().sum::<usize>().to_string()
}