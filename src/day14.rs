use regex;
use std::collections::HashSet;
use image::{RgbImage, Rgb};

fn math_mod(x: i64, m: usize) -> usize {
    // This function is used to correct the negative modulus of a number
    // For example, -1 % 5 = 4
    let ret = (x % m as i64 + m as i64) as usize % m;
    return ret
}

fn parse_contents(contents: &String) -> (Vec<(usize, usize)>,Vec<(i64, i64)>) {
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for line in contents.lines() {
        let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let cap = re.captures(line).unwrap();
        let p: (usize, usize) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let v: (i64, i64) = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        positions.push(p);
        velocities.push(v);
    }
    return (positions, velocities);
}

fn part1(positions: &Vec<(usize, usize)>, velocities: &Vec<(i64, i64)>, size: (usize, usize)) -> usize {
    let num_steps = 100;
    let traj = positions.iter().zip(velocities.iter()).map(
        |(p, v)| {
            let traj0 = math_mod(p.0 as i64 + v.0*(num_steps as i64), size.0);
            let traj1 = math_mod(p.1 as i64 + v.1*(num_steps as i64), size.1);
            return (traj0, traj1);
        })
        .collect::<Vec<(usize, usize)>>();
    // Get positions of all robots after 100 steps
    // Count how many are in each quadrant:
    let (mut ne, mut nw, mut se, mut sw) = (0, 0, 0, 0);
    for pos in &traj {
        ne += (pos.0 > size.0/2 && pos.1 < size.1/2) as usize;
        nw += (pos.0 < size.0/2 && pos.1 < size.1/2) as usize;
        se += (pos.0 > size.0/2 && pos.1 > size.1/2) as usize;
        sw += (pos.0 < size.0/2 && pos.1 > size.1/2) as usize;
    }
    println!("NE: {}, NW: {}, SE: {}, SW: {}", ne, nw, se, sw);
    return ne*nw*se*sw;
}

fn create_image(positions: &Vec<(usize, usize)>, size: (usize, usize)) {
    let mut img = RgbImage::new(size.0 as u32, size.1 as u32);
    for (x, y) in positions {
        img.put_pixel(*x as u32, *y as u32, Rgb([255, 255, 255]));
    }
    img.save("data/day14.png").unwrap();
}

fn part2(positions: &Vec<(usize, usize)>, velocities: &Vec<(i64, i64)>, size: (usize, usize)) -> usize {
    let mut positions = positions.clone();
    // Iterate until all positions are unique
    let mut step = 1;
    let max_unique = 0;
    let mut num_unique = 0;
    loop {
        let mut pos_set = HashSet::new();
        for (p, v) in positions.iter_mut().zip(velocities.iter()) {
            p.0 = math_mod(p.0 as i64 + v.0, size.0);
            p.1 = math_mod(p.1 as i64 + v.1, size.1);
            pos_set.insert((p.0, p.1));
        }
        if pos_set.len() == positions.len() {
            if num_unique == max_unique {
                break;
            }
            num_unique += 1;
        }
        step += 1;
    }
    create_image(&positions, size);
    return step;
}

pub fn fcn(contents: &String) {
    let (positions, velocities) = parse_contents(contents);
    let p1 = part1(&positions, &velocities, (101, 103));
    println!("Part 1: {}", p1);
    let p2 = part2(&positions, &velocities, (101, 103));
    println!("Part 2: {}", p2);
}