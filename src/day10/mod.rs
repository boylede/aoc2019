use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use aoc2019::Day;

const DAY: i32 = 10;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Slope {
    sign_x: i32,
    sign_y: i32,
    value: OrderedFloat<f32>,
}

fn part1(lines: &Vec<String>) {
    let asteroids : HashSet<(i32, i32)> = lines
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
        let h: HashSet<(i32, i32)> = line
        .chars()
        .enumerate()
        .filter_map(|(x, c) | {
            if c == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect();
        h
    }).collect();

    let mut matches : HashMap<(i32, i32), u32> = HashMap::new();
    for asteroid in asteroids.iter() {
        let visible: HashSet<Slope> = asteroids
            .iter()
            .zip(vec![asteroid].iter().cycle())
            .filter(|(base, asteriod)| {
                *base != **asteriod
            })
            .map(|(base, asteriod)| {
                let run: f32 = (asteriod.0 - base.0) as f32;
                let rise: f32 = (asteriod.1 - base.1) as f32;
                let sign_x = sign(run);
                let sign_y = sign(rise);
                let value : OrderedFloat<f32> = (rise / run).into();
                println!("{:?}", value);
                Slope {sign_x, sign_y, value}
            })
            .collect();
        let count = visible.len() as u32;
        matches.insert(*asteroid, count);
    }
    let best = matches.iter().fold(0, |best, (k, v)| {
        println!("{:?}: {}", k, v);
        if *v > best {
            *v
        } else {
            best
        }
    });
   
    println!("Part 1: {:?}", best);
}

fn sign(num: f32) -> i32 {
    if num >= 0.0 {
        1
    } else {
        -1
    }
}

fn part2(lines: &Vec<String>) {
    let asteroids = lines
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
        line
        .chars()
        .enumerate()
        .filter_map(|(x, c) | {
            if c == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<(i32, i32)>>()
    }).collect::<HashSet<(i32, i32)>>();

    let candidates : HashMap<(i32, i32), usize> = asteroids
    .iter()
    .map(|&asteroid| {
        let visible: HashSet<Slope> = asteroids
            .iter()
            .zip(vec![asteroid].iter().cycle())
            .filter(|(base, asteriod)| {
                *base != *asteriod
            })
            .map(|(base, asteriod)| {
                let run: f32 = (asteriod.0 - base.0) as f32;
                let rise: f32 = (asteriod.1 - base.1) as f32;
                let sign_x = sign(run);
                let sign_y = sign(rise);
                let value : OrderedFloat<f32> = (rise / run).into();
                // println!("{:?}", value);
                let distance = (asteroid.0 - base.0).abs() + (asteroid.1 - base.1).abs();
                Slope {sign_x, sign_y, value}
            })
            .collect();
        let count = visible.len();
        (asteroid, count)
    }).collect();

    let mut candidates_iter = candidates.iter();
    let first = candidates_iter.next().unwrap();

    let (base, _) = candidates.iter().fold(first, |(asteroid, score), (k, v)| {
        if v > score {
            (k, v)
        } else {
            (asteroid, score)
        }
    });

    let strike_list : BTreeMap<(Slope, i32), (i32, i32)> = asteroids
    .iter()
    .map(|&asteroid| {
        
        let run: f32 = (asteroid.0 - base.0) as f32;
        let rise: f32 = (asteroid.1 - base.1) as f32;
        let sign_x = if run >= 0.0 {
            1
        } else {
            -1
        };
        let sign_y = if rise >= 0.0 {
            1
        } else {
            -1
        };
        let value : OrderedFloat<f32> = (rise / run).into();
        
        let distance = (asteroid.0 - base.0).abs() + (asteroid.1 - base.1).abs();
        let slope = Slope {sign_x, sign_y, value};

        ((slope, distance), asteroid)
    }).collect();
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
