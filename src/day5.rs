use std::collections::HashMap;

// Split contents into two sections based on where \n\n is located
// The first section is a list of tuples of (i32, i32).
// I return a HashMap taking in a number and returning numbers that correspond to it
// The second section is a list of lists of i32
fn parse_contents(contents: &String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let first_section: Vec<(i32, i32)> = sections[0].lines().map(|line| {
        let parts: Vec<&str> = line.split("|").collect();
        let p1: i32 = parts[0].parse().unwrap();
        let p2: i32 = parts[1].parse().unwrap();
        return (p1, p2);
    }).collect();
    let second_section: Vec<Vec<i32>> = sections[1].lines().map(|line| {
        return line.split(",").map(|num| num.parse().unwrap()).collect();
    }).collect();
    // If first_section is (x, y), get list of all (y) corresponding to each (x)
    let mut first_section_nums: HashMap<i32, Vec<i32>> = HashMap::new();
    for (x, y) in first_section {
        if first_section_nums.contains_key(&x) {
            first_section_nums.get_mut(&x).unwrap().push(y);
        } else {
            first_section_nums.insert(x, vec![y]);
        }
    }
    return (first_section_nums, second_section);
}

fn check_valid_list(first_section: &HashMap<i32, Vec<i32>>, list: &[i32]) -> bool {
    // Recursively check validity of list
    if list.len() < 1 {
        return true;
    }
    // Get the first number in the list
    let x = list[list.len() - 1];
    // Get all numbers that correspond to x
    let y_list_option = first_section.get(&x);
    if !y_list_option.is_none() {
        let y_list = y_list_option.unwrap();
        // Make sure no numbers in y_list are in the list
        for y in y_list {
            if list.contains(y) {
                return false;
            }
        }
    }
    // Recurse with the rest of the list
    return check_valid_list(first_section, &list[..list.len()-1]);
}

// Modify list inplace to make it valid
fn make_valid_list(first_section: &HashMap<i32, Vec<i32>>, list: &[i32]) {
    // Recursively make a valid list
    if list.len() < 1 {
        return;
    }
    // Get the first number in the list
    let x = list[list.len() - 1];
    // Get all numbers that correspond to x
    let y_list_option = first_section.get(&x);
    if !y_list_option.is_none() {
        let y_list = y_list_option.unwrap();
        // Make sure no numbers in y_list are in the list
        for y in y_list {
            if let Some(index) = list.iter().position(|&num| num == *y) {
                // Move all the other numbers up one index
                for i in index..list.len()-1 {
                    list[i+1] = list[i];
                }
                list[index] = x;
                break;
            }
        }
    }
    // Recurse with the rest of the list
    make_valid_list(first_section, &list[..list.len()-1]);
}

fn part1(first_section: &HashMap<i32, Vec<i32>>, second_section: Vec<Vec<i32>>) -> i32 {
    let mut p1_count = 0;
    for list in second_section {
        if check_valid_list(first_section, &list.as_slice()) {
            // Get middle element of list
            let middle_num = list[list.len() / 2];
            p1_count += middle_num;
        }
    }
    return p1_count;
}

pub fn fcn(contents: &String) {
    let (first_section_nums, second_section) = parse_contents(contents);
    let p1 = part1(&first_section_nums, second_section);
    println!("Part 1: {}", p1);
}