use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use aoc2019::Day;

const DAY: i32 = 3;

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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct RelativeLine {
    direction: Direction,
    distance: i32,
}

impl RelativeLine {
    fn extract_points(&self, mut x: i32, mut y: i32) -> Vec<Coordinate> {
        let mut pts = vec![];
        for _ in 0..self.distance {
            match self.direction {
                Direction::Up => y = y + 1,
                Direction::Right => x = x + 1,
                Direction::Left => x = x - 1,
                Direction::Down => y = y - 1,
            }
            pts.push(Coordinate { x, y })
        }
        pts
    }
    fn _end(&self, mut x: i32, mut y: i32) -> (i32, i32) {
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
        let direction = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction in input"),
        };
        Ok(RelativeLine {
            direction,
            distance,
        })
    }
}

fn part1(lines: &Vec<String>) {
    let a: Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[0]
        .split(',')
        .map(|l| l.parse::<RelativeLine>())
        .collect();
    let a = a.unwrap();

    let a_points: HashSet<Coordinate> = a
        .iter()
        .fold(vec![Coordinate{x:0,y:0}], |mut points, r| {
            let Coordinate{x:xx,y:yy} = points.last().unwrap();
            let new_points = r.extract_points(*xx, *yy);
            points.extend(new_points);
            points
        })
        .iter()
        .map(|point| *point)
        .collect();

    let b: Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[1]
        .split(',')
        .map(|l| l.parse::<RelativeLine>())
        .collect();
    let b = b.unwrap();
    let b_points: HashSet<Coordinate> = b
        .iter()
        .fold(vec![Coordinate{x:0,y:0}], |mut points, r| {
            let Coordinate{x:xx,y:yy} = points.last().unwrap();
            let new_points = r.extract_points(*xx, *yy);
            points.extend(new_points);
            points
        })
        .iter()
        .map(|point| *point)
        .collect();

    let best: i32 = a_points
        .intersection(&b_points)
        // .inspect(|point| {
        //     println!("{:?}", point);
        // })
        .map(|Coordinate { x, y }| x.abs() + y.abs())
        .fold(
            10000,
            |highest, current| {
                if current < highest && current != 0 {
                    current
                } else {
                    highest
                }
            },
        );
    println!("Part 1: {}", best);
}

fn part2(lines: &Vec<String>) {
    let a: Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[0]
        .split(',')
        .map(|l| l.parse::<RelativeLine>())
        .collect();
    let a = a.unwrap();

    let a_points: HashMap<Coordinate, i32> = a
        .iter()
        .fold(vec![Coordinate{x:0,y:0}], |mut points, r| {
            let Coordinate{x:xx,y:yy} = points.last().unwrap();
            let new_points = r.extract_points(*xx, *yy);
            points.extend(new_points);
            points
        })
        .iter()
        .enumerate()
        .map(|(dist, point)| {
            (*point, dist as i32)
        }).collect();

    let b: Result<Vec<RelativeLine>, std::num::ParseIntError> = lines[1]
        .split(',')
        .map(|l| l.parse::<RelativeLine>())
        .collect();
    let b = b.unwrap();
    let mut common_points: HashMap<Coordinate, i32> = b
        .iter()
        .fold(vec![Coordinate{x:0,y:0}], |mut points, r| {
            let Coordinate{x:xx,y:yy} = points.last().unwrap();
            let new_points = r.extract_points(*xx, *yy);
            points.extend(new_points);
            points
        })
        .iter()
        .enumerate()
        .filter_map(|(dist, point)| {
            if a_points.contains_key(point) {
                let d = a_points.get(point).unwrap();
                Some((dist as i32 + d, point))
            } else {
                None
            }
        })
        .map(|(dist, point)| {
            (*point, dist as i32)
        }).collect();

    let best = common_points.drain().fold(10000, |best, (_, distance)| {
        if distance < best && distance != 0 {
            distance
        } else {
            best
        }
    });
    println!("Part 2: {}", best);
}
