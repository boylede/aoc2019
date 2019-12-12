use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2019::Day;

const DAY: i32 = 12;

fn calculate_energy([x, y, z]: [i32;3]) -> i32 {
    // println!("{} + {} + {} = {}", x.abs(), y.abs(), z.abs(), x.abs() + y.abs() + z.abs());
    x.abs() + y.abs() + z.abs()
}

fn print(pos: &Vec<[i32;3]>, vel: &Vec<[i32;3]>, num: usize) {
    println!("after {}", num);
    pos.iter().zip(vel.iter()).for_each(|(p, v)| {
        println!("pos: {:?} vel: {:?}", p, v);
    });
    println!("");

}


fn part1(lines: &Vec<String>) {
    // let lines = vec!["<x=-1, y=0, z=2>","<x=2, y=-10, z=-7>","<x=4, y=-8, z=8>","<x=3, y=5, z=-1>"];
    // let lines = vec!["<x=-8, y=-10, z=0>","<x=5, y=5, z=10>","<x=2, y=-7, z=3>","<x=9, y=-8, z=-3>"];
    let mut moon_positions : Vec<[i32;3]> = lines.iter().map(|line| {
        let mut parts: Vec<String> = line.split(",").map(|s|s.to_string()).collect();
        let x = parts[0].split_off(3).parse::<i32>().unwrap();
        let y = parts[1].split_off(3).parse::<i32>().unwrap();
        let mut z = parts[2].split_off(3);
        z.pop();
        let z = z.parse::<i32>().unwrap();
        // println!("{}, {}, {}", x, y, z);
        [x, y, z]
        // (1, 2, 3)
    }).collect();
    let mut moon_velocities : Vec<[i32;3]> = vec![[0;3];4];

    for turn in 0..1000 {
        // print(&moon_positions, &moon_velocities, turn);
        //gravity
        for (i, moon) in moon_positions.iter().enumerate() {
            for (j, other) in moon_positions.iter().enumerate() {
                if i != j {
                    // println!("comparing {} with {}", i, j);
                    for axis in 0..3 {
                        if moon[axis] > other[axis] {
                            moon_velocities[i][axis] -= 1;
                            // moon_velocities[j][axis] += 1;
                        } else if moon[axis] < other[axis]{
                            // moon_velocities[j][axis] -= 1;
                            moon_velocities[i][axis] += 1;
                        }
                    }
                }
            }
        }
        // velocity
        for (i, moon) in moon_velocities.iter().enumerate() {
            for axis in 0..3 {
                moon_positions[i][axis] += moon[axis];
            }
        }
    }
    // print(&moon_positions, &moon_velocities, 100);
    let total_energy = moon_positions.iter().zip(moon_velocities.iter()).fold(0, |t, (pos, vel)| {
        let p = calculate_energy(*pos);
        let v = calculate_energy(*vel);
        t + (p * v)
    });
    // println("pot: {}", potential_energy);
    // let kinetic_energy = moon_velocities.iter().fold(0, |t, moon| {
    //     let e = calculate_energy(*moon);
    //     t + e
    // });
    // println!("pot: {}, kin: {}, *= {}", total_energy, kinetic_energy, potential_energy * kinetic_energy);
    // let total_energy = potential_energy * kinetic_energy;

    println!("Part 1: {:?}", total_energy);
}

fn part2(lines: &Vec<String>) {
    println!("Part 2: {}", 0);
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
