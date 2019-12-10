use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2019::Day;
use crate::intcode::Program;

const DAY: i32 = 9;

fn part1(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.input(1);
    program.execute();
    let value = program.output.unwrap();
    println!("Part 1: {:?}", value);
}

fn part2(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.input(2);
    program.execute();
    let value = program.output.unwrap();
    println!("Part 2: {:?}", value);
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
pub fn tests() {
    use crate::intcode::RunStatus;
    pub fn run(program: &str, input: i64) -> Vec<i64> {
        let mut program: Program = program.parse().unwrap();
        program.input(input);
        program.execute();
        if program.status != RunStatus::Finished {
            panic!("virtual machine terminated early");
        }
        program.output.unwrap().iter().map(|e|*e).collect::<Vec<i64>>()
    }
    assert_eq!(run("104,1125899906842624,99", 0), vec![1125899906842624]);
    assert_eq!(run("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", 0), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    assert_eq!(run("1102,34915192,34915192,7,4,7,99,0", 0), vec![1219070632396864]);
}
