use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world! 🕯️");
    let file = File::open("src/day7input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut crab_data: Vec<i32> = Vec::new();

    for line in reader.lines() {
        crab_data = line
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    }

    println!("{:?} \n size: {}", crab_data, crab_data.len());
    part1(&crab_data);
    part2(&crab_data);
}

#[derive(Debug)]
struct CrabFuel {
    index: usize,
    fuel: i32,
}

fn part1(crab_data: &Vec<i32>) {
    let crab_data_len = crab_data.len();
    let mut fuel_consumption: Vec<CrabFuel> = Vec::new();

    for i in 0..crab_data_len {
        let mut curr_fuel = 0;
        for j in 0..crab_data_len {
            if i != j {
                let i_fuel = crab_data[i];
                let j_fuel = crab_data[j];
                curr_fuel += (j_fuel-i_fuel).abs();
            }
        }
        let curr_crab_fuel = CrabFuel {
            index: i,
            fuel: curr_fuel,
        };

        fuel_consumption.push(curr_crab_fuel);
    }

    println!("{:?}", fuel_consumption);
    
    fuel_consumption.sort_by(|f1, f2| (f1.fuel).cmp(&f2.fuel));

    println!("-----\n{:?}", fuel_consumption);
    println!("part 1 answer: {:?}", fuel_consumption[0]);
}

fn part2(crab_data: &Vec<i32>) {
    let crab_data_len = crab_data.len();
    let mut fuel_consumption: Vec<CrabFuel> = Vec::new();

    for i in 0..crab_data_len {
        let mut curr_fuel = 0;
        for j in 0..crab_data_len {
            let j_fuel = crab_data[j];
            let n = (j_fuel-i as i32).abs()+1;
            let k = 2;
            let binomial: i32 = num_integer::binomial(n, k);
            // println!("Move from {} to {}: {} fuel", i_fuel, j_fuel, binomial);
            curr_fuel += binomial;
        }
        let curr_crab_fuel = CrabFuel {
            index: i,
            fuel: curr_fuel,
        };

        fuel_consumption.push(curr_crab_fuel);
    }

    println!("{:?}", fuel_consumption);
    
    fuel_consumption.sort_by(|f1, f2| (f1.fuel).cmp(&f2.fuel));

    println!("-----\n{:?}", fuel_consumption);
    println!("part 2 answer: {:?}", fuel_consumption[0]);
}
