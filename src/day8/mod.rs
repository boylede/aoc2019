use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::VecDeque;

use aoc2019::Day;

const DAY: i32 = 8;

fn part1(lines: &Vec<String>) {
    let pixels = lines[0].chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let width = 25;
    let height = 6;
    // let layers : Vec<Vec<u32>> = Vec::new();
    let num_layers = pixels.len() / (width * height);
    println!("image has {} layers", num_layers);
    let layers = pixels.windows(width * height);
    let (_, count) = layers.fold((usize::max_value(), 0), |(num_zeros, num_sum), layer| {
        let count = layer.iter().filter(|d| **d == 0).count();
        if count < num_zeros {
            let ones = layer.iter().filter(|d| **d == 1).count();
            let twos = layer.iter().filter(|d| **d == 2).count();
            (count, ones * twos)
        } else {
            (num_zeros, num_sum)
        }
    });
   
    println!("Part 1: {:?}", count);
}

fn print_image(image: &Vec<u32>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if image[x + y * width] == 0 {
                print!(" ");
            } else {
                print!("W");
            }
            
        }
        print!("\n");
    }
    print!("\n\n");
}

fn part2(lines: &Vec<String>) {
    let pixels = lines[0].chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let width = 25;
    let height = 6;
    let mut layers = pixels.windows(width * height).step_by(width*height);
    let num_layers = layers.clone().count();
    println!("layers: {}", num_layers);
    // let mut out = File::create("out_image.txt").unwrap();
    
    let mut image: Vec<u32> = Vec::from(layers.next().unwrap());
    
    layers.clone().enumerate().for_each(|(layer_num, layer)| {
        // write!(out, "{}\n", layer_num);
        let rows = layer.windows(width).step_by(width);
        // let row_count = rows.clone().count();
        // println!("rows in this layer: {}", row_count);
        rows.enumerate().for_each(|(y, row)| {
            // write!(out, "{}\t:", i);
            row.iter().enumerate().for_each(|(x, d)| {
                if image[x + y*width] == 2 {
                    image[x + y*width] = *d;
                }
            });
            // write!(out, "\n");
        });
        // write!(out, "\n");
    });
    // out.flush();
    
    // print_image(&image, width, height);
    // for layer in layers {
    //     for (i, pixel) in image.iter_mut().enumerate() {
    //         if *pixel == 2 && layer[i] != 2 {
    //             *pixel = layer[i];
    //         }
    //     }
    //     // print_image(&Vec::from(layer), width, height);
    // }
    print_image(&image, width, height);
    println!("Part 2: {:?}", 0);
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
    
}
