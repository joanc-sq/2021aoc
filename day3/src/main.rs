use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("src/day3input.txt").unwrap();
    let reader = BufReader::new(file);
    
    // Parse the data as an 2d matrix
    // so you can iterate over columns
    let mut data: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let mut binary_vector: Vec<u32> = Vec::new();
        for character in line.unwrap().chars(){
            binary_vector.push(character.to_digit(10).unwrap())
        }
        data.push(binary_vector)
    }

    let count_vectors = part1(&data);
    part2(&data)
}

fn part1(input: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let binary_len = input[0].len();
    let mut one_count_vec = vec![0; binary_len];
    let mut zero_count_vec = vec![0; binary_len];

    for i in 0..input.len() {
        let row = &input[i];

        // e.g. [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1]
        for j in 0..binary_len {
            let num = row[j];
            if num == 0 {
                zero_count_vec[j] += 1;
            } else {
                one_count_vec[j] += 1;
            }
        }
    }
    println!("One count: {:?}", one_count_vec);
    println!("Zero count: {:?}", zero_count_vec);

    // gamma rate determined by most common value of bits
    // epsilon rate determined by lead common value of bits
    let mut gamma: String = "".to_owned();
    let mut epsilon: String = "".to_owned();

    for i in 0..binary_len {
        let zero_count = zero_count_vec[i];
        let one_count = one_count_vec[i];

        if zero_count > one_count {
            gamma.push_str("0");
            epsilon.push_str("1")
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }
    let gamma_int = isize::from_str_radix(gamma.as_ref(), 2).unwrap();
    let epsilon_int = isize::from_str_radix(epsilon.as_ref(), 2).unwrap();

    println!("Gamma: {}, epsilon: {}", gamma, epsilon);
    println!("int Gamma: {}, epsilon: {}, answer: {}", gamma_int, epsilon_int, gamma_int*epsilon_int);

    return (zero_count_vec, one_count_vec);
}

fn part2(input: &Vec<Vec<u32>>) {
    // Oxygen generator rating
    // find the most common value in the current bit position
    // and only keep numbers with that bit in that position 
    // if they are equally common, keep values with a 1 in the position

    // CO2 scrubber rating is inverse, keep only numbers with less common value
    // in that position

    // keep track of the index that matches criteria
    // let input_size: u32 = input.len() as u32;
    // let mut oxygen_index: Vec<u32> =  (0..input_size).collect::<Vec<u32>>();
    // let mut co2_index: Vec<u32> = (0..input_size).collect::<Vec<u32>>();

    let mut oxygen_rating = input.clone(); // start with all possible numbers

    let mut index = 0;
    while index < 12 && oxygen_rating.len() > 1 {
        let count_vectors = part1(&oxygen_rating);
        let zero_count_vec = count_vectors.0;
        let one_count_vec = count_vectors.1;

        let mut passing_oxygen_values: Vec<Vec<u32>> = Vec::new();

        let zero_greater = zero_count_vec[index] > one_count_vec[index];
        println!("Zero more popular at index {}: {}", index, zero_greater);

        for i in 0..oxygen_rating.len() {
            let row = &oxygen_rating[i]; // e.g. [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1]
            if zero_count_vec[index] > one_count_vec[index] {
                // 0 is more common, keep numbers with 0 at index
                if row[index] == 0 {
                    passing_oxygen_values.push(row.clone());
                }
            } else {
                // 1 is more common, keep numbers with 1 at index
                if row[index] == 1 {
                    passing_oxygen_values.push(row.clone());
                }
            }
        }
        println!("-- passing oxygen val: {:?}", passing_oxygen_values);
        oxygen_rating = passing_oxygen_values;
        index += 1;
    }

    let mut co2_rating = input.clone(); // start with all possible numbers
    index = 0;
    while index < 12 && co2_rating.len() > 1 {
        let count_vectors = part1(&co2_rating);
        let zero_count_vec = count_vectors.0;
        let one_count_vec = count_vectors.1;

        let mut passing_co2_values: Vec<Vec<u32>> = Vec::new();
        let zero_greater = zero_count_vec[index] > one_count_vec[index];
        println!("Zero more popular at index {}: {}", index, zero_greater);
        for i in 0..co2_rating.len() {
            let row = &co2_rating[i]; // e.g. [0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1]
            if zero_count_vec[index] > one_count_vec[index] {
                // 0 is more common, keep numbers with 1 at index
                if row[index] == 1 {
                    passing_co2_values.push(row.clone());
                }
            } else {
                // 1 is more common, keep numbers with 0 at index
                if row[index] == 0 {
                    passing_co2_values.push(row.clone());
                }
            }
        }
        println!("-- passing co2 val: {:?}", passing_co2_values);
        co2_rating = passing_co2_values;
        index += 1;
    }

    let oxygen_str: String = oxygen_rating[0].iter().map( |&id| id.to_string()).collect(); 
    let co2_str: String = co2_rating[0].iter().map( |&id| id.to_string()).collect(); 
    println!("Oxygen rating: {:?}", oxygen_str);
    println!("Co2 rating: {:?}", co2_str);
    let oxygen_int = isize::from_str_radix(oxygen_str.as_ref(), 2).unwrap();
    let co2_int = isize::from_str_radix(co2_str.as_ref(), 2).unwrap();
    println!("O2: {}, Co2: {}, answer: {}", oxygen_int, co2_int, oxygen_int*co2_int);
}