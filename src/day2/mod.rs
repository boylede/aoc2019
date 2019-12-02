use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2019::Day;

const DAY: i32 = 2;

pub fn load(days_array: &mut Vec<Day>) {
    days_array.push(Day::new(DAY, run));
}

pub fn run(input: File) {
    println!("loading day {} input.", DAY);
    let a_time = time::precise_time_ns();
    
    let mut lines = vec!();
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
    let mut program : Vec<i32> = lines[0].split_terminator(',').map(|num| num.parse::<i32>().unwrap()).collect();
    println!("program {:?}", program);

    program[1] = 12;
    program[2] = 2;

    let mut PC = 0;
    while program[PC] != 99 {
        match program[PC] {
            1 => {
                let a_index = program[PC + 1];
                let b_index = program[PC + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];
    
                let output_index = program[PC + 3];
                let output = a + b;
                program[output_index as usize] = output;
                println!("{} + {} = {}, put in {}", a, b, output, output_index);
                PC = PC + 4;
            },
            2 => {
                let a_index = program[PC + 1];
                let b_index = program[PC + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];
    
                let output_index = program[PC + 3];
                let output = a * b;
                program[output_index as usize] = output;
                println!("{} * {} = {}, put in {}", a, b, output, output_index);
                PC = PC + 4;
            },
            99 => {
                println!("Program halted at 99");
            },
            _ => {
                panic!("Program halted at unexpected input");
            }
        }
    }
    println!("program result: {}", program[0]);
    
}

fn part2(lines: &Vec<String>) {

}
