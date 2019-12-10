use std::collections::VecDeque;
use std::collections::HashMap;
use std::str::FromStr;


pub trait VirtualMachine {
    fn put_input(&mut self, value: i64);
    fn take_output(&mut self) -> Option<i64>;
    fn step(&mut self, steps: usize);
    fn can_run(&self) -> bool;
    fn unblock(&mut self) -> bool;
    fn execute(&mut self) {
        self.unblock();
        while self.can_run() {
            let count = self.step(100);
        }
    }
}
impl VirtualMachine for Program {
    fn put_input(&mut self, value: i64) {
        self.input(value)
    }
    #[inline]
    fn take_output(&mut self) -> Option<i64> {
        self.output()
    }
    fn step(&mut self, steps: usize) {
        self.step(steps)
    }
    #[inline]
    fn can_run(&self) -> bool {
        self.status.unfinished()
    }
    #[inline]
    fn unblock(&mut self) -> bool {
        if self.status.blocked() {
            self.status = RunStatus::Running;
            true
        } else {
            false
        }
    }
}

pub struct LoopNetwork<VM> 
where VM: VirtualMachine
{
    first: u32,
    last: u32,
    vms: HashMap<u32, VM>,
    connections: HashMap<u32, u32>,
    inputs: HashMap<u32, i64>,
    closed: bool,
    last_output: Option<i64>,
    // outputs: HashMap<u32, i64>,
}


impl<VM> LoopNetwork<VM> 
where VM: VirtualMachine
{
    pub fn new() -> Self {
        LoopNetwork {
            first: 0,
            last: 0,
            vms: HashMap::new(),
            connections: HashMap::new(),
            inputs: HashMap::new(),
            last_output: None,
            closed: false,
        }
    }
    pub fn insert(&mut self, id: u32, vm: VM) -> Option<VM> {
        if self.vms.len() == 0 {
            self.first = id;
        }
        let old = self.vms.insert(id, vm);
        // self.connect(self.last, id);
        self.connections.insert(self.last, id);
        self.last = id;
        old
    }
    pub fn connect_ends(&mut self) {
        self.connections.insert(self.last, self.first);
        self.closed = true;
    }
    // fn connect(&mut self, a: u32, b: u32) -> Option<u32> {
    //     self.connections.insert(a, b)
    // }
}

impl<VM> VirtualMachine for LoopNetwork<VM> 
where VM: VirtualMachine
{
    fn put_input(&mut self, value: i64) {
        // self.vms.get_mut(&self.first).unwrap().put_input(value)
        self.inputs.insert(self.first, value);
    }
    fn take_output(&mut self) -> Option<i64> {
        if self.closed {
            self.last_output
        } else {
            self.vms.get_mut(&self.last).unwrap().take_output()
        }
    }
    fn step(&mut self, steps: usize) {
        for (id, vm) in self.vms.iter_mut() {
            if let Some(input) = self.inputs.remove(id) {
                vm.put_input(input);
            }
            vm.step(steps);
            if let Some(pair) = self.connections.get(id) {
                if let Some(output) = vm.take_output() {
                    if *id == self.last {
                        self.last_output = Some(output);
                    }
                    self.inputs.insert(*pair, output);
                }
            }
        }
    }
    fn can_run(&self) -> bool {
        // if self.vms.values().all(|vm| ! vm.can_run()) {
        //     println!("All VMs blocked");
        //     panic!("Stopped");
        //     false
        // } else {
        //     true
        // }
        self.vms.values().any(|vm| vm.can_run())
    }
    fn unblock(&mut self) -> bool {
        self.vms.values_mut().map(|vm| vm.unblock()).any(|b|b)
    }
}

#[derive(Clone)]
pub struct Program {
    pub id: usize,
    counter: usize,
    base: i64,
    pub cycles: usize,
    pub status: RunStatus,
    memory: Vec<i64>,
    pub input: Option<VecDeque<i64>>,
    pub output: Option<VecDeque<i64>>,
}


#[derive(Clone, PartialEq, Debug)]
pub enum RunStatus {
    Running,
    Finished,
    Killed,
    Blocked,
}

impl RunStatus {
    #[inline]
    pub fn running(&self) -> bool {
        *self == RunStatus::Running
    }
    #[inline]
    pub fn blocked(&self) -> bool {
        *self == RunStatus::Blocked
    }
    #[inline]
    pub fn unfinished(&self) -> bool {
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
    pub fn new(memory: Vec<i64>) -> Program {
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

    pub fn input(&mut self, value: i64) {
        if let Some(input) = &mut self.input {
            input.push_back(value);
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }
    pub fn output(&mut self) -> Option<i64> {
        if let Some(output) = &mut self.output {
            output.pop_front()
        } else {
            panic!("tried to add input while buffer was moved");
        }
    }

    pub fn step(&mut self, steps: usize) {
        // print!("stepping vm {}", self.id);
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
        // println!(", {} times ended in {:?}", self.cycles, self.status);
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
    pub fn get(&mut self, offset: usize) -> i64 {
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
    pub fn set(&mut self, index: usize, value: i64) {
        // println!("set {{value:{}, address:{}}} ", value, index);
        self.prep_get(index);
        self.memory[index] = value;
    }
    pub fn execute(&mut self) {
        if self.status.blocked() {
            self.status = RunStatus::Running;
        }
        while self.status.running() {
            self.step(100);
        }
    }
    pub fn dump_ram(self) -> Vec<i64> {
        self.memory
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