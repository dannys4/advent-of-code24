use regex::Regex;

fn part1_line(line: &str) -> i32{
    // Create regex pattern of mul(x,y)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // For each match in the line, compute x * y and add to accumulator
    let mut acc = 0;
    for cap in re.captures_iter(line) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        acc += x * y;
    }
    return acc;
}

fn part2_line(line: &str) -> i32 {
    let re_filter = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();
    let new_line = re_filter.replace_all(line, "").to_string();
    return part1_line(&new_line);
}

fn part1(lines: &String) -> i32 {
    return lines.lines().map(|line| part1_line(line)).sum();
}

fn part2(lines: &String) -> i32 {
    let input = lines.replace("\n","");
    return part2_line(input.as_str());
}

pub fn fcn(contents: &String) {
    let result = part1(contents);
    println!("Part 1: {}", result);
    let result2 = part2(contents);
    println!("Part 2: {}", result2);
}