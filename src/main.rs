extern crate aoc2019;
extern crate lapp;

use aoc2019::get_input_file;
use aoc2019::Config;
use aoc2019::Day;

use std::process;
// use std::env;

mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

const USAGE: &'static str = "Dan Boyle's Advent of Code 2019 entries.
	-d, --day (default 1) which day's code to run.
	-a, --all run all day modules currently present.
	-i, --input (string) optional input file.
	-s, --session (string) optional session string.";

fn main() {
    /* 	Load all challenges into a Vec for easy running */
    let mut days: Vec<Day> = Vec::new();

    day1::load(&mut days);
    day2::load(&mut days);
    day3::load(&mut days);
    day4::load(&mut days);
    day5::load(&mut days);
    day6::load(&mut days);
    day7::load(&mut days);
    day8::load(&mut days);
    day9::load(&mut days);
    day10::load(&mut days);
    day11::load(&mut days);
    day12::load(&mut days);

    /* 	Debug Prints */
    // let mut debug_text = String::new();
    // debug_text.push_str("Loaded days:\n");
    // for day in &days {
    // 	debug_text.push_str(&format!("{}\n", day));
    // }
    // print!("{}", debug_text);

    /* 	Parse Arguments */

    let mut args = lapp::Args::new(USAGE);
    match args.parse_result() {
        Ok(()) => (),
        Err(error) => {
            println!("Error parsing arguments: {}, try --help.", error);
            return;
        }
    }
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(0);
    });

    /* 	Main Logic */
    if config.all {
        for day in days {
            println!("Running day: {}", &day);
            day.run(get_input_file(day.index, &config.input).unwrap());
        }
    } else {
        let index = (config.day - 1) as usize;
        if index < days.len() {
            let day = &days[index];
            println!("Running 1 day: {}", &day);
            day.run(get_input_file(day.index, &config.input).unwrap());
        } else {
            println!("Invalid day selection: {}", config.day);
        }
    }
}
