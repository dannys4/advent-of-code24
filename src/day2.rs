use rayon::prelude::*;

/// Parse each line of input as a vector of integers
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

/// Check if a given report is safe, skipping the number at skip_idx
fn is_safe(report: &Vec<i32>, skip_idx: usize)->bool{
    let mut increasing = true;
    let mut decreasing = true;
    let mut safe_change = true;

    // For each consecutive pair of numbers, check if the difference is between 1 and 3
    // Also check if the sequence is increasing or decreasing
    // Skipping the number at skip_idx
    let loop_start = if skip_idx == 0 {2} else {1};
    let mut prev_idx = if skip_idx == 0 {1} else {0};
    for i in loop_start..report.len() {
        if i == skip_idx { // Skip the number at skip_idx
            continue
        }
        let diff = report[i] - report[prev_idx];

        // Update the flags
        increasing &= diff > 0;
        decreasing &= diff < 0;
        safe_change &= diff.abs() <= 3;
        prev_idx = i;
    }
    return safe_change && (increasing || decreasing);
}

/// Check if a given report is safe, skipping no numbers
fn is_safe_undamped(report: &Vec<i32>) -> i32 {
    return is_safe(report, usize::MAX) as i32;
}

/// Check if a given report is safe if you skip any one number
fn is_safe_damped(report: &Vec<i32>) -> i32{
    for skip_idx in 0..report.len() {
        if is_safe(report, skip_idx) {
            return 1;
        }
    }
    return 0;
}

/// Count the number of safe reports
fn count_safe(reports: &Vec<Vec<i32>>, use_damped: bool)->i32{
    // Choose the appropriate function to check if a report is safe
    let is_safe_fcn = if use_damped {is_safe_damped} else {is_safe_undamped};
    // Count the number of safe reports using parallel iterators
    let safe = reports
        .par_iter() // Use parallel iterator
        .fold(|| 0, // Initialize the accumulator
            |acc, report| {
                return acc + is_safe_fcn(report); // Map is_safe_fcn and accumulate
            })
        .sum::<i32>(); // Sum the results
    return safe;
}

/// Solve the puzzle for part 1
fn part1(reports: &Vec<Vec<i32>>) -> i32 {
    return count_safe(reports, false);
}

/// Solve the puzzle for part 2
fn part2(reports: &Vec<Vec<i32>>) -> i64 {
    return count_safe(reports, true) as i64;
}

/// Function to run the day's solutions
pub fn fcn(contents: &String) {
    let reports = parse_input(contents);
    let p1_ans = part1(&reports);
    println!("Part 1: {}", p1_ans);
    let p2_ans = part2(&reports);
    println!("Part 2: {}", p2_ans);
}