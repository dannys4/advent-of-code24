#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West
}

fn parse_contents(contents: &String, double_block: bool) -> ((usize,usize),Vec<Vec<char>>,Vec<Direction>) {
    // Parse the contents of the file into a position, grid of characters, and a list of directions
    // Return where the robot (represented by @)
    let mut pos = (0,0);
    let mut grid = Vec::new();
    let mut split_contents = contents.split("\n\n");
    let grid_str = split_contents.next().unwrap();
    let directions_str = split_contents.next().unwrap().replace("\n", "");
    // Parse the grid
    for (y, line) in grid_str.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                if double_block {
                    pos = (2*x,y);
                } else {
                    pos = (x,y);
                }
            }
            if double_block {
                if c == 'O' {
                    row.push('[');
                    row.push(']');
                } else if c == '#' {
                    row.push('#');
                    row.push('#');
                } else if c == '@' {
                    row.push(c);
                    row.push('.');
                } else {
                    row.push(c);
                    row.push(c);
                }
            } else {
                row.push(c);
            }
        }
        grid.push(row);
    }
    // Parse the directions
    let directions = directions_str.chars().enumerate().map(|(i, c)| {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '<' => Direction::East,
            '>' => Direction::West,
            _ => panic!("Invalid direction {} at index {}", c, i)
        }
    }).collect();
    return (pos, grid, directions);
}

fn attempt_step(grid: &mut Vec<Vec<char>>, pos: (usize,usize), dir: Direction) -> (usize,usize) {
    // Attempt to move the robot in the given direction
    let (x,y) = pos;
    let (dx,dy) = match dir {
        Direction::North => (0,-1),
        Direction::South => (0,1),
        Direction::East => (-1,0),
        Direction::West => (1,0)
    };
    let new_pos = (x as i32 + dx, y as i32 + dy);
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return pos;
    }
    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
    if new_pos.1 >= grid.len() || new_pos.0 >= grid[0].len() {
        return pos;
    }
    if grid[new_pos.1][new_pos.0] == '#' {
        return pos;
    }
    if grid[new_pos.1][new_pos.0] == 'O' {
        let crate_pos = attempt_step(grid, new_pos, dir);
        if crate_pos == new_pos {
            return pos;
        }
    }
    grid[new_pos.1][new_pos.0] = grid[y][x];
    grid[y][x] = '.';
    return new_pos;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    // Print the grid
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn get_gps(i: usize, j: usize) -> usize {
    return i*100 + j;
}

fn part1(pos: (usize,usize), grid: Vec<Vec<char>>, directions: Vec<Direction>) -> usize{
    // Part 1: Move the robot through the grid
    let mut grid = grid;
    let mut pos = pos;
    for dir in directions {
        pos = attempt_step(&mut grid, pos, dir);
    }
    print_grid(&grid);
    let mut gps_sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                let gps = get_gps(i,j);
                gps_sum += gps;
            }
        }
    }
    return gps_sum;
}

fn can_double_step(grid: &Vec<Vec<char>>, pos: (usize,usize), dir: Direction) -> bool {
    // Attempt to move the robot in the given direction
    let (x,y) = pos;
    let (dx,dy) = match dir {
        Direction::North => (0,-1),
        Direction::South => (0,1),
        Direction::East => (-1,0),
        Direction::West => (1,0)
    };
    let new_pos = (x as i32 + dx, y as i32 + dy);
    
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return false;
    }
    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
    
    if new_pos.1 >= grid.len() || new_pos.0 >= grid[0].len() {
        return false;
    }
    
    if grid[new_pos.1][new_pos.0] == '#' {
        return false;
    }
    
    if grid[new_pos.1][new_pos.0] == '[' {
        if dir == Direction::North || dir == Direction::South {
            let crate_left = can_double_step(grid, new_pos, dir);
            let crate_right = can_double_step(grid, (new_pos.0+1, new_pos.1), dir);
            return crate_left && crate_right;
        } else {
            return can_double_step(grid, new_pos, dir);
        }
    }
    
    if grid[new_pos.1][new_pos.0] == ']' {
        if dir == Direction::North || dir == Direction::South {
            let crate_left = can_double_step(grid, (new_pos.0-1, new_pos.1), dir);
            let crate_right = can_double_step(grid, new_pos, dir);
            return crate_left && crate_right;
        } else {
            return can_double_step(grid, new_pos, dir);
        }
    }
    
    return true;
}

fn perform_double_step(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: Direction) -> (usize, usize) {
    let c = grid[pos.1][pos.0];
    if c == '.' {
        // println!("Found empty space at {:?}", pos);
        return pos;
    }
    let (x,y) = pos;
    let (dx,dy) = match dir {
        Direction::North => (0,-1),
        Direction::South => (0,1),
        Direction::East => (-1,0),
        Direction::West => (1,0)
    };
    let new_pos = (x as i32 + dx, y as i32 + dy);
    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
    // println!("Moving {:?} to {:?}", pos, new_pos);
    perform_double_step(grid, new_pos, dir);
    if dir == Direction::North || dir == Direction::South {
        if c == '[' {
            let right_new_pos = (new_pos.0+1, new_pos.1);
            perform_double_step(grid, right_new_pos, dir);
            grid[right_new_pos.1][right_new_pos.0] = ']';
            grid[y][x+1] = '.';
        }
        if c == ']' {
            let left_new_pos = (new_pos.0-1, new_pos.1);
            perform_double_step(grid, left_new_pos, dir);
            grid[left_new_pos.1][left_new_pos.0] = '[';
            grid[y][x-1] = '.';
        }
    }
    grid[new_pos.1][new_pos.0] = c;
    grid[y][x] = '.';
    return new_pos;
}

fn part2(pos: (usize,usize), grid: Vec<Vec<char>>, directions: Vec<Direction>) -> usize {
    // Part 2: Move the robot through the grid
    let mut grid = grid;
    let mut pos = pos;
    for (_iter,dir) in directions.iter().enumerate() {
        // println!("Iteration {}: Moving {:?}", iter, dir);
        // print_grid(&grid);
        let can_step = can_double_step(&grid, pos, *dir);
        // println!("Can step: {}\n", can_step);
        if can_step {
            // println!("Performing double step at {:?}", pos);
            pos = perform_double_step(&mut grid, pos, *dir);
        }
    }
    // print_grid(&grid);
    let mut gps_sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '[' {
                let gps = get_gps(i,j);
                gps_sum += gps;
            }
        }
    }
    return gps_sum;
}

pub fn fcn(contents: &String) {
    let (pos, grid, directions) = parse_contents(contents, false);
    let p1 = part1(pos, grid, directions);
    println!("Part 1: {}", p1);
    let (pos, grid, directions) = parse_contents(contents, true);
    let p2 = part2(pos, grid, directions);
    println!("Part 2: {}", p2);
}