use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


use aoc2019::Day;
use crate::intcode::Program;

const DAY: i32 = 2;

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

fn part1(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.set(1, 12);
    program.set(2, 2);
    program.execute();
    let value = program.get(0);
    println!("Part 1: {}", value);
}

fn part2(lines: &Vec<String>) {
    let program: Program = lines[0].parse().unwrap();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut attempt = program.clone();
            attempt.set(1, noun);
            attempt.set(2, verb);
            attempt.execute();
            let value = attempt.get(0);
            if value == 19690720 {
                println!("Part 2: {}", noun * 100 + verb);
                return;
            }
        }
    }
}

#[test]
pub fn tests() {
    pub fn run(test: &str) -> Vec<i64> {
        let mut program: Program = test.parse().unwrap();
        program.execute();
        program.dump_ram()
    }

    assert_eq!(run("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
    assert_eq!(run("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
    assert_eq!(run("2,4,4,5,99,0"), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run("1,1,1,4,99,5,6,0,99"),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
