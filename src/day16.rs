use std::{cmp::min, collections::HashSet};

const TURN_COST: usize = 1000;
const STRAIGHT_COST: usize = 1;

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum Direction {
    RowUp,
    RowDn,
    ColLf,
    ColRt
}

const START_DIR: Direction = Direction::ColRt;

struct Maze {
    maze: Vec<Vec<bool>>, // Shows where each wall is
    start: (usize, usize),
    end: (usize, usize)
}

// Parse contents as maze
fn parse_contents(contents: &String) -> Maze {
    let mut maze: Vec<Vec<bool>> = Vec::new();
    let mut start = (0,0);
    let mut end = (0,0);
    for (row,line) in contents.lines().enumerate() {
        let mut maze_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => {start = (row, col);},
                'E' => {end = (row, col);},
                _ => {}
            }
            maze_row.push(c == '#'); // Push whether it is wall
        }
        maze.push(maze_row);
    }
    return Maze{maze, start, end};
}

fn get_offset(dir: Direction) -> (i8, i8) {
    return match dir {
        Direction::ColLf => {(0,-1)},
        Direction::RowUp => {(-1,0)},
        Direction::RowDn => {( 1,0)},
        Direction::ColRt => {( 0,1)},
    }
}

fn get_next_pos(curr_pos: (usize, usize), offset: (i8, i8)) -> Option<(usize, usize)> {
    if offset.0 < 0 && curr_pos.0 == 0 {
        return None;
    }
    if offset.1 < 0 && curr_pos.1 == 0 {
        return None;
    }
    let row = if offset.0 < 0 {curr_pos.0 - 1} else {curr_pos.0 + offset.0 as usize};
    let col = if offset.1 < 0 {curr_pos.1 - 1} else {curr_pos.1 + offset.1 as usize};
    return Some((row, col));
}

fn is_valid_step(maze: &Maze, curr_pos: (usize, usize), dir: Direction) -> ((usize, usize), bool) {
    let offset = get_offset(dir);
    let next_pos_opt = get_next_pos(curr_pos, offset);
    if next_pos_opt.is_none() {
        // println!("\tNext Position is Out-of-bounds");
        return ((0,0), false);
    }
    let next_pos = next_pos_opt.unwrap();
    if next_pos.0 >= maze.maze.len() || next_pos.1 >= maze.maze[next_pos.0].len() {
        // println!("\tNext Position {:?} is Out-of-bounds", next_pos);
        return ((0,0), false);
    }
    let is_wall = maze.maze[next_pos.0][next_pos.1];
    if is_wall {
        // println!("\tCurr pos {:?}, Next position {:?} is a wall", curr_pos, next_pos);
        return ((0,0), false);
    }
    return (next_pos, true);
}

fn opp_direction(dir: Direction) -> Direction {
    return match dir {
        Direction::ColLf => {Direction::ColRt},
        Direction::ColRt => {Direction::ColLf},
        Direction::RowDn => {Direction::RowUp},
        Direction::RowUp => {Direction::RowDn},
    };
}

// Find quickest way through maze
fn maze_step(maze: &Maze, curr_pos: (usize, usize), dir: Direction, history:HashSet<(usize,usize)>, curr_cost: usize, curr_best: usize) -> usize {
    // println!("Current pos: {:?}, direction: {:?}, current_cost: {:?}", curr_pos, dir, curr_cost);
    if curr_pos == maze.end || curr_cost >= curr_best {
        // println!("Ending!");
        return curr_cost;
    }
    let mut min_cost = curr_best;
    let opp_dir = opp_direction(dir);
    for new_dir in [Direction::ColLf, Direction::ColRt, Direction::RowDn, Direction::RowUp] {
        if new_dir == opp_dir {continue}
        // println!("Checking direction {:?}...", new_dir);
        let (new_pos, is_valid) = is_valid_step(maze, curr_pos, new_dir);
        // if is_valid {println!("\tValid direction!")};
        if is_valid && !history.contains(&new_pos) {
            let mut new_history = history.clone();
            new_history.insert(new_pos);
            let new_add_cost = STRAIGHT_COST + ((dir != new_dir) as usize)*TURN_COST;
            let new_total_cost = maze_step(maze, new_pos, new_dir, new_history, curr_cost + new_add_cost, min_cost);
            min_cost = min(min_cost, new_total_cost);
        }
    }
    return min_cost;
}

fn part1(maze: &Maze) -> usize {
    let history: HashSet<(usize,usize)> = HashSet::new();
    let total_cost = maze_step(maze, maze.start, START_DIR, history, 0, usize::MAX);
    return total_cost;
}

// fn print_maze(maze: &Maze, pos: (usize, usize)) {
//     for (i,line) in maze.maze.iter().enumerate() {
//         for (j, is_wall) in line.iter().enumerate() {
//             let is_pos = (i,j) == pos;
//             let print_str = if is_pos {'X'} else { if *is_wall {'#'} else {'.'}};
//             print!("{}",print_str);
//         }
//         println!();
//     }
// }

pub fn fcn(contents: &String) {
    let maze = parse_contents(contents);
    // print_maze(&maze, maze.start);
    let p1 = part1(&maze);
    println!("Part 1: {p1}");
}