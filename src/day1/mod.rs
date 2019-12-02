use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2019::Day;

const DAY: i32 = 1;

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
    let fuel_sum = lines.iter()
    .map(|line| {
        let num = line.parse::<i32>().unwrap();
        // print!("num: {} = ", num);
        num
    }).fold(0, |accumulator, mass| {
        // println!("{}:{}", mass, accumulator);
        (mass / 3) - 2 + accumulator
    });
    println!("part 1 answer: {}", fuel_sum);
}

fn part2(lines: &Vec<String>) {
    let fuel_sum = lines.iter()
    .map(|line| {
        let num = line.parse::<i32>().unwrap();
        // print!("num: {} = ", num);
        num
    }).fold(0, |accumulator, mass| {
        let modules_fuel = (mass / 3) - 2;
        // print!("module needed: {} + ", modules_fuel);
        
        let mut supporting_fuel : Vec<i32> = Vec::new();
        let mut recent_fuel = modules_fuel;
        while recent_fuel > 0 {
            // print!("{} + ", recent_fuel);
            supporting_fuel.push(recent_fuel);
            recent_fuel = (recent_fuel / 3) - 2;
        }
        // println!("module needed: {:?}", supporting_fuel);
        supporting_fuel.iter().fold(0, |accum, fuel| accum + fuel) + accumulator
    });
    println!("part 2 answer: {}", fuel_sum);
}
