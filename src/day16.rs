use std::cmp::min;

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
        return ((0,0), false);
    }
    let next_pos = next_pos_opt.unwrap();
    if next_pos.0 >= maze.maze.len() || next_pos.1 >= maze.maze[next_pos.0].len() {
        return ((0,0), false);
    }
    let is_wall = maze.maze[next_pos.0][next_pos.1];
    if is_wall {
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
fn solve_maze(maze: &Maze) -> (usize,Vec<(usize,usize)>) {
    // Treat the graph as (position, direction, cost) tuple
    let mut next_visit: Vec<((usize,usize),Direction,usize,Vec<(usize,usize)>)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; maze.maze[0].len()]; maze.maze.len()];
    next_visit.push((maze.start, START_DIR, 0, Vec::new()));
    let mut min_cost = usize::MAX;
    let mut min_cost_history: Option<Vec<(usize,usize)>> = None;
    loop {
        let curr_pos_opt = next_visit.pop();
        if curr_pos_opt.is_none() {
            break;
        }
        let (curr_pos, curr_dir, curr_cost, history) = curr_pos_opt.unwrap();
        if curr_pos == maze.end {
            if curr_cost < min_cost {
                min_cost = curr_cost;
                min_cost_history = Some(history);
            }
            continue;
        }
        if visited[curr_pos.0][curr_pos.1] {
            continue;
        }
        visited[curr_pos.0][curr_pos.1] = true;
        let mut history_copy = history.clone();
        history_copy.push(curr_pos);
        for next_dir in vec![Direction::ColLf, Direction::ColRt, Direction::RowUp, Direction::RowDn] {
            let (next_pos, is_valid) = is_valid_step(maze, curr_pos, next_dir);
            if !is_valid {
                continue;
            }
            let next_cost = if next_dir == curr_dir {STRAIGHT_COST} else if next_dir == opp_direction(curr_dir) {STRAIGHT_COST+2*TURN_COST} else {STRAIGHT_COST+TURN_COST};
            next_visit.push((next_pos, next_dir, curr_cost + next_cost, history_copy.clone()));
        }
    }
    return (min_cost, min_cost_history.unwrap());
}

fn print_maze_history(maze: &Maze, history: &Vec<(usize,usize)>) {
    let mut maze_chars: Vec<Vec<char>> = Vec::new();
    for row in 0..maze.maze.len() {
        let mut maze_row = Vec::new();
        for col in 0..maze.maze[row].len() {
            maze_row.push(if maze.maze[row][col] {'#'} else {'.'});
        }
        maze_chars.push(maze_row);
    }
    for pos in history {
        maze_chars[pos.0][pos.1] = 'X';
    }
    for row in maze_chars {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part1(maze: &Maze) -> usize {
    let (maze_cost, maze_history) = solve_maze(maze);
    print_maze_history(maze, &maze_history);
    return maze_cost;
}

pub fn fcn(contents: &String) {
    let maze = parse_contents(contents);
    let p1 = part1(&maze);
    println!("Part 1: {p1}");
}