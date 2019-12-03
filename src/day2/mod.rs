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
    let program : Vec<i32> = lines[0].split_terminator(',').map(|num| num.parse::<i32>().unwrap()).collect();
    let value = evaluate(&program, 12, 2);
    println!("Part 1: {}", value);
}

fn part2(lines: &Vec<String>) {
    let program : Vec<i32> = lines[0].split_terminator(',').map(|num| num.parse::<i32>().unwrap()).collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let value = evaluate(&program, noun, verb);
            if value == 19690720 {
                println!("Part 2: {}", noun * 100 + verb);
                return;
            }
        }
    }
}

fn evaluate(initial_program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut program : Vec<i32> = initial_program.clone();
    program[1] = noun;
    program[2] = verb;
    let final_state = run_program(program);
    final_state[0]
}

fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let mut counter = 0;
    loop {
        match program[counter] {
            1 => {
                let a_index = program[counter + 1];
                let b_index = program[counter + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];
    
                let output_index = program[counter + 3];
                let output = a + b;
                program[output_index as usize] = output;
                counter = counter + 4;
            },
            2 => {
                let a_index = program[counter + 1];
                let b_index = program[counter + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];
    
                let output_index = program[counter + 3];
                let output = a * b;
                program[output_index as usize] = output;
                counter = counter + 4;
            },
            99 => {
                // println!("Program halted at 99");
                break;
            },
            _ => {
                panic!("Program halted at unexpected input");
            }
        }
    }
    program
}


#[test]
pub fn tests() {
  pub fn run(test: &str) -> Vec<i32> {
    let mut program = test.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    run_program(program)
  }

assert_eq!(run("1,0,0,0,99"), vec![2,0,0,0,99]);
assert_eq!(run("2,3,0,3,99"), vec![2,3,0,6,99]);
assert_eq!(run("2,4,4,5,99,0"), vec![2,4,4,5,99,9801]);
assert_eq!(run("1,1,1,4,99,5,6,0,99"), vec![30,1,1,4,2,5,6,0,99]);

}