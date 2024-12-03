mod day1;
mod day2;
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
        (&day2::fcn, true),
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