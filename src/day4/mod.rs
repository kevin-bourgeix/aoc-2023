use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn day4_gen(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    input.lines().map(|l| {
        let numLists: Vec<&str> = l.split(":").nth(1).unwrap().split("|").collect();
        (
            numLists[0].trim().split(" ").flat_map(|n| {
                let parsed = n.parse::<usize>();
                if let Ok(res) = parsed {
                    return Vec::<usize>::from([res]);
                }
                return Vec::<usize>::new();
            }).collect(),
            numLists[1].trim().split(" ").flat_map(|n| {
                let parsed = n.parse::<usize>();
                if let Ok(res) = parsed {
                    return Vec::<usize>::from([res]);
                }
                return Vec::<usize>::new();
            }).collect(),
        )
    }).collect()
}

#[aoc(day4, part1)]
fn day4_run1(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
    input.iter().map(|(l1, l2)| {
        let filtered: Vec<usize> = l2.iter().filter(|n| l1.contains(n)).cloned().collect();
        usize::pow(2, filtered.len() as u32 - 1)
    }).sum()
}

#[aoc(day4, part2)]
fn day4_run2(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let mut vec: Vec<usize> = vec![1; input.len()];
    input.iter().enumerate().for_each(|(i, (l1, l2))| {
        let filtered: Vec<usize> = l2.iter().filter(|n| l1.contains(n)).cloned().collect();
        for j in 1..filtered.len()+1 {
            vec[i + j] += vec[i];
        }
    });
    println!("{:?}", vec);
    vec.iter().sum()
}