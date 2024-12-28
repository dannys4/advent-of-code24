// Progress bars
use indicatif::ProgressBar;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green
}

fn parse_color(color: char) -> Color {
    return match color {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        _ => panic!("Invalid color")
    }
}

fn number_patterns(pattern: &[Color], towels: &Vec<Vec<Color>>, map: &mut HashMap<Vec<Color>,usize>) -> usize {
    if map.contains_key(pattern) {
        return map[pattern];
    }
    if pattern.len() == 0 {
        return 1;
    }
    let mut current_count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            current_count += number_patterns(&pattern[towel.len()..], towels, map);
        }
    }
    map.insert(pattern.to_vec(), current_count);
    return current_count;
}

fn parse_towels(towels_str: &str) -> Vec<Vec<Color>> {
    let towel_str_fcn = |towel_str: &str| towel_str.chars().map(parse_color).collect();
    return towels_str.split(", ").map(towel_str_fcn).collect();
}

fn parse_pattern(pattern_str: &str) -> Vec<Color> {
    return pattern_str.chars().map(|c| parse_color(c)).collect();
}

fn parse_contents(contents: &String) -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    let mut lines = contents.lines();
    let towels_str = lines.next().unwrap();
    let towels = parse_towels(towels_str);
    lines.next();
    let mut patterns = Vec::new();
    for pattern_str in lines {
        patterns.push(parse_pattern(pattern_str));
    }
    return (patterns, towels);
}

fn both_parts(patterns: &Vec<Vec<Color>>, towels: &Vec<Vec<Color>>) -> (usize, usize) {
    let mut count = 0;
    let mut total_combos = 0;
    // Track progress
    let pb = ProgressBar::new(patterns.len() as u64);
    for pattern in patterns {
        let mut map = HashMap::new();
        let combo_pattern = number_patterns(&pattern, towels, &mut map);
        count += (combo_pattern > 0) as usize;
        total_combos += combo_pattern;
        pb.inc(1);
    }
    pb.finish();
    return (count, total_combos);
}


pub fn fcn(contents: &String) {
    let (patterns, towels) = parse_contents(contents);
    let (result1, result2) = both_parts(&patterns, &towels);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}