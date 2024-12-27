mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
use std::fs;

use itertools::enumerate;

// Function to read the input file for a given day
fn setup(day: usize) -> String {
    let filename = format!("data/day{}.txt", day);
    let contents = fs::read_to_string(filename).expect("Could not read file");
    return contents;
}

fn main() {
    // List of days to run as a tuple of the function and a boolean to evaluate it
    let days: Vec<(&dyn Fn(&String) -> (), bool)> = vec![
        (&day1::fcn, false),
        (&day2::fcn, false),
        (&day3::fcn, false),
        (&day4::fcn, false),
        (&day5::fcn, false),
        (&day6::fcn, false),
        (&day7::fcn, false),
        (&day8::fcn, false),
        (&day9::fcn, false),
        (&day10::fcn, false),
        (&day11::fcn, false),
        (&day12::fcn, false),
        (&day13::fcn, false),
        (&day14::fcn, false),
        (&day15::fcn, false),
        (&day16::fcn, false),
        (&day17::fcn, false),
        (&day18::fcn, true),
    ];

    // Iterate over the days and run the function if the boolean is true
    for (idx, (f, show)) in enumerate(&days) {
        let day = idx + 1;
        if *show {
            println!("\nDay {day} result:\n");
            let contents_j = setup(day);
            (f)(&contents_j);
        }
    }
}
