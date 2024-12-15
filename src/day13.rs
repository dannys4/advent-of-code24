use regex;

// Use static arrays
fn matrix_solve2x2(matrix: [[f64; 2]; 2], vec: [f64; 2]) -> [f64; 2] {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    // let inv_matrix = [[matrix[1][1], -matrix[0][1]], [-matrix[1][0], matrix[0][0]]];
    let ret = [matrix[1][1] * vec[0] - matrix[0][1] * vec[1], -matrix[1][0] * vec[0] + matrix[0][0] * vec[1]];
    return [ret[0]/det, ret[1]/det];
}

fn get_block(block: &str, offset: usize) -> ([[f64; 2]; 2], [f64; 2]) {
    // Block formatting:
    //  Button A: X+M[0][0], Y+M[1][0]
    // Button B: X+M[0][1], Y+M[1][1]
    // Prize: X=b[0], Y=b[1]
    let lines = block.lines().collect::<Vec<&str>>();
    let mut matrix = [[0.0; 2]; 2];
    let mut vec = [0.0; 2];
    // Match first line with regex
    let re = regex::Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let cap = re.captures(lines[0]).unwrap();
    matrix[0][0] = cap[1].parse::<f64>().unwrap();
    matrix[1][0] = cap[2].parse::<f64>().unwrap();
    // Match second line with regex
    let re = regex::Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let cap = re.captures(lines[1]).unwrap();
    matrix[0][1] = cap[1].parse::<f64>().unwrap();
    matrix[1][1] = cap[2].parse::<f64>().unwrap();
    // Match third line with regex
    let re = regex::Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let cap = re.captures(lines[2]).unwrap();
    vec[0] = cap[1].parse::<f64>().unwrap() + offset as f64;
    vec[1] = cap[2].parse::<f64>().unwrap() + offset as f64;
    return (matrix, vec);
}

fn check_valid_soln(soln: [f64; 2]) -> bool {
    // Check if the solution is made from nonnegative integers
    if soln[0] < 0.0 || soln[1] < 0.0 {
        return false;
    }
    if soln[0] % 1.0 != 0.0 || soln[1] % 1.0 != 0.0 {
        return false;
    }
    return true;
}

fn solve_block(block: &str, cost: (usize, usize), offset: usize) -> usize {
    let (matrix, vec) = get_block(block, offset);
    let solution = matrix_solve2x2(matrix, vec);
    let soln_cost = cost.0 * solution[0] as usize + cost.1 * solution[1] as usize;
    let is_valid = check_valid_soln(solution);
    return (is_valid as usize) * soln_cost;
}

fn parse_contents(contents: &String, cost: (usize, usize), offset: usize) -> Vec<usize> {
    // Split contents by double newline
    let mut ret: Vec<usize> = Vec::new();
    for block in contents.split("\n\n") {
        ret.push(solve_block(block, cost, offset));
    }
    return ret;
}

pub fn fcn(contents: &String) {
    let p1_sol = parse_contents(contents, (3, 1), 0);
    println!("Part 1: {}", p1_sol.iter().sum::<usize>());
    let p2_sol = parse_contents(contents, (3, 1), 10000000000000);
    println!("Part 2: {}", p2_sol.iter().sum::<usize>());
}