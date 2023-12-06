use std::ptr::replace;
use aoc_runner_derive::{aoc, aoc_generator};

struct Track {
    time: isize,
    distance: isize,
}

#[aoc_generator(day6, part1)]
fn day6_gen(input: &str) -> Vec<Track> {
    let mut time: Vec<isize> = vec![];
    let mut distance: Vec<isize> = vec![];
    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.split_whitespace().for_each(|s| {
                if s.contains("Time") || s.contains("Distance") {
                    return;
                }
                let num = s.parse::<isize>().unwrap();
                if i % 2 == 0 {
                    time.push(num);
                } else {
                    distance.push(num);
                }
            });
        });
    time.iter().zip(distance.iter()).map(|(&t, &d)| Track { time: t, distance: d }).collect()
}

#[aoc(day6, part1)]
fn day6_part1(input: &[Track]) -> isize {
    input.iter().map(|t| {
        let mut record: Vec<isize> = vec![];
        for i in 0..=t.time {
            record.push((t.time - i) * i);
        }
        record.iter().filter(|r| **r > t.distance).count() as isize
    }).fold(1, |acc, x| acc * x)
}

#[aoc_generator(day6, part2)]
fn day6_gen2(input: &str) -> Vec<Track> {
    let mut time: Vec<isize> = vec![];
    let mut distance: Vec<isize> = vec![];
    input.lines().enumerate()
        .for_each(|(i, line)| {
            let replaced = line.replace(" ", "").clone();
            let num = replaced.split(":").nth(1).unwrap();
            if i % 2 == 0 {
                time.push(num.parse::<isize>().unwrap());
            } else {
                distance.push(num.parse::<isize>().unwrap());
            }
        });
    time.iter().zip(distance.iter()).map(|(&t, &d)| Track { time: t, distance: d }).collect()
}

#[aoc(day6, part2)]
fn day6_part2(input: &[Track]) -> isize {
    input.iter().map(|t| {
        let mut record: Vec<isize> = vec![];
        for i in 0..=t.time {
            record.push((t.time - i) * i);
        }
        record.iter().filter(|r| **r > t.distance).count() as isize
    }).fold(1, |acc, x| acc * x)
}