use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1(lines: &Vec<String>) {
    let range: Vec<u32> = lines[0]
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let lower = range[0];
    let upper = range[1];
    let mut potential_matches = vec![];
    for attempt in lower..=upper {
        if increasing(attempt) && adjacency(attempt) {
            potential_matches.push(attempt);
        }
    }
    println!("Part 1: {}", potential_matches.len());
}

fn part2(lines: &Vec<String>) {
    let range: Vec<u32> = lines[0]
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let lower = range[0];
    let upper = range[1];
    let mut potential_matches = vec![];
    for attempt in lower..=upper {
        if increasing(attempt) && single_adjacency(attempt) {
            potential_matches.push(attempt);
        }
    }
    println!("Part 2: {}", potential_matches.len());
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

fn adjacency(password: u32) -> bool {
    let st: String = password.to_string();
    let mut last_digit = 0;
    for ch in st.chars() {
        let digit = ch.to_string().parse::<u8>().unwrap();
        if digit == last_digit {
            return true;
        }
        last_digit = digit;
    }
    false
}

fn single_adjacency(password: u32) -> bool {
    let st: String = password.to_string();
    let mut last_digit = 0;
    let mut runs: Vec<u8> = vec![];
    let mut run_length = 0;
    for ch in st.chars() {
        let digit = ch.to_string().parse::<u8>().unwrap();
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

fn increasing(password: u32) -> bool {
    let st: String = password.to_string();
    let mut last_digit = 0;
    for ch in st.chars() {
        let digit = ch.to_string().parse::<u8>().unwrap();
        if digit > last_digit {
            last_digit = digit
        } else if digit < last_digit {
            return false;
        }
    }
    return true;
}
