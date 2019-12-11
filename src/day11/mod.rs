use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2019::Day;
use crate::intcode::Program;
// use crate::intcode::VirtualMachine;

const DAY: i32 = 11;

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


fn part1(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    let mut area : HashMap<Point, i64> = HashMap::new();
    let mut current_place = Point(0,0);
    let mut direction = Direction::Up;
    let mut painted: HashSet<Point> = HashSet::new();
    while program.status.unfinished() {
        let current = area.entry(current_place).or_insert(0);
        program.input(*current);
        program.execute();
        let paint = program.output().unwrap();
        if *current != paint {
            *current = paint;
            painted.insert(current_place);
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
    // program.input(1);
    // program.execute();
    let value = painted.len();
    println!("Part 1: {:?}", value);
}

fn part2(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    let mut area : HashMap<Point, i64> = HashMap::new();
    let mut current_place = Point(0,0);
    let mut direction = Direction::Up;
    let mut painted: HashSet<Point> = HashSet::new();
    area.insert(current_place, 1);
    while program.status.unfinished() {
        let current = area.entry(current_place).or_insert(0);
        program.input(*current);
        program.execute();
        let paint = program.output().unwrap();
        if *current != paint {
            *current = paint;
            painted.insert(current_place);
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
    println!("lower left of image: {},{}", left, bottom);
    println!("upper right of image: {},{}", right, top);

    let width = (right - left + 1) as usize;
    let height = (top - bottom + 1) as usize;
    println!("width, height: {}, {} ", width, height);
    // let start = vec![0; width * height];
    let image: Vec<i64> = area.iter().fold( vec![0; width * height], |mut image, (Point(u, v), color)| {
        let x = u;
        let y = -v;
        let index = x + (y * width as i32);
        println!("storing pixel {},{} as {}", x, y, color);
        image[index as usize] = *color;
        image
    });
    // program.input(1);
    // program.execute();
    // let value = painted.len();
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
    use crate::intcode::RunStatus;
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
