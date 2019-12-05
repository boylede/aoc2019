use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
struct Digits {
    number: u32,
    place: u32,
}

impl Digits {
    fn new(number: u32) -> Digits {
        Digits {
            number,
            place: 100_000,
        }
    }
}

impl Iterator for Digits {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.place == 0 {
            return None;
        }
        let mut digit = self.number / self.place;
        self.place = self.place / 10;
        digit = digit % 10;
        Some(digit as u8)
    }
}

#[inline]
fn adjacency(password: &Digits) -> bool {
    password.zip(password.skip(1)).any(|(a, b)| a == b)
}

#[inline]
fn single_adjacency(password: &Digits) -> bool {
    let mut last_digit = 0;
    let mut runs: Vec<u8> = vec![];
    let mut run_length = 0;
    for digit in *password {
        if digit == last_digit {
            run_length = run_length + 1;
        } else {
            runs.push(run_length);
            run_length = 0;
        }
        last_digit = digit;
    }
    runs.push(run_length);
    runs.iter().any(|l| *l == 1)
}

#[inline]
fn increasing(password: &Digits) -> bool {
    password.zip(password.skip(1)).all(|(a, b)| a <= b)
}

fn part1(lines: &Vec<String>) {
    let range: Vec<u32> = lines[0]
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let lower = range[0];
    let upper = range[1];
    let potential_matches = (lower..=upper)
        .map(|n| Digits::new(n))
        .filter(increasing)
        .filter(adjacency)
        .count();
    println!("Part 1: {}", potential_matches);
}

fn part2(lines: &Vec<String>) {
    let range: Vec<u32> = lines[0]
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let lower = range[0];
    let upper = range[1];
    let potential_matches = (lower..=upper)
    .map(|n| Digits::new(n))
    .filter(increasing)
    .filter(single_adjacency)
    .count();

    println!("Part 2: {}", potential_matches);
}

use aoc2019::Day;

const DAY: i32 = 4;

pub fn load(days_array: &mut Vec<Day>) {
    days_array.push(Day::new(DAY, run));
}

pub fn run(input: File) {
    println!("loading day {} input.", DAY);
    let a_time = time::precise_time_ns();

    let mut lines = vec![];
    {
        let mut lines_iterator = BufReader::new(&input).lines();
        while let Some(Ok(line)) = lines_iterator.next() {
            lines.push(line);
        }
    }
    let b_time = time::precise_time_ns();
    let total_time = b_time - a_time;
    if total_time > 100000 {
        println!("Loading took: {}ms", total_time as f32 / 1000000.0);
    } else {
        println!("Loading took: {}ns", total_time);
    }

    post_load(lines);
}

fn post_load(lines: Vec<String>) {
    let a_time = time::precise_time_ns();
    part1(&lines);
    let b_time = time::precise_time_ns();
    part2(&lines);
    let c_time = time::precise_time_ns();
    println!("Day {} Part 1 took: {}ns", DAY, b_time - a_time);
    println!("Day {} Part 2 took: {}ns", DAY, c_time - b_time);
}
