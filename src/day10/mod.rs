use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2019::Day;

const DAY: i32 = 10;

fn check_visible(space: &HashSet<(i32, i32)>, point: (i32, i32)) -> bool {
    if let Some(_) = space.get(&point) {
        true
    } else {
        false
    }
}

fn lerp(start: i32, end: i32, t: f32) -> i32 {
    let start = start as f32;
    let end = end as f32;
    (start + t * (end - start)) as i32
}

fn lerp_point(start: (i32, i32), end: (i32, i32), t: f32) -> (i32, i32) {
    (lerp(start.0, end.0, t), lerp(start.1, end.1, t))
}
fn dist(start: (i32, i32), end: (i32, i32)) -> i32 {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    std::cmp::max(dx, dy)
}

fn hits(space: &HashSet<(i32, i32)>, origin: (i32, i32), target: (i32, i32)) -> Option<(i32, i32)> {
    // let (ox, oy) = origin;
    // let (tx, ty) = target;
    // let ray = (ox - tx, oy - ty);
    // for divisor in 1..52 {
    //     let (rx, ry) = (ray.0 / divisor, ray.1 / divisor);
    //     let point = (rx + tx, ry + ty);
    //     if check_visible(space, point) {
    //         return true
    //     }
    // }
    // println!("{:?}", ray);
    let mut points = vec![];
    let max_points = dist(origin, target) + 1;
    for step in 1..max_points {
        let t = step as f32 / max_points as f32;
        points.push(lerp_point(origin, target, t));
    }
    for point in points {
        if check_visible(space, point) {
            return Some(point);
        }
    }
    None
}

fn part1(lines: &Vec<String>) {
    let space : HashSet<(i32, i32)> = lines
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
    for asteroid in space.iter() {
        let mut visible: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..36 {
            for x in 0..36 {
                let x = x as i32;
                let y = y as i32;
                if (x == 0 || x == 35) || (y == 0 || y == 35) {
                    // print!("{:?}, {},{} = ", *asteroid, x, y);
                    if let Some(hit) = hits(&space, *asteroid, (x, y)) {
                        visible.insert(hit);        
                    }
                }
                // if check_visible(&space, x, y) {
                //     print!("#");
                // } else {
                //     print!(" ");
                // }
            }
            // print!("\n");
        }
        let count = visible.len() as u32;
        matches.insert(*asteroid, count);  //.entry(*asteroid).and_modify(|c| *c = *c + 1).or_insert(1);
    }
    let best = matches.iter().fold(0, |best, (k, v)| {
        println!("{:?}: {}", k, v);
        if *v > best {
            *v
        } else {
            best
        }
    });
    // let asteroids :HashSet= space.iter().filter_map(|(k, v)| {
    //     if v {
    //         k
    //     }
    // })

    // let mut matches : HashMap<(usize, usize), u32> = HashMap::new();
    // for asteroid in space.iter() {
    //     for y in 0..36*2 {
    //         for x in 0..36*2 {
    //             if hits(&space, *asteroid, (x, y)) {
    //                 matches.entry((x, y)).and_modify(|c| *c = *c + 1).or_insert(1);
    //             }
    //         }
    //     }
    // }

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

    println!("Part 1: {:?}", best);
}

// fn print_image(image: &Vec<u32>, width: usize, height: usize) {
//     for y in 0..height {
//         for x in 0..width {
//             if image[x + y * width] == 0 {
//                 print!(" ");
//             } else {
//                 print!("█");
//             }
//         }
//         print!("\n");
//     }
//     print!("\n\n");
// }

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
