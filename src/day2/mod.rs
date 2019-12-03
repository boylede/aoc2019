use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

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
    let mut program : Program = lines[0].parse().unwrap();
    program.load_input(12, 2);
    program.execute();
    let value = program.output();
    println!("Part 1: {}", value);
}

fn part2(lines: &Vec<String>) {
    let program : Program = lines[0].parse().unwrap();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut attempt = program.clone();
            attempt.load_input(noun, verb);
            attempt.execute();
            let value = attempt.output();
            if value == 19690720 {
                println!("Part 2: {}", noun * 100 + verb);
                return;
            }
        }
    }
}
#[derive(Clone)]
struct Program {
    counter: usize,
    cycles: usize,
    status: RunStatus,
    memory: Vec<i32>,
}

#[derive(Clone, PartialEq)]
enum RunStatus {
    Running,
    Finished,
    Killed,
}

impl RunStatus {
    fn running(&self) -> bool {
        *self == RunStatus::Running
    }
}

impl Program {
    fn load_input(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }
    fn new(memory: Vec<i32>) -> Program {
        Program {
            counter: 0,
            cycles: 0,
            status: RunStatus::Running,
            memory,
        }
    }
    fn step(&mut self, steps: usize) {
        let mut ticks = 0;
        while steps > ticks {
            match self.memory[self.counter] {
                1 => {
                    let a = self.get_indirect(1);
                    let b = self.get_indirect(2);
                    self.set_indirect(3, a + b);
                    self.counter = self.counter + 4;
                },
                2 => {
                    let a = self.get_indirect(1);
                    let b = self.get_indirect(2);
                    self.set_indirect(3, a * b);
                    self.counter = self.counter + 4;
                },
                99 => {
                    self.status = RunStatus::Finished;
                    break;
                },
                _ => {
                    self.status = RunStatus::Killed;
                }
            }
            ticks = ticks + 1;
        }
        self.cycles = self.cycles + ticks;
    }
    fn get_offset(&self, offset: usize) -> i32 {
        self.memory[self.counter + offset]
    }
    fn get_indirect(&self, offset: usize) -> i32 {
        self.memory[self.get_offset(offset) as usize]
    }
    fn set_indirect(&mut self, offset: usize, value: i32) {
        let index = self.memory[self.counter + offset];
        self.set(index as usize, value);
    }
    fn set(&mut self, index: usize, value: i32) {
        self.memory[index] = value; 
    }
    fn execute(&mut self) {
        while self.status.running() {
            self.step(100);
        }
    }
    fn output(&self) -> i32 {
        self.memory[0]
    }
}


impl FromStr for Program {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Program, Self::Err> {
        Ok(
            Program::new(
                input
                .split_terminator(',')
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<i32>, Self::Err>>()?
            )
        )
    }
}




#[test]
pub fn tests() {
  pub fn run(test: &str) -> Vec<i32> {
    let mut program : Program = test.parse().unwrap();
    program.execute();
    program.memory
  }

assert_eq!(run("1,0,0,0,99"), vec![2,0,0,0,99]);
assert_eq!(run("2,3,0,3,99"), vec![2,3,0,6,99]);
assert_eq!(run("2,4,4,5,99,0"), vec![2,4,4,5,99,9801]);
assert_eq!(run("1,1,1,4,99,5,6,0,99"), vec![30,1,1,4,2,5,6,0,99]);

}