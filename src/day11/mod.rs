use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use aoc2019::Day;
use crate::intcode::Program;

const DAY: i32 = 11;

fn simulate_robot(mut program: Program, starting_point: i64) -> HashMap<Point, i64> {
    let mut area : HashMap<Point, i64> = HashMap::new();
    let mut current_place = Point(0,0);
    let mut direction = Direction::Up;
    area.insert(current_place, starting_point);
    while program.status.unfinished() {
        let current = area.entry(current_place).or_insert(0);
        program.input(*current);
        program.execute();
        let paint = program.output().unwrap();
        if *current != paint {
            *current = paint;
        }
        
        let turn = program.output().unwrap();
        use self::Direction::*;
        direction = if turn == 0 {
            match direction {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            }
        } else if turn == 1 {
            match direction {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            }
        } else {
            panic!("did not understand turning instructions.");
        };
        current_place = current_place + match direction {
            Up => Point(0, 1),
            Left => Point(-1, 0),
            Down => Point(0, -1),
            Right => Point(1, 0),
        };
    }
    area
}

fn part1(lines: &Vec<String>) {
    let program: Program = lines[0].parse().unwrap();
    let image = simulate_robot(program, 0);
    let value = image.keys().len();
    println!("Part 1: {:?}", value);
}

fn part2(lines: &Vec<String>) {
    let program: Program = lines[0].parse().unwrap();
    let area = simulate_robot(program, 1);
    let left = area.iter().fold(0, |best, (Point(x, _y), _color)|{
        if best > *x {
            *x
        } else {
            best
        }
    });
    let right = area.iter().fold(0, |best, (Point(x, _y), _color)|{
        if best < *x {
            *x
        } else {
            best
        }
    });
    let top = area.iter().fold(0, |best, (Point(_x, y), _color)|{
        if best < *y {
            *y
        } else {
            best
        }
    });
    let bottom = area.iter().fold(0, |best, (Point(_x, y), _color)|{
        if best > *y {
            *y
        } else {
            best
        }
    });

    let width = (right - left + 1) as usize;
    let height = (top - bottom + 1) as usize;

    let image: Vec<i64> = area.iter().fold( vec![0; width * height], |mut image, (Point(u, v), color)| {
        let x = u;
        let y = -v;
        let index = x + (y * width as i32);

        image[index as usize] = *color;
        image
    });
    println!("Part 2:");
    print_image(&image, width, height);
}

fn print_image(image: &Vec<i64>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if image[x + y * width] == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        print!("\n");
    }
    print!("\n\n");
}
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
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
