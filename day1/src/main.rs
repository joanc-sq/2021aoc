use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("hello world");

    let file = File::open("src/day1input.txt").unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut increased_count = 0;
    let mut previous_depth = 9999999;

    // part 1, find increases
    // Q: Had to use numbers.clone because under the hood, 
    // rust implicity makes 
    // let mut iterator = numbers.into_iter() which changes ownership
    // of numbers. Is calling clone the standard way to get around
    // iterating over vectors multiple times?
    for current_depth in numbers.clone() {
        // count the number of times a depth measurement increases
        if current_depth > previous_depth {
            increased_count += 1;
        }

        previous_depth = current_depth;
    }

    println!("Increased {} times", increased_count);

    // part 2, find increases in 3 measurement windows
    let mut index = 0;
    let mut prev_window_val = 9999999;
    let mut window_increases = 0;
    let window = 3;

    println!("Length of numbers: {}", numbers.len());

    while index <= numbers.len() - window {
        let current_window_val = numbers[index] 
        + numbers[index + 1]
        + numbers[index + 2];

        if current_window_val > prev_window_val {
            window_increases += 1;
        }

        prev_window_val = current_window_val;
        index += 1
    }
    
    println!("Increased {} windows", window_increases);
}
