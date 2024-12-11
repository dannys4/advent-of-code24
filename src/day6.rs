fn parse_contents(contents: &String)->(usize, Vec<Vec<bool>>, (usize,usize)) {
    // Returns length of row, grid representing where # are, and location of carot ^
    let lines = contents.lines();
    let mut row_len = 0;
    let mut first_line = true;
    let mut grid = Vec::new();
    let mut carot = (0,0);
    for (row,line) in lines.enumerate() {
        // Get the length of the first line
        if first_line {
            row_len = line.len();
            first_line = false;
        }
        // Parse the line into a vector of bools
        let mut row_vec = Vec::with_capacity(row_len);
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                carot = (row, col);
            }
            row_vec.push(c == '#');
        }
        grid.push(row_vec);
    }
    return (row_len, grid, carot);
}

// Create enum for direction
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn fill_in_direction(grid: &Vec<Vec<bool>>, visit_grid: &mut Vec<Vec<bool>>, guard: (usize,usize), direction: &Direction) -> (bool, (usize, usize)) {
    let mut row = guard.0;
    let mut col = guard.1;
    let mut exits = false;
    loop {
        // Check if we have visited this spot
        visit_grid[row][col] = true;

        // Check if we go out of bounds
        match direction {
            Direction::Up => if row == 0 { exits = true },
            Direction::Down => if row == grid.len()-1 { exits = true },
            Direction::Left => if col == 0 { exits = true },
            Direction::Right => if col == grid[0].len()-1 { exits = true },
        }
        if exits {
            break;
        }

        // Check if we have hit a wall
        let hit_wall;
        match direction {
            Direction::Up => hit_wall = grid[row-1][col],
            Direction::Down => hit_wall = grid[row+1][col],
            Direction::Left => hit_wall = grid[row][col-1],
            Direction::Right => hit_wall = grid[row][col+1],
        }
        if hit_wall {
            break;
        }

        // Move in the direction
        match direction {
            Direction::Up => row -= 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
        }

    }
    return (exits, (row, col));
}

// Function to print the grid, dead code
#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>, visited: &Vec<Vec<bool>>, carot: (usize,usize), direction: &Direction) {
    let mut grid_str = String::new();
    for (row_idx, row) in grid.clone().into_iter().enumerate() {
        for (col_idx,spot) in row.into_iter().enumerate() {
            if (row_idx, col_idx) == carot {
                match direction {
                    Direction::Up => grid_str.push('^'),
                    Direction::Down => grid_str.push('v'),
                    Direction::Left => grid_str.push('<'),
                    Direction::Right => grid_str.push('>'),
                }
            }
            else if visited[row_idx][col_idx] {
                grid_str.push('X');
            } else if spot {
                grid_str.push('#');
            } else {
                grid_str.push('.');
            }
        }
        grid_str.push('\n');
    }
    println!("{}", grid_str);
}


fn part1(row_len: usize, grid: &Vec<Vec<bool>>, carot: (usize,usize)) -> (i32, Vec<Vec<bool>>) {
    // Create a grid to keep track of visited spots
    let mut visit_grid = vec![vec![false; row_len]; grid.len()];

    // Create the grid as a string for debugging
    // print_grid(&grid, &visit_grid, carot, &Direction::Up);

    // Start at the carot
    let mut guard = carot;
    let mut direction = Direction::Up;
    loop {
        // Fill in the direction
        let (exits, new_guard) = fill_in_direction(&grid, &mut visit_grid, guard, &direction);
        if exits {
            break;
        }

        // Turn right
        match direction {
            Direction::Up => direction = Direction::Right,
            Direction::Right => direction = Direction::Down,
            Direction::Down => direction = Direction::Left,
            Direction::Left => direction = Direction::Up,
        }

        // Move to the new guard
        guard = new_guard;
        // print_grid(&grid, &visit_grid, guard, &direction);
        // break;
    }


    // Finish by summing over all the visited spots
    let mut count = 0;
    for row in &visit_grid {
        for visited in row {
            count += *visited as i32;
        }
    }
    return (count, visit_grid);
}

fn go_in_direction(grid: &Vec<Vec<bool>>, carot: (usize, usize), direction: &Direction) -> (bool, (usize, usize)) {
    let mut row = carot.0;
    let mut col = carot.1;
    let mut exits = false;
    loop {
        // Check if we go out of bounds
        match direction {
            Direction::Up => if row == 0 { exits = true },
            Direction::Down => if row == grid.len()-1 { exits = true },
            Direction::Left => if col == 0 { exits = true },
            Direction::Right => if col == grid[0].len()-1 { exits = true },
        }
        if exits {
            break;
        }

        // Check if we have hit a wall
        let hit_wall;
        match direction {
            Direction::Up => hit_wall = grid[row-1][col],
            Direction::Down => hit_wall = grid[row+1][col],
            Direction::Left => hit_wall = grid[row][col-1],
            Direction::Right => hit_wall = grid[row][col+1],
        }
        if hit_wall {
            break;
        }

        // Move in the direction
        match direction {
            Direction::Up => row -= 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
        }

    }
    return (exits, (row, col));
}

fn is_loop(grid: &Vec<Vec<bool>>, carot: (usize, usize)) -> bool {
    // Check if this grid induces a loop
    // Start at the carot
    let mut guard = carot;
    let mut direction = Direction::Up;
    let mut ret = false;
    loop {
        // Fill in the direction
        let (exits, new_guard) = go_in_direction(grid, guard, &direction);
        if new_guard == carot {
            ret = true;
            break;
        }

        if exits {
            break;
        }

        // Turn right
        match direction {
            Direction::Up => direction = Direction::Right,
            Direction::Right => direction = Direction::Down,
            Direction::Down => direction = Direction::Left,
            Direction::Left => direction = Direction::Up,
        }

        // Move to the new guard
        guard = new_guard;
    }
    return ret;
}

fn part2(grid: &Vec<Vec<bool>>, visited: &Vec<Vec<bool>>, carot: (usize, usize)) -> i32{
    // Check every location that the carot can go to.
    // If you change grid to be true at that location, does it cause a loop?
    let mut count = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if !visited[row_idx][col_idx] {
                continue;
            }
            let mut new_grid = grid.clone();
            new_grid[row_idx][col_idx] = true;
            count += is_loop(&new_grid, carot) as i32;
        }
    }
    return count;
}

pub fn fcn(contents: &String) {
    let (row_len, grid, carot) = parse_contents(contents);
    let (p1_result, visited) = part1(row_len, &grid, carot);
    println!("Part 1: {}", p1_result);
    let p2_result = part2(&grid, &visited, carot);
    println!("Part 2: {}", p2_result);
}