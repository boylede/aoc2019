use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2019::Day;

const DAY: i32 = 6;

fn part1(lines: &Vec<String>) {
    let relationships : HashMap<&str, &str> = lines.iter().map(|line| {
        let kv = line.split(")").collect::<Vec<&str>>();
        (kv[1], kv[0])
    }).collect();

    let count = relationships.keys().fold(0, |mut total, mut k| {
        while *k != "COM" {
            k = relationships.get(k).unwrap();
            total = total + 1;
        }
        total
    });
    println!("Part 1: {:?}", count);
}

fn part2(lines: &Vec<String>) {
    let relationships : HashMap<&str, &str> = lines.iter().map(|line| {
        let kv = line.split(")").collect::<Vec<&str>>();
        (kv[1], kv[0])
    }).collect();
    
    let mut my_tree = vec![];
    let mut key = "YOU";
    while key != "COM" {
        key = relationships.get(key).unwrap();
        my_tree.push(key);
    }
    let mut santa_tree = vec![];
    let mut key = "SAN";
    while key != "COM" {
        key = relationships.get(key).unwrap();
        santa_tree.push(key);
    }

    let mut mine = my_tree.pop().unwrap();
    let mut santas = santa_tree.pop().unwrap();

    while *mine == *santas {
        mine = my_tree.pop().unwrap();
        santas = santa_tree.pop().unwrap();
    }

    println!("Part 2: {:?}", my_tree.len() + santa_tree.len() + 2);
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
