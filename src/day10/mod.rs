use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2019::Day;

const DAY: i32 = 10;

fn check_visible(space: &HashSet<(usize, usize)>, x: usize, y: usize) -> bool {
    if let Some(asteroid) = space.get(&(x, y)) {
        true
    } else {
        false
    }
}
fn hits(space: &HashSet<(usize, usize)>, origin: (usize, usize), target: (usize, usize)) -> bool {
    unimplemented!()
}


fn part1(lines: &Vec<String>) {
    let space : HashSet<(usize, usize)> = lines
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
        let h: HashSet<(usize, usize)> = line
        .chars()
        .enumerate()
        .filter_map(|(x, c) | {
            if c == '#' {
                Some((x, y))
            } else {
                None
            }
        })
        .collect();
        h
    }).collect();

    for y in 0..36 {
        for x in 0..36 {
            if check_visible(&space, x, y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    // let asteroids :HashSet= space.iter().filter_map(|(k, v)| {
    //     if v {
    //         k
    //     }
    // })
    let mut matches : HashMap<(usize, usize), u32> = HashMap::new();
    for asteroid in space.iter() {
        for y in 0..36*2 {
            for x in 0..36*2 {
                if hits(&space, *asteroid, (x, y)) {
                    matches.entry((x, y)).and_modify(|c| *c = *c + 1).or_insert(1);
                }
            }
        }
    }

    // space.iter().for_each(|t| {
    //     t.iter().for_each(|pa| {
    //         if *pa {
    //             print!("█");
    //         } else {
    //             print!(" ");
    //         }
    //     });
    //     print!("\n");
    // });
    // space.iter().

    // let pixels = lines[0]
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect::<Vec<u32>>();
    // let width = 25;
    // let height = 6;

    // let layers = pixels.windows(width * height).step_by(width * height);
    // let (_, count) = layers.fold((usize::max_value(), 0), |(num_zeros, num_sum), layer| {
    //     let zero_count = layer.iter().filter(|d| **d == 0).count();
    //     if zero_count < num_zeros {
    //         let ones = layer.iter().filter(|d| **d == 1).count();
    //         let twos = layer.iter().filter(|d| **d == 2).count();
    //         (zero_count, ones * twos)
    //     } else {
    //         (num_zeros, num_sum)
    //     }
    // });

    println!("Part 1: {:?}", 0);
}

fn print_image(image: &Vec<u32>, width: usize, height: usize) {
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

fn part2(lines: &Vec<String>) {
    // let pixels = lines[0]
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect::<Vec<u32>>();
    // let width = 25;
    // let height = 6;
    // let mut layers = pixels.windows(width * height).step_by(width * height);

    // let mut image: Vec<u32> = Vec::from(layers.next().unwrap());

    // image = layers.fold(image, |image, layer| {
    //     layer
    //         .iter()
    //         .zip(image)
    //         .map(|(layer_pixel, mut image_pixel)| {
    //             if image_pixel == 2 && *layer_pixel != 2 {
    //                 image_pixel = *layer_pixel;
    //             }
    //             image_pixel
    //         })
    //         .collect()
    // });

    println!("Part 2: {}", 0);
    // print_image(&image, width, height);
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
