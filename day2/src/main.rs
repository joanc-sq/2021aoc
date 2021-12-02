use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("src/day2input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut data: Vec<Vec<String>> = Vec::new();

    // Parse the data from the file 
    for line in reader.lines() {
        let val = line.unwrap();
        // formats as ["instruction", "number"]
        let split: Vec<&str> = val.split_whitespace().collect();
        println!("split: {:?}", split);

        data.push(vec![split[0].to_string(), split[1].to_string()]);
    }

    part1(&data);
    part2(&data);
}

fn part1(input: &Vec<Vec<String>>) {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for instruction in input {
        let direction: &str = instruction[0].as_ref();
        let value: &str = instruction[1].as_ref();

        match direction {
            "forward" => horizontal_pos += value.parse::<i32>().unwrap(),
            "down" => vertical_pos += value.parse::<i32>().unwrap(),
            "up" => vertical_pos -= value.parse::<i32>().unwrap(),
            _ => panic!("Unknown instructions {}", direction)
        }
    }
    println!("Horizontal: {}, Vertical: {}, answer: {}", horizontal_pos, vertical_pos, horizontal_pos*vertical_pos);
}

fn part2(input: &Vec<Vec<String>>) {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    let mut aim = 0;

    for instruction in input {
        let direction: &str = instruction[0].as_ref();
        let value: &str = instruction[1].as_ref();

        let x = value.parse::<i32>().unwrap();
        match direction {
            "forward" => {
                horizontal_pos += x;
                vertical_pos += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("Unknown instructions {}", direction)
        }
    }
    println!("Horizontal: {}, Vertical: {}, answer: {}", horizontal_pos, vertical_pos, horizontal_pos*vertical_pos);
}
