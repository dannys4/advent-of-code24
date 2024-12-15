use std::collections::{HashSet, HashMap};
// Parse the contents as a matrix of characters
// Also return all unique characters in the matrix
fn parse_contents(contents: &String) -> (Vec<Vec<char>>, Vec<char>) {
    let mut unique_chars: HashSet<char> = HashSet::new();
    let grid = contents.lines().map(|x| {
        let ret = x.chars().collect::<Vec<char>>();
        for c in ret.iter() {
            unique_chars.insert(*c);
        }
        return ret;
    }).collect();
    return (grid, unique_chars.into_iter().collect());
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

fn check_next_pos(pos: (usize, usize), dir: Direction, maxes: (usize, usize)) -> Option<(usize, usize)> {
    let (max0, max1) = maxes;
    match dir {
        Direction::North  => {if pos.0 >    0 {Some((pos.0-1,pos.1  ))} else {None}},
        Direction::South  => {if pos.0 < max0 {Some((pos.0+1,pos.1  ))} else {None}},
        Direction::East   => {if pos.1 >    0 {Some((pos.0  ,pos.1-1))} else {None}},
        Direction::West   => {if pos.1 < max1 {Some((pos.0  ,pos.1+1))} else {None}},
    }
}

// Return how much to add to perimeter
fn explore_direction(grid: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, c: char, pos: (usize, usize), dir: Direction, stacks: &mut HashMap<char, Vec<(usize, usize)>>) -> usize {
    let maxes = (grid.len() - 1, grid[pos.0].len()-1);
    let next_pos_opt = check_next_pos(pos, dir, maxes);
    if next_pos_opt.is_none() {
        return 1;
    }
    let next_pos = next_pos_opt.unwrap();

    let c_next = grid[next_pos.0][next_pos.1];
    if !visited[next_pos.0][next_pos.1] {
        let next_stack = stacks.get_mut(&c_next).unwrap();
        next_stack.push(next_pos);
    }
    let p_ret = (c_next != c) as usize;
    // print!(", {:?}'{}'({})", dir, c_next, p_ret);
    return p_ret
}

fn explore_tile(grid: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, c: char, pos: (usize, usize), stacks: &mut HashMap<char, Vec<(usize, usize)>>) -> usize {
    // Explore in all four directions (or as many as possible).
    // In every direction that has a different character, add one to the perimeter
    // print!("{}: ({},{})", c, pos.0, pos.1);
    let east_p = explore_direction(grid, visited, c, pos, Direction::East, stacks);
    let west_p = explore_direction(grid, visited, c, pos, Direction::West, stacks);
    let north_p = explore_direction(grid, visited, c, pos, Direction::North, stacks);
    let south_p = explore_direction(grid, visited, c, pos, Direction::South, stacks);
    // println!("E{}, W{}, N{}, S{}", c, pos.0, pos.1, east_p, west_p, north_p, south_p);
    // print!("\n");
    return east_p + west_p + north_p + south_p;
}

fn find_first_stack(stacks: &HashMap<char, Vec<(usize, usize)>>) -> char {
    // Find first stack that has elements
    let mut which_stack = '.';
    for (k, v) in stacks.iter() {
        if v.len() > 0 {
            which_stack = *k;
            break;
        }
    }
    return which_stack
}

fn part1(grid: &Vec<Vec<char>>, unique_chars: &Vec<char>) -> usize {
    let mut visited: Vec<Vec<bool>> = grid.iter().map(|x| x.iter().map(|_| false).collect::<Vec<bool>>()).collect();
    let mut stacks: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut area_perims: HashMap<char, (usize, usize)> = HashMap::new();
    for c in unique_chars {
        stacks.insert(*c, vec![]);
        area_perims.insert(*c, (0,0));
    }
    let first_char = grid[0][0];
    stacks.get_mut(&first_char).unwrap().push((0,0));
    loop {
        let which_stack = find_first_stack(&stacks);
        if which_stack == '.' {
            break;
        } else {
            let stack = stacks.get_mut(&which_stack).unwrap();
            let next_pos = stack.pop().unwrap();
            if !visited[next_pos.0][next_pos.1] {
                let area_perim = area_perims.get_mut(&which_stack).unwrap();
                let add_perim = explore_tile(grid, &visited, which_stack, next_pos, &mut stacks);
                area_perim.0 += 1;
                area_perim.1 += add_perim;
                visited[next_pos.0][next_pos.1] = true;
            }
        }
    }
    let ret: usize = area_perims.iter().map(|ap| ap.1.0 * ap.1.1).sum();
    for (k,v) in area_perims {
        println!("{}: {:?}", k, v);
    }
    return ret;
}

pub fn fcn(contents: &String) {
    // Parse the input
    let (grid, chars) = parse_contents(contents);
    let p1 = part1(&grid, &chars);
    println!("Part 1: {}", p1);
}