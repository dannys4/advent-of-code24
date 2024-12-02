use regex::Regex;
use std::iter::zip;

// Parse the input as two columns of integers and return two vectors,
// one for each column.
fn parse_input(contents: &String) -> (Vec<i32>, Vec<i32>) {
    // Parse the input as two columns of integers
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    // Initialize two vectors to store the columns
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    // Iterate over the captures and store them in the vectors
    for cap in re.captures_iter(contents) {
        col1.push(cap[1].parse().unwrap());
        col2.push(cap[2].parse().unwrap());
    }
    return (col1, col2);
}

// Function that takes sorted vectors as input
// and computes Wasserstein-1 distance
fn part1(v1: &Vec<i32>, v2: &Vec<i32>) -> i64 {
    let mut result: i64 = 0;
    // Iterate over the two vectors and compute the absolute difference
    for (xi, yi) in zip(v1, v2) {
        let diff = (xi - yi).abs();
        result += diff as i64;
    }
    return result;
}

fn part2(v1: &Vec<i32>, v2: &Vec<i32>) -> i64 {
    // Nested iteration: For each element in v1, check how many times it appears
    // in v2 and multiply the occurrence count by the value
    let mut score = 0;
    for i in v1 {
        let mut idx2 = 0;
        while idx2 < v2.len() && v2[idx2] < *i {
            idx2 += 1;
        }
        let mut count = 0;
        while idx2 < v2.len() && v2[idx2] == *i {
            count += 1;
            idx2 += 1;
        }
        score += (count as i64) * (*i as i64);
    }
    return score;
}

pub fn fcn(contents: &String) {
    // Parse the input
    let (mut v1, mut v2) = parse_input(contents);

    // Sort the vectors
    v1.sort();
    v2.sort();

    // Compute the Wasserstein-1 distance
    let p1_ans = part1(&v1, &v2);
    println!("Part 1: {}", p1_ans);

    // Compute the Similarity score
    let p2_ans = part2(&v1, &v2);
    println!("Part 2: {}", p2_ans);
}