use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use aoc2019::Day;

const DAY: i32 = 5;

fn part1(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.input(1);
    program.execute();
    let value = program.output;
    println!("Part 1: {:?}", value);
}

fn part2(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.input(5);
    program.execute();
    let value = program.output;
    println!("Part 2: {:?}", value);
}


#[derive(Clone)]
struct Program {
    counter: usize,
    cycles: usize,
    status: RunStatus,
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
}

#[derive(Clone, PartialEq)]
#[derive(Clone, PartialEq, Debug)]
enum RunStatus {
    Running,
    Finished,
    Killed,
    Blocked,
}

impl RunStatus {
    fn running(&self) -> bool {
        *self == RunStatus::Running
    }
    fn blocked(&self) -> bool {
        *self == RunStatus::Blocked
    }
}

fn opcode(instruction: i32) -> i32 {
    instruction % 100
}

fn is_immediate(instruction: i32, paramater: usize) -> bool {
    (instruction as u32 / (10u32.pow((paramater as u32) + 1))) % 10 == 1
}

impl Program {
    fn inject_input(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }
    fn new(memory: Vec<i32>) -> Program {
        Program {
            counter: 0,
            cycles: 0,
            status: RunStatus::Running,
            memory,
            input: vec![],
            output: vec![],
        }
    }
    fn input(&mut self, value: i32) {
        self.input.push(value);
    }

    fn step(&mut self, steps: usize) {
        let mut ticks = 0;
        while steps > ticks {
            let instruction = self.memory[self.counter];
            match opcode(instruction) {
                1 => {
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    self.set_indirect(3, a + b);
                    self.counter = self.counter + 4;
                }
                2 => {
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    self.set_indirect(3, a * b);
                    self.counter = self.counter + 4;
                }
                3 => {
                    let input = self.input.pop();
                    if let Some(value) = input {
                        self.set_indirect(1, value);
                        self.counter = self.counter + 2;
                    } else {
                        self.status = RunStatus::Blocked;
                        break;
                    }
                }
                4 => {
                    let output = self.get_parameter(1);
                    self.output.push(output);
                    self.counter = self.counter + 2;
                }
                5 => {
                    let condition = self.get_parameter(1);
                    let jump = self.get_parameter(2);
                    if condition != 0 {
                        self.counter = jump as usize;
                    } else {
                        self.counter = self.counter + 3;
                    }
                }
                6 => {
                    let condition = self.get_parameter(1);
                    let jump = self.get_parameter(2);
                    if condition == 0 {
                        self.counter = jump as usize;
                    } else {
                        self.counter = self.counter + 3;
                    }
                }
                7 => {
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    let output = self.get_parameter(3);
                    if a < b {
                        self.set_indirect(3, 1);
                    } else {
                        self.set_indirect(3, 0);
                    }
                    self.counter = self.counter + 4;
                }
                8 => {
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    let output = self.get_parameter(3);
                    if a == b {
                        self.set_indirect(3, 1);
                    } else {
                        self.set_indirect(3, 0);
                    }
                    self.counter = self.counter + 4;
                }
                99 => {
                    self.status = RunStatus::Finished;
                    break;
                }
                _ => {
                    self.status = RunStatus::Killed;
                    break;
                }
            }
            ticks = ticks + 1;
        }
        self.cycles = self.cycles + ticks;
    }
    fn get_parameter(&self, offset: usize) -> i32 {
        if is_immediate(self.memory[self.counter], offset) {
            self.get_offset(offset)
        } else {
            self.get_indirect(offset)
        }
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
    fn r#continue(&mut self) {
        if self.status.blocked() && self.input.len() > 0 {
            self.status = RunStatus::Running;
        }
        self.execute();
    }
    fn execute(&mut self) {
        while self.status.running() {
            self.step(100);
        }
    }
    fn extract_output(&self) -> i32 {
        self.memory[0]
    }
}

impl FromStr for Program {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Program, Self::Err> {
        Ok(Program::new(
            input
                .split_terminator(',')
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<i32>, Self::Err>>()?,
        ))
    }
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
    pub fn run(test: &str) -> Vec<i32> {
        let mut program: Program = test.parse().unwrap();
        program.execute();
        if program.status != RunStatus::Finished {
            panic!("virtual machine terminated early");
        }
        program.memory
    }
    pub fn io_test(program: &str, input: i32) -> i32 {
        let mut program: Program = program.parse().unwrap();
        program.input(input);
        program.execute();
        if program.status != RunStatus::Finished {
            panic!("virtual machine terminated early");
        }
        program.output[0]
    }

    assert_eq!(run("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
    assert_eq!(run("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
    assert_eq!(run("2,4,4,5,99,0"), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run("1,1,1,4,99,5,6,0,99"),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    assert_eq!(io_test("3,0,4,0,99", -9854), -9854);
    assert_eq!(io_test("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
    assert_eq!(io_test("3,9,7,9,10,9,4,9,99,-1,8", -6), 1);
    assert_eq!(io_test("3,3,1108,-1,8,3,4,3,99", 8), 1);
    assert_eq!(io_test("3,3,1107,-1,8,3,4,3,99", -8), 1);

    assert_eq!(io_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -9), 1);
    assert_eq!(io_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -9), 1);

    // todo: tests fail if there are linebreaks in input;
    let eight_tester = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    assert_eq!(io_test(eight_tester, -9), 999);
    assert_eq!(io_test(eight_tester, 8), 1000);
    assert_eq!(io_test(eight_tester, 9000), 1001);
}
