use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use aoc2019::Day;
use crate::intcode::Program;

const DAY: i32 = 7;

fn all_possibilities(mut values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut all = vec![];
    if values.len() < 2 {
        all.push(vec![values[0]]);
    } else if values.len() == 2 {
       all.push(vec![values[0], values[1]]);
       all.push(vec![values[1], values[0]]);
    } else {
        let digit = values.pop().unwrap();
        let some = all_possibilities(values);
        for p in some {
            for i in 0..=p.len() {
                let mut new = p.clone();
                new.insert(i, digit);
                all.push(new);
            }
        }
    }
    all
}

fn part1(lines: &Vec<String>) {
    let phases = all_possibilities(vec![0, 1, 2, 3, 4]);
    let p: Program = lines[0].parse().unwrap();
    let amp_driver = p.dump_ram();
    let mut best = 0;
    for phase in 0..5 * 4 * 3 * 2 {
        let mut setting = phases[phase].clone();
        let mut amplifiers = vec![];
        for i in 0..=4 {
            let mut amp = Program::new(amp_driver.clone());
            amp.id = i;
            amplifiers.push(amp);
        }
        let mut last_output: i64 = 0;
        for amp in &mut amplifiers {
            amp.input(setting.pop().unwrap());
            amp.input(last_output);
            amp.execute();
            if let Some(value) = amp.output() {
                last_output = value;
            } else {
                panic!("intcode program didn't provide expected output");
            }
        }
        if last_output > best {
            best = last_output;
        }
    }

    println!("Part 1: {}", best);
}

fn part2(lines: &Vec<String>) {
    let phases = all_possibilities(vec![5, 6, 7, 8, 9]);
    let p : Program = lines[0].parse().unwrap();
    let amp_driver = p.dump_ram();
    let mut best = 0;
    for phase in 0..5 * 4 * 3 * 2 {
        let mut amplifiers = vec![];
        for i in 0..=4 {
            let mut amp = Program::new(amp_driver.clone());
            amp.id = i;
            amplifiers.push(amp);
        }
        let mut setting = phases[phase].clone();
        for amp in &mut amplifiers {
            amp.input(setting.pop().unwrap());
            amp.execute();
        }
        let mut queue: VecDeque<i64> = VecDeque::new();
        queue.push_back(0);
        let mut last_queue = Some(queue);

        while amplifiers.iter().any(|p| p.status.unfinished()) {
            for current in &mut amplifiers {
                current.input = last_queue.take();
                current.output = Some(VecDeque::new());
                current.execute();
                last_queue = current.output.take();
            }
        }
        if let Some(mut q) = last_queue {
            if let Some(out) = q.pop_front() {
                if out > best {
                    best = out;
                }
            }
        }
    }
    println!("Part 2: {:?}", best);
}

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

#[test]
pub fn tests() {}
