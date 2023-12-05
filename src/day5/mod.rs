use std::cmp::min;
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Range;
use aoc_runner_derive::{aoc, aoc_generator};

// Works but wrong way of thinking
#[aoc_generator(day5, part1)]
fn day5_gen(input: &str) -> Vec<Range<isize>> {
    let mut seeds = Vec::<Range<isize>>::new();
    let mut seeds_next: Vec<Range<isize>> = Vec::new();
    input.lines().filter(|l| *l != "").for_each(|l| {
        if l.len() == 0 {
            return;
        }
        let mut split: Vec<&str> = l.split(":").collect();
        if split.len() == 2 {
            if let Some(name) = split.get(0) {
                match *name {
                    "seeds" => {
                        split.get(1).unwrap().split(" ").filter(|s| s.trim() != "").map(|n| n.trim().parse::<isize>().unwrap()).for_each(|n| {
                            seeds.push(n..n);
                            seeds_next.push(n..n);
                        });
                    },
                    _ => {
                        seeds = seeds_next.clone();
                    },
                }
                return;
            }
        }

        let mut num_split = l.split(" ");

        let dest = num_split.nth(0).unwrap().parse::<isize>().unwrap();
        let src = num_split.nth(0).unwrap().parse::<isize>().unwrap();
        let range = num_split.nth(0).unwrap().parse::<isize>().unwrap();

        let src_range = src..src + range;

        for i in 0..seeds.len() {
            if src_range.contains(&seeds[i].start) && src_range.contains(&seeds[i].end) {
                seeds_next[i] = (seeds[i].start + dest - src)..(seeds[i].end + dest - src);
            }
        }
    });
    seeds_next
}

#[aoc(day5, part1)]
fn day5_run(input: &[Range<isize>]) -> isize {
    println!("{:?}", input);
    input.iter().map(|r| r.start).min().unwrap()
}

#[aoc_generator(day5, part2)]
fn day5_gen2(input: &str) -> Vec<Range<isize>> {
    let mut seeds = Vec::<Range<isize>>::new();
    let mut seeds_next: Vec<Range<isize>> = Vec::new();

    input.lines().filter(|l| *l != "").for_each(|l| {
        if l.len() == 0 {
            return;
        }
        let mut split: Vec<&str> = l.split(":").collect();
        if split.len() == 2 {
            if let Some(name) = split.get(0) {
                match *name {
                    "seeds" => {
                        let mut mem: isize = 0;
                        split.get(1).unwrap().split(" ").filter(|s| s.trim() != "").map(|n| n.trim().parse::<isize>().unwrap()).enumerate().for_each(|(i, n)| {
                            if i % 2 == 0 {
                                mem = n;
                            } else {
                                seeds.push(mem..mem+n);
                                seeds_next.push(mem..mem+n);
                            }
                        });
                    },
                    _ => {
                        seeds_next.iter().for_each(|s| {
                            seeds.push(s.clone());
                        });
                        seeds_next.clear();
                    },
                }
                return;
            }
        }

        let mut num_split = l.split(" ");

        let dest = num_split.nth(0).unwrap().parse::<isize>().unwrap();
        let src = num_split.nth(0).unwrap().parse::<isize>().unwrap();
        let range = num_split.nth(0).unwrap().parse::<isize>().unwrap();

        let src_range = src..src + range;

        let seeds_to_compute = seeds.clone();
        seeds.clear();

        for i in 0..seeds_to_compute.len() {
            if src_range == seeds_to_compute[i] {
                seeds_next.push((seeds_to_compute[i].start + dest - src)..(seeds_to_compute[i].end + dest - src));
                continue;
            }

            if src_range.contains(&seeds_to_compute[i].start) && src_range.contains(&seeds_to_compute[i].end) {
                seeds_next.push((seeds_to_compute[i].start + dest - src)..(seeds_to_compute[i].end + dest - src));
                continue;
            }

            if src_range.contains(&seeds_to_compute[i].start) {
                seeds_next.push((seeds_to_compute[i].start + dest - src)..(src_range.end + dest - src));
                seeds_next.push(src_range.end..seeds_to_compute[i].end);
                continue;
            }

            if src_range.contains(&seeds_to_compute[i].end) {
                seeds_next.push(seeds_to_compute[i].start..src_range.start);
                seeds_next.push((src_range.start + dest - src)..(seeds_to_compute[i].end + dest - src));
                continue;
            }

            seeds.push(seeds_to_compute[i].clone());
        }
        println!("range: {:?}" , src_range);
        println!("seeds : {:?}", seeds);
        println!("seeds to compute: {:?}", seeds_to_compute);
        println!("seeds next: {:?}", seeds_next);
    });
    seeds_next
}

#[aoc(day5, part2)]
fn day5_run2(input: &[Range<isize>]) -> isize {
    input.iter().map(|r| r.start).min().unwrap()
}