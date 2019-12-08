use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use aoc2019::Day;

const DAY: i32 = 7;

fn all_possibilities(mut values: Vec<i32>) -> Vec<Vec<i32>> {
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
    let amp_driver = p.memory;
    let mut best = 0;
    for phase in 0..5 * 4 * 3 * 2 {
        let mut setting = phases[phase].clone();
        let mut amplifiers = vec![];
        for i in 0..=4 {
            let mut amp = Program::new(amp_driver.clone());
            amp.id = i;
            amplifiers.push(amp);
        }
        let mut last_output: i32 = 0;
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
    let Program {
        memory: amp_driver, ..
    } = lines[0].parse().unwrap();
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
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(0);
        let mut last_queue = Some(queue);

        while amplifiers.iter().any(running) {
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

fn running(p: &Program) -> bool {
    p.status.unfinished()
}

#[derive(Clone)]
struct Program {
    id: usize,
    counter: usize,
    cycles: usize,
    status: RunStatus,
    memory: Vec<i32>,
    input: Option<VecDeque<i32>>,
    output: Option<VecDeque<i32>>,
}

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
    fn unfinished(&self) -> bool {
        self.running() || self.blocked()
    }
}

fn opcode(instruction: i32) -> i32 {
    instruction % 100
}

fn is_immediate(instruction: i32, paramater: usize) -> bool {
    (instruction as u32 / (10u32.pow((paramater as u32) + 1))) % 10 == 1
}

impl Program {
    fn new(memory: Vec<i32>) -> Program {
        Program {
            id: 0,
            counter: 0,
            cycles: 0,
            status: RunStatus::Running,
            memory,
            input: Some(VecDeque::new()),
            output: Some(VecDeque::new()),
        }
    }

    fn input(&mut self, value: i32) {
        if let Some(input) = &mut self.input {
            input.push_back(value);
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }
    fn output(&mut self) -> Option<i32> {
        if let Some(output) = &mut self.output {
            output.pop_front()
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }

    fn step(&mut self, steps: usize) {
        let mut ticks = 0;
        // let input = self.input.as_ref().expect("tried to use program while input buffer was moved");
        // let output = self.output.as_ref().expect("tried to use program while output buffer was moved");

        while steps > ticks {
            // print!("{}: ", self.counter);
            let instruction = self.memory[self.counter];
            match opcode(instruction) {
                1 => {
                    // print!("add ");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    // print!(" -> ");
                    self.set_indirect(3, a + b);
                    self.counter = self.counter + 4;
                }
                2 => {
                    // print!("mul ");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    // print!(" -> ");
                    self.set_indirect(3, a * b);
                    self.counter = self.counter + 4;
                }
                3 => {
                    // print!("pop ");
                    let input = &self
                        .input
                        .as_mut()
                        .expect("tried to use input while backing buffer was removed")
                        .pop_front();
                    if let Some(value) = input {
                        self.set_indirect(1, *value);
                        self.counter = self.counter + 2;
                    } else {
                        self.status = RunStatus::Blocked;
                        break;
                    }
                }
                4 => {
                    // print!("push ");
                    let value = self.get_parameter(1);

                    self.output
                        .as_mut()
                        .expect("tried to use output while backing buffer was removed")
                        .push_back(value);
                    self.counter = self.counter + 2;
                }
                5 => {
                    // print!("if ");
                    let condition = self.get_parameter(1);
                    // print!("isn't 0, jump ");
                    let jump = self.get_parameter(2);
                    // println!("");

                    if condition != 0 {
                        self.counter = jump as usize;
                    } else {
                        self.counter = self.counter + 3;
                    }
                }
                6 => {
                    // print!("if ");
                    let condition = self.get_parameter(1);
                    // print!("is 0, jump ");
                    let jump = self.get_parameter(2);
                    // println!("");

                    if condition == 0 {
                        self.counter = jump as usize;
                    } else {
                        self.counter = self.counter + 3;
                    }
                }
                7 => {
                    // print!("less than");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);

                    if a < b {
                        self.set_indirect(3, 1);
                    } else {
                        self.set_indirect(3, 0);
                    }
                    self.counter = self.counter + 4;
                }
                8 => {
                    // print!("greater than");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);

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
                    println!("killed program: found instruction {}", instruction);
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
            let value = self.get_offset(offset);
            // print!("{}i@{}, ", value, self.counter + offset);
            value
        } else {
            let value = self.get_indirect(offset);

            value
        }
    }
    fn get_offset(&self, offset: usize) -> i32 {
        self.memory[self.counter + offset]
    }
    fn get_indirect(&self, offset: usize) -> i32 {
        let address = self.get_offset(offset) as usize;
        // print!("[{}]={}, ", address, self.memory[address]);
        self.memory[address]
    }
    fn set_indirect(&mut self, offset: usize, value: i32) {
        let index = self.memory[self.counter + offset];
        // println!("{} @{}, ", value, index);
        self.set(index as usize, value);
    }
    fn set(&mut self, index: usize, value: i32) {
        self.memory[index] = value;
    }
    fn execute(&mut self) {
        if self.status.blocked() {
            self.status = RunStatus::Running;
        }
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
pub fn tests() {}
