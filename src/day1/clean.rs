use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

fn words_to_number(input: &str) -> usize {
    match input {
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => 0,
    }
}

fn compute(input: &str, withWords: bool) -> Vec<usize> {
    let strReg = match withWords {
        true => String::from("1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine"),
        false => String::from("1|2|3|4|5|6|7|8|9"),
    };

    let regex = Regex::new(&strReg).unwrap();
    let reverseRegex = Regex::new(&strReg.chars().rev().collect::<String>()).unwrap();

    input.lines().map(|l| {
        let reverse = l.chars().rev().collect::<String>();
        let num1 = regex.find(l).unwrap().as_str();
        let num2 = match withWords {
            true => reverseRegex.find(&reverse).unwrap().as_str(),
            false => regex.find(&reverse).unwrap().as_str(),
        };

        words_to_number(num1) * 10 + words_to_number(num2)
    }).collect()
}

#[aoc_generator(day1, part1)]
fn clean_day1(input: &str) -> Vec<usize> {
    compute(input, false)
}

#[aoc_generator(day1, part2)]
fn clean_day2(input: &str) -> Vec<usize> {
    compute(input, true)
}

#[aoc(day1, part1)]
fn clean_day1_run(input: &[usize]) -> usize {
    input.iter().sum::<usize>()
}

#[aoc(day1, part2)]
fn clean_day2_run(input: &[usize]) -> usize {
    input.iter().sum::<usize>()
}
