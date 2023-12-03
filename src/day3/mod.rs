use aoc_runner_derive::{aoc, aoc_generator};
use log::debug;

#[derive(Clone, Debug)]
struct Number {
    value: isize,
    x_start: isize,
    x_end: isize,
    y: isize,
}

#[derive(Clone, Debug)]
struct Operator {
    value: char,
    x: isize,
    y: isize,
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_operator(c: char) -> bool {
    if is_number(c) {
        return false;
    }

    match c {
        '.' => false,
        _ => true,
    }
}

#[aoc_generator(day3)]
fn day3_gen(input: &str) -> (Vec<Number>, Vec<Operator>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    input.lines().enumerate().for_each(|(y, l)| {
        let mut number: String = String::new();
        l.chars().enumerate().for_each(|(x, c)| {
            if is_number(c) {
                number.push(c);
                return;
            }
            if is_operator(c) {
                operators.push(Operator {
                    value: c,
                    x: x as isize,
                    y: y as isize,
                });
                if number.len() > 0 {
                    numbers.push(Number {
                        value: number.parse::<isize>().unwrap(),
                        x_start: (x - number.len()) as isize,
                        x_end: (x - 1) as isize,
                        y: y as isize
                    });
                    number = String::new();
                }
                return;
            }
            match c {
                '.' => {
                    if number.len() > 0 {
                        numbers.push(Number {
                            value: number.parse::<isize>().unwrap(),
                            x_start: (x - number.len()) as isize,
                            x_end: (x - 1) as isize,
                            y: y as isize
                        });
                        number = String::new();
                    }
                },
                _ => {},
            }
        });
        if number.len() > 0 {
            numbers.push(Number {
                value: number.parse::<isize>().unwrap(),
                x_start: (l.len() - number.len() - 1) as isize,
                x_end: (l.len() - 1 - 1) as isize,
                y: y as isize
            });
            number = String::new();
        }
    });

    (numbers, operators)
}

#[aoc(day3, part1)]
fn day3_run(input: &(Vec<Number>, Vec<Operator>)) -> isize {
    let (numbers, operators) = input;

    println!("numbers: {:?}", numbers);
    println!("operators: {:?}", operators);

    let filtered: Vec<Number> = numbers.iter().filter(|n| {
        operators.iter().any(|o| {
            o.x >= n.x_start - 1 && o.x <= n.x_end + 1 && o.y >= n.y - 1 && o.y <= n.y + 1
        })
    }).cloned().collect();

    println!("filtered: {:?}", filtered);

    filtered.iter().map(|n| n.value).sum()
}

#[aoc(day3, part2)]
fn day3_run2(input: &(Vec<Number>, Vec<Operator>)) -> isize {
    let (numbers, operators) = input;

    println!("numbers: {:?}", numbers);
    println!("operators: {:?}", operators);

    let filtered = operators.iter().flat_map(|o| {
        if o.value != '*' {
            return Vec::<isize>::new();
        }

        let nums = numbers.iter().filter(|n| {
            o.x >= n.x_start - 1 && o.x <= n.x_end + 1 && o.y >= n.y - 1 && o.y <= n.y + 1
        }).map(|n| n.value).collect::<Vec<isize>>();

        if nums.len() != 2 {
            return Vec::<isize>::new();
        }

        return Vec::<isize>::from([nums[0] * nums[1]]);
    }).collect::<Vec<isize>>();
ewgfwng
    println!("totals: {:?}", filtered);

    filtered.iter().sum()
}