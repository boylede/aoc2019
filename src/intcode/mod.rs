use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn running(p: &Program) -> bool {
    p.status.unfinished()
}

#[derive(Clone)]
struct Program {
    pub id: usize,
    counter: usize,
    base: i64,
    cycles: usize,
    status: RunStatus,
    memory: Vec<i64>,
    input: Option<VecDeque<i64>>,
    output: Option<VecDeque<i64>>,
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

fn opcode(instruction: i64) -> i64 {
    instruction % 100
}

fn get_mode(instruction: i64, paramater: usize) -> i64 {
    ((instruction / (10i64.pow((paramater as u32) + 1))) % 10) as i64
}

impl Program {
    fn new(memory: Vec<i64>) -> Program {
        Program {
            id: 0,
            counter: 0,
            base: 0,
            cycles: 0,
            status: RunStatus::Running,
            memory,
            input: Some(VecDeque::new()),
            output: Some(VecDeque::new()),
        }
    }

    fn input(&mut self, value: i64) {
        if let Some(input) = &mut self.input {
            input.push_back(value);
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }
    fn output(&mut self) -> Option<i64> {
        if let Some(output) = &mut self.output {
            output.pop_front()
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }

    fn step(&mut self, steps: usize) {
        let mut ticks = 0;
        while steps > ticks {
            let instruction = self.memory[self.counter];
            // print!("{}:\t{}\t", self.counter, instruction);
            match opcode(instruction) {
                1 => {
                    // print!("add ");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    // print!(" -> ");
                    self.set_parameter(3, a + b);
                    self.counter = self.counter + 4;
                }
                2 => {
                    // print!("mul ");
                    let a = self.get_parameter(1);
                    let b = self.get_parameter(2);
                    // print!(" -> ");
                    self.set_parameter(3, a * b);
                    self.counter = self.counter + 4;
                }
                3 => {
                    let input = &self
                        .input
                        .as_mut()
                        .expect("tried to use input while backing buffer was removed")
                        .pop_front();
                    if let Some(value) = input {
                        // print!("pop {} ->", value);
                        self.set_parameter(1, *value);
                        // self.set(address as usize, *value);
                        self.counter = self.counter + 2;
                        // println!("<-");
                    } else {
                        // println!("no input - blocking");
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
                    // println!("");
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
                    // print!("if ");
                    let a = self.get_parameter(1);
                    // print!(" < ");
                    let b = self.get_parameter(2);
                    // print!(" -> ");

                    if a < b {
                        self.set_parameter(3, 1);
                    } else {
                        self.set_parameter(3, 0);
                    }
                    self.counter = self.counter + 4;
                }
                8 => {
                    // print!("if ");
                    let a = self.get_parameter(1);
                    // print!(" == ");
                    let b = self.get_parameter(2);

                    if a == b {
                        self.set_parameter(3, 1);
                    } else {
                        self.set_parameter(3, 0);
                    }
                    self.counter = self.counter + 4;
                }
                9 => {
                    // print!("adjust base\t",);
                    self.base = self.base + self.get_parameter(1);
                    // println!("\t({})", self.base);
                    self.counter = self.counter + 2;
                }
                99 => {
                    // println!("program finished");
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
    fn get_parameter(&mut self, offset: usize) -> i64 {
        match get_mode(self.memory[self.counter], offset) {
            0 => self.get_indirect(offset),
            1 => self.get_offset(offset), 
            2 => {
                let address = (self.memory[self.counter + offset] as i64 + self.base)as usize;
                self.get(address)
            },
            _ => panic!("unknown parameter mode"),
        }
    }
    fn get(&mut self, offset: usize) -> i64 {
        self.prep_get(offset);
        // print!("{{value:{}, address:{}}} ", self.memory[offset], offset);
        self.memory[offset]
    }
    fn get_offset(&mut self, offset: usize) -> i64 {
        self.prep_get(self.counter + offset);
        // print!("{{value:{}, address:{}}} ", self.memory[self.counter + offset], self.counter + offset);
        self.memory[self.counter + offset]
    }
    fn get_indirect(&mut self, offset: usize) -> i64 {
        
        self.prep_get(offset + self.counter);
        let address = self.memory[self.counter + offset] as usize;
        self.prep_get(address);
        // print!("{{value:{}, address:{}, calling address:{}}} ", self.memory[address], address, self.counter + offset);
        
        self.memory[address]
    }
    fn prep_get(&mut self, index: usize) {
        // println!("checking memory is atleast {}", index);
        if index as usize >= self.memory.len() {
            let need = index as usize - self.memory.len() + 1; 
            // print!("extending {}-integer memory by {} to allow for index up to {}", self.memory.len(), need, index); 
            self.memory.extend([0].iter().cycle().take(need));
        }
    }
    fn set_parameter(&mut self, offset: usize, value: i64) {
        match get_mode(self.memory[self.counter], offset) {
            0 => self.set_indirect(offset, value),
            1 => panic!("tried to set an immediate value"), 
            2 => self.set_indirect_base(offset, value),
            _ => panic!("unknown parameter mode"),
        }
    }
    
    fn set_indirect(&mut self, offset: usize, value: i64) {
        self.prep_get(self.counter + offset);
        let index = self.memory[self.counter + offset];
        // println!("{} @{}, ", value, index);
        self.set(index as usize, value);
    }
    fn set_indirect_base(&mut self, offset: usize, value: i64) {
        let extra = self.memory[self.counter + offset];
        // println!("setting indirect+base with {} + {} -> {}", self.base, extra, value);
        
        let address = (extra as i64 + self.base) as usize;
        // println!("add: {}", address);
        self.prep_get(address);
        self.set(address, value);
    }
    fn set(&mut self, index: usize, value: i64) {
        // println!("set {{value:{}, address:{}}} ", value, index);
        self.prep_get(index);
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
    fn _extract_output(&self) -> i64 {
        self.memory[0]
    }
}

impl FromStr for Program {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Program, Self::Err> {
        Ok(Program::new(
            input
                .split_terminator(',')
                .map(|num| num.parse::<i64>())
                .collect::<Result<Vec<i64>, Self::Err>>()?,
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
