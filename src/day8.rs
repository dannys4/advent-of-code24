use std::collections::{HashMap, HashSet};
// For each non-period character, insert all locations into a HashMap
// Also return the size of the matrix
fn create_positions(contents: &String) -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                let entry = positions.entry(c).or_insert(Vec::new());
                entry.push((x, y));
            }
        }
        num_rows += 1;
        if num_cols == 0 {
            num_cols = line.len();
        }
    }
    return (positions, (num_rows, num_cols));
}

fn is_inbounds(pos: (isize, isize), size: (usize, usize)) -> bool {
    return pos.0 >= 0 && pos.0 < size.1 as isize && pos.1 >= 0 && pos.1 < size.0 as isize;
}

fn insert_antinodes_part1(x0: isize, y0: isize, x1: isize, y1: isize, size: (usize, usize), antinodes: &mut HashSet<(usize, usize)>) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let (p0, p1) = ((x0, y0), (x1, y1));
    let pos = (x0 - dx, y0 - dy);
    if is_inbounds(pos, size) && pos != p0 && pos != p1 {
        antinodes.insert((pos.0 as usize, pos.1 as usize));
    }
    let pos = (x1 + dx, y1 + dy);
    if is_inbounds(pos, size) && pos != p0 && pos != p1 {
        antinodes.insert((pos.0 as usize, pos.1 as usize));
    }
}

fn insert_antinodes_part2(x0: isize, y0: isize, x1: isize, y1: isize, size: (usize, usize), antinodes: &mut HashSet<(usize, usize)>) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let (p0, p1) = ((x0, y0), (x1, y1));
    let mut pos = (x0 - dx, y0 - dy);
    while is_inbounds(pos, size) {
        if pos != p0 && pos != p1 {
            antinodes.insert((pos.0 as usize, pos.1 as usize));
        }
        pos = (pos.0 - dx, pos.1 - dy);
    }
    pos = (x1 + dx, y1 + dy);
    while is_inbounds(pos, size) {
        if pos != p0 && pos != p1 {
            antinodes.insert((pos.0 as usize, pos.1 as usize));
        }
        pos = (pos.0 + dx, pos.1 + dy);
    }
}

fn find_all_antinodes(
    positions: &Vec<(usize, usize)>,
    size: (usize, usize),
    curr_antinodes: &mut HashSet<(usize, usize)>,
    is_part1: bool,
) {
    for j0 in 0..positions.len() {
        let (x0, y0) = positions[j0];
        let (x0, y0) = (x0 as isize, y0 as isize);
        for j1 in (j0+1)..positions.len() {
            let (x1, y1) = positions[j1];
            let (x1, y1) = (x1 as isize, y1 as isize);
            if is_part1 {
                insert_antinodes_part1(x0, y0, x1, y1, size, curr_antinodes);
            } else {
                insert_antinodes_part2(x0, y0, x1, y1, size, curr_antinodes);
            }
        }
    }
}

fn solve_part(positions: &HashMap<char, Vec<(usize, usize)>>, size: (usize, usize), is_part1: bool) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (_, pos) in positions {
        find_all_antinodes(pos, size, &mut antinodes, is_part1);
        if !is_part1 && pos.len() > 1 {
            for (x, y) in pos {
                antinodes.insert((*x, *y));
            }
        }
    }
    return antinodes.len();
}

fn part1(positions: &HashMap<char, Vec<(usize, usize)>>, size: (usize, usize)) -> usize {
    return solve_part(positions, size, true);
}

fn part2(positions: &HashMap<char, Vec<(usize, usize)>>, size: (usize, usize)) -> usize {
    return solve_part(positions, size, false);
}

// Function to run for day 8
pub fn fcn(contents: &String) {
    let (positions, size) = create_positions(contents);
    let p1 = part1(&positions, size);
    println!("Part 1: {}", p1);
    let p2 = part2(&positions, size);
    println!("Part 2: {}", p2);
}
