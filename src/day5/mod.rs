use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2019::Day;

use crate::intcode::Program;

const DAY: i32 = 5;

fn part1(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    // program.input(1);
    program.execute();
    // println!("{:?}", program.status);
    program.input(1);
    program.execute();
    // println!("{:?}", program.status);
    // program.step(5);
    let value = program.output.unwrap();
    println!("Part 1: {:?}", value);
}

fn part2(lines: &Vec<String>) {
    let mut program: Program = lines[0].parse().unwrap();
    program.input(5);
    program.execute();
    let value = program.output.unwrap();
    println!("Part 2: {:?}", value);
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
    pub fn run(test: &str) -> Vec<i64> {
        let mut program: Program = test.parse().unwrap();
        program.execute();
        if program.status != RunStatus::Finished {
            panic!("virtual machine terminated early");
        }
        program.dump_ram()
    }
    pub fn io_test(program: &str, input: i64) -> i64 {
        let mut program: Program = program.parse().unwrap();
        program.input(input);
        program.execute();
        if program.status != RunStatus::Finished {
            panic!("virtual machine terminated early");
        }
        program.output().unwrap()
    }

    assert_eq!(run("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
    assert_eq!(run("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
    assert_eq!(run("2,4,4,5,99,0"), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run("1,1,1,4,99,5,6,0,99"),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    assert_eq!(io_test("3,0,4,0,99", -9854), -9854);
    assert_eq!(io_test("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
    assert_eq!(io_test("3,9,7,9,10,9,4,9,99,-1,8", -6), 1);
    assert_eq!(io_test("3,3,1108,-1,8,3,4,3,99", 8), 1);
    assert_eq!(io_test("3,3,1107,-1,8,3,4,3,99", -8), 1);

    assert_eq!(io_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -9), 1);
    assert_eq!(io_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -9), 1);

    // todo: tests fail if there are linebreaks in input;
    let eight_tester = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    assert_eq!(io_test(eight_tester, -9), 999);
    assert_eq!(io_test(eight_tester, 8), 1000);
    assert_eq!(io_test(eight_tester, 9000), 1001);
}
