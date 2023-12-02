use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

fn compute(input: &str) -> Vec<usize> {
    let game_regex = "Game ([0-9]*): ";

    input.lines().map(|l| {
        let regex = Regex::new(game_regex).unwrap();
        let game_id = regex.captures(l).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        let game = regex.replace(l, "").to_string();
        let rounds = game.split(";").collect::<Vec<&str>>();
        let ok: bool = rounds.iter().map(|r| {
            let subset = r.trim().split(",").collect::<Vec<&str>>();
            subset.iter().map(|s| {
                let ss_splited = s.trim().split(" ").collect::<Vec<&str>>();
                let number = ss_splited[0].parse::<usize>().unwrap();
                let color = ss_splited[1];

                match color {
                    "red" => number <= 12,
                    "green" => number <= 13,
                    "blue" => number <= 14,
                    _ => false,
                }
            }).all(|b| b)
        }).all(|b| b);

        match ok {
            true => game_id,
            _ => 0,
        }
    }).collect()
}

struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn new() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn compute_pow(input: &str) -> Vec<usize> {
    let game_regex = "Game ([0-9]*): ";

    input.lines().map(|l| {
        let regex = Regex::new(game_regex).unwrap();
        let game_id = regex.captures(l).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
        let game = regex.replace(l, "").to_string();
        let rounds = game.split(";").collect::<Vec<&str>>();
        let mut subsetType = Set::new();
        rounds.iter().for_each(|r| {
            let subset = r.trim().split(",").collect::<Vec<&str>>();
            subset.iter().for_each(|s| {
                let ss_splited = s.trim().split(" ").collect::<Vec<&str>>();
                let number = ss_splited[0].parse::<usize>().unwrap();
                let color = ss_splited[1];

                if color == "red" && number > subsetType.red {
                    subsetType.red = number;
                } else if color == "green" && number > subsetType.green {
                    subsetType.green = number;
                } else if color == "blue" && number > subsetType.blue {
                    subsetType.blue = number;
                }
            });
        });
        subsetType
    }).map(|s| s.red * s.green * s.blue).collect()
}

#[aoc_generator(day2, part1)]
fn clean_day2_gen(input: &str) -> Vec<usize> {
    compute(input)
}

#[aoc(day2, part1)]
fn clean_day2_run(input: &[usize]) -> usize {
    input.iter().sum::<usize>()
}

#[aoc_generator(day2, part2)]
fn clean_day2_gen2(input: &str) -> Vec<usize> {
    compute_pow(input)
}

#[aoc(day2, part2)]
fn clean_day2_run2(input: &[usize]) -> usize {
    input.iter().sum::<usize>()
}