use core::num;

// Parse each line of input as a vector of integers
fn parse_input(contents: &String) -> Vec<Vec<i32>> {
    // Initialize a vector to store the parsed input
    let mut result = Vec::new();
    // Iterate over the lines of the input
    for line in contents.lines() {
        // Parse the line as a vector of integers
        let row: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        // Store the parsed row in the result vector
        result.push(row);
    }
    return result;
}

fn is_safe_undamped_inner(report: &Vec<i32>)->(bool, bool, bool){
    let mut increasing = true;
    let mut decreasing = true;
    let mut safe_change = true;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        increasing &= diff > 0;
        decreasing &= diff < 0;
        safe_change &= diff.abs() <= 3;
    }
    return (increasing, decreasing, safe_change);
}

fn is_safe_undamped(report: &Vec<i32>)->bool{
    let (increasing, decreasing, safe_change) = is_safe_undamped_inner(report);
    return safe_change && (increasing || decreasing);
}

fn is_safe_damped(report: &Vec<i32>)->bool{
    let (increasing, decreasing, safe_change) = is_safe_undamped_inner(report);
    if safe_change && (increasing || decreasing) {
        return true;
    }

    // Go through each pair of numbers and check if it would be increasing except for one skipped number
    let mut one_skipped_diff = false;
    let mut last_idx_increasing = 0;
    let mut last_idx_decreasing = 0;
    let mut num_skipped_increasing = 0;
    let mut num_skipped_decreasing = 0;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() > 3 || diff == 0 {
            if one_skipped_diff {
                return false;
            }
            one_skipped_diff = true;
            continue
        }
        if diff < 0 {
            num_skipped_increasing += 1;
            last_idx_increasing = i-1;
        } else {
            num_skipped_decreasing += 1;
            last_idx_decreasing = i-1;
        }
    }
    println!("num_skipped_increasing: {}, num_skipped_decreasing: {}", num_skipped_increasing, num_skipped_decreasing);
    return num_skipped_increasing < 2 || num_skipped_decreasing < 2;
}

fn count_safe(reports: &Vec<Vec<i32>>, use_damped: bool)->i32{
    let is_safe_fcn = if use_damped {is_safe_damped} else {is_safe_undamped};
    let mut safe = 0;
    for report in reports {
        let is_safe = is_safe_fcn(report);
        println!("{:?} is safe: {}", report, is_safe);
        safe += is_safe as i32;
    }
    return safe;
}

fn part1(reports: &Vec<Vec<i32>>) -> i32 {
    return count_safe(reports, false);
}

fn part2(reports: &Vec<Vec<i32>>) -> i64 {
    return count_safe(reports, true) as i64;
}

pub fn fcn(contents: &String) {
    let reports = parse_input(contents);
    // let p1_ans = part1(&reports);
    // println!("Part 1: {}", p1_ans);
    let p2_ans = part2(&reports);
    println!("Part 2: {}", p2_ans);
}