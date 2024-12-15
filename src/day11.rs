// use indicatif::ProgressBar;
use std::collections::HashMap;
use rayon::prelude::*;

fn parse_contents(contents: &String) -> Vec<usize> {
    return contents.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
}

#[inline]
fn get_num_digits(num: usize) -> u8 {
    // Check if num has an even number of digits, e.g., 1234->true, 123->false
    let mut num_digits = 0;
    let mut num = num;
    while num > 0 {
        num_digits += 1;
        num /= 10;
    }
    return num_digits;
}

#[inline]
fn split_digits(num: usize, num_digits: u8) -> (usize, usize) {
    // Split the digits of num into the left and right numbers
    let split_mod = 10usize.pow(num_digits as u32/2);
    let right = num % split_mod;
    let left = num / split_mod;
    // Make sure not to reverse the digits
    return (left, right);
}

fn recursive_soln(num: usize, num_iter: u8, cache: &mut HashMap<(u8, usize), usize>) -> usize {
    if cache.contains_key(&(num_iter, num)) {
        return *cache.get(&(num_iter, num)).unwrap();
    }
    if num_iter == 0 {
        cache.insert((num_iter, num), 1);
        return 1;
    }
    if num == 0 {
        let soln = recursive_soln(1, num_iter - 1, cache);
        cache.insert((num_iter, num), soln);
        return soln;
    }
    let d = get_num_digits(num);
    if d % 2 == 0 {
        let (num1, num2) = split_digits(num, d);
        let soln = recursive_soln(num1, num_iter - 1, cache) + recursive_soln(num2, num_iter - 1, cache);
        cache.insert((num_iter, num), soln);
        return soln;
    } else {
        let soln = recursive_soln(2024*num, num_iter - 1, cache);
        cache.insert((num_iter, num), soln);
        return soln;
    }
}

fn both_parts_recursive(line: &Vec<usize>, num_iter: u8) -> usize {
    let mut cache: HashMap<(u8, usize), usize> = HashMap::new();
    return line.iter().map(|x| recursive_soln(*x, num_iter, &mut cache)).sum();
}

pub fn fcn(contents: &String) {
    let line = parse_contents(contents);
    let p1 = both_parts_recursive(&line, 25);
    println!("\nPart 1: {}", p1);
    let p2 = both_parts_recursive(&line, 75);
    println!("Part 2: {}", p2);
}