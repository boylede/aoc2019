use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::str::FromStr;

use aoc2019::Day;

const DAY: i32 = 3;

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

struct SpaceMarker {
    inner: Vec<RelativeLine>,
    index: usize,
    x: usize,
    y: usize,
}

impl SpaceMarker {
    fn mark(&mut self, space: &mut HashMap<Coordinate, State>) {
        //
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum State {
    Unvisited,
    Bert,
    Ernie,
    Both,
}


#[derive(Copy, Clone)]
struct RelativeLine {
    direction: Direction,
    distance: i32,
}

impl RelativeLine {
    fn extract_points(&self, mut x: i32, mut y: i32) -> Vec<Coordinate> {
        let mut pts = vec![];
        for i in 0..self.distance {
            match self.direction {
                Direction::Up => {
                    y = y + 1;
                }
                Direction::Right => x = x + 1,
                Direction::Left => x = x - 1,
                Direction::Down => y = y - 1,
            }
            pts.push(Coordinate{x, y})
        }
        pts
    }
    fn end(&self, mut x: i32, mut y: i32) -> (i32, i32) {
        match self.direction {
            Direction::Up => y = y + self.distance,
            Direction::Right => x = x + self.distance,
            Direction::Left => x = x - self.distance,
            Direction::Down => y = y - self.distance,
        }
        (x, y)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl FromStr for RelativeLine {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<RelativeLine, Self::Err> {
        let (dir, dist) = input.split_at(1);
        let distance = dist.parse::<i32>().unwrap();
        // println!("{} -> {}", dir, distance);
        let direction = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction in input"),
        };
        Ok(RelativeLine{direction, distance})
    }
}


fn part1(lines: &Vec<String>) {
    let mut space: HashMap<Coordinate, State> = HashMap::new();
    let b : Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[0].split(',').map(|l| l.parse::<RelativeLine>()).collect();
    let bert = b.unwrap();

    let mut x : i32 = 0;
    let mut y : i32 = 0;
    bert.iter().for_each(|r| {
        let points = r.extract_points(x, y);
        points.iter().for_each(|p| {
            space.entry(*p).and_modify(|e| {
                match e {
                    State::Unvisited => *e = State::Bert,
                    State::Bert => (),
                    State::Ernie => *e = State::Both,
                    State::Both => (),
                }
            }).or_insert(State::Bert);
        });
        let (u, v) = r.end(x, y);
        x = u;
        y = v;
    });

    x = 0;
    y = 0;
    let e : Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[1].split(',').map(|l| l.parse::<RelativeLine>()).collect();
    let ernie = e.unwrap();
    ernie.iter().for_each(|r| {
        let points = r.extract_points(x, y);
        points.iter().for_each(|p| {
            space.entry(*p).and_modify(|e| {
                match e {
                    State::Unvisited => *e = State::Ernie,
                    State::Bert => *e = State::Both,
                    State::Ernie => (),
                    State::Both => (),
                }
            }).or_insert(State::Ernie);
        });
        let (u, v) = r.end(x, y);
        x = u;
        y = v;
    });
    let candidates : i32 = space.iter().filter(|(c, s)| **s == State::Both).map(|(Coordinate{x, y}, _)| x.abs() + y.abs() ).fold(10000, |highest, current| {
        if current < highest {
            current 
        } else {
            highest
        }
    });
    println!("candidate distances {:?}", candidates);
    println!("Part 1: {}", 0);
    
}

fn part2(lines: &Vec<String>) {
    println!("Part 2: {}", 0);
}
