fn parse_line(line: &str) -> (usize, Vec<usize>) {
    // Line is in the format:
    // N: M M M M ...
    let mut colon_split = line.split(": ");
    let n = colon_split.next().unwrap().parse::<usize>().unwrap();
    let m = colon_split.next().unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    return (n, m);
}

fn concat(first: usize, second: usize) -> usize {
    // Concatenate a and b
    return format!("{}{}", first, second).parse::<usize>().unwrap();
}

fn result_helper(desired: usize, check_num: usize, numbers: &[usize], use_cat: bool, _level: i32) -> bool {
    // See if the numbers can be combined using concat, add, mul to get desired
    // Print number of tabs according to level, then desired and numbers
    let _level_tabs = "\t".repeat(_level as usize);
    // println!("{}Desired: {}\n{}Numbers: {:?}", level_tabs, desired, "\t".repeat(level as usize), numbers);
    if numbers.len() == 0 {
        return check_num == desired;
    }
    let add_check_num = check_num + numbers[0];
    // Check if addition works
    if result_helper(desired, add_check_num, &numbers[1..], use_cat, _level+1) {
        return true;
    }
    let mul_check_num = if check_num == 0 { numbers[0] } else {  check_num * numbers[0] };
    if result_helper(desired, mul_check_num, &numbers[1..], use_cat, _level+1) {
        return true;
    }
    if use_cat {
        let cat_check_num = concat(check_num, numbers[0]);
        if result_helper(desired, cat_check_num, &numbers[1..], use_cat, _level+1) {
            return true;
        }
    }
    return false;
}

fn result(lines: &Vec<(usize, Vec<usize>)>, use_cat: bool) -> usize {
    let mut count = 0;
    for (_j,(desired, numbers)) in lines.into_iter().enumerate() {
        // println!("Checking line {}", j);
        let result = result_helper(*desired, 0, numbers, use_cat, 0);
        if result {
            count += *desired;
        }
        // println!("Result {}: {}\n\n", j, result);
    }
    return count;
}

fn part1(lines: &Vec<(usize, Vec<usize>)>) -> usize {
    return result(lines, false);
}

fn part2(lines: &Vec<(usize, Vec<usize>)>) -> usize {
    return result(lines, true);
}

pub fn fcn(contents: &String) {
    let lines = contents.lines().map(parse_line).collect::<Vec<(usize, Vec<usize>)>>();
    let p1 = part1(&lines);
    println!("Part 1 result: {}", p1);
    let p2 = part2(&lines);
    println!("Part 2 result: {}", p2);
}