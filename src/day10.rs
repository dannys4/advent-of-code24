use std::collections::HashSet;

// Given a string of integers 0-9 separated by newlines
// Create a matrix
fn parse_contents(contents: &String) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            if c == '.' {
                row.push(u8::MAX);
            } else {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }
        matrix.push(row);
    }
    return matrix;
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn find_trailheads_helper(matrix: &Vec<Vec<u8>>, i: usize, j: usize, from: Direction, curr_level: u8, curr_reachable: &mut HashSet<(usize,usize)>) -> usize {
    if matrix[i][j] != curr_level {
        return 0;
    }
    if curr_level == 9 {
        curr_reachable.insert((i,j));
        return 1;
    }
    let mut rating = 0;
    if i > 0 && from != Direction::Down {
        rating += find_trailheads_helper(matrix, i - 1, j, Direction::Up, curr_level + 1, curr_reachable);
    }
    if i < matrix.len() - 1 && from != Direction::Up {
        rating += find_trailheads_helper(matrix, i + 1, j, Direction::Down, curr_level + 1, curr_reachable);
    }
    if j > 0 && from != Direction::Left {
        rating += find_trailheads_helper(matrix, i, j - 1, Direction::Right, curr_level + 1, curr_reachable);
    }
    if j < matrix[i].len() - 1 && from != Direction::Right {
        rating += find_trailheads_helper(matrix, i, j + 1, Direction::Left, curr_level + 1, curr_reachable);
    }
    return rating;
}

fn both_parts(matrix: &Vec<Vec<u8>>) -> (usize,usize) {
    let mut count = 0;
    let mut rating = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                let mut reachable_ij: HashSet<(usize,usize)> = HashSet::new();
                let ij_rating = find_trailheads_helper(matrix, i, j, Direction::None, 0, &mut reachable_ij);
                let ij_count = reachable_ij.len();
                count += ij_count;
                rating += ij_rating;
            }
        }
    }
    return (count, rating);
}

pub fn fcn(contents: &String) {
    let matrix = parse_contents(contents);
    let (p1, p2) = both_parts(&matrix);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}