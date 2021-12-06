use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr;

fn main() {
    println!("Hello, justin!");

    let num_file = File::open("src/bingoNumbers.txt").unwrap();
    let num_reader = BufReader::new(num_file);
    let mut num_data : Vec<i32> = Vec::new();

    // parse the bingo numbers into vector
    for line in num_reader.lines() {
        num_data = line.unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
    }
    println!("{:?}", num_data);

    // a bingo board is a vector of vectors
    let board_file = File::open("src/bingoBoards.txt").unwrap();
    let board_reader = BufReader::new(board_file);
    let mut boards_data: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut current_board: Vec<Vec<i32>> = Vec::new();

    for line in board_reader.lines() {
        let data = line.unwrap();
        if !data.is_empty() {
            // line not empty, is a bingo board line
            let bingo_line: Vec<i32> = data.split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
            current_board.push(bingo_line);
        } else {
            // line is empty, we have completed a bingo board
            boards_data.push(current_board.clone());
            current_board = Vec::new();
        }
    }
    // tbh there's still info in current_board after the last line
    // too lazy to figure out why, it's late.
    boards_data.push(current_board);

    println!("Boards data: {:?}", boards_data);
    part1(&num_data, &boards_data);
    
    // part 2
    boards_data.reverse();
    part1(&num_data, &boards_data);
}

fn part1(num_data: &Vec<i32>, boards_data: &Vec<Vec<Vec<i32>>>) {
    let mut marked_boards = boards_data.clone();
    let mut board_win_count = 0;
    let mut last_winning_board: Vec<Vec<i32>> = Vec::new();
    let mut last_winning_num = 0;
    // mark the numbers in sequence off the bingo boards
    // for each number in num data
    // get a board. for int row and int col
    // check if that value == bingo number
    // if equal, mark off with -1 and check if bingo win occured
    // copy marked board off

    for num in num_data {
        // println!("NUM: {} ---------", num);
        let mut index = 0;
        for board in marked_boards.iter_mut() {
            for i in 0..5 { // row
                for j in 0..5 { // col
                    let val = board[i][j];
                    if val == *num {
                        // mark number
                        // check for bingo win
                        board[i][j] = -1;
                        let is_win = bingo_win_checker(&board, i, j, &num);
                        if is_win && board_win_count < boards_data.len() {
                            // for part 2
                            marked_boards[index] = vec![vec![0]]
                            // board_win_count += 1;
                            // last_winning_num = num.clone();
                            // last_winning_board = board.clone();
                        }
                    }
                }
            }
            index += 1;
            println!("{}, {:?}", index, board);
        }
    }

    // this is for part 2
    println!("------ part 2:");
    board_sum(&last_winning_board, &last_winning_num);
}

fn bingo_win_checker(board: &Vec<Vec<i32>>, row: usize, col: usize, num: &i32) -> bool {
    // a win is any row or column of all -1 

    // row = 2
    // col = 3
    // 52 17 62 49 76
    // 8 78 93 -1 12
    // 9 40 -1 -1* 94
    // 45  2 81 44 63
    // 73 18 48 11 90

    // check if horizontal win
    let horizontal = &board[row];
    let mut horizontal_win = true;
    for i in 0..5 {
        let val = horizontal[i];
        if val != -1 {
            horizontal_win = false;
            break;
        }
    }

    // check if vertical win
    // for each row, check if col is the same
    let mut vertical_win = true;
    for line in board {
        let val = line[col];
        if val != -1 {
            vertical_win = false;
            break;
        }
    }

    if horizontal_win || vertical_win {
        println!("WIN with board {:?}", board);
        board_sum(board, num);
        
        return true;
    }
    return false;
}

fn board_sum(board: &Vec<Vec<i32>>, num: &i32) {
    // sum all the unmarked numbers on the board
    // i'm sure there's a smarter way to sum this
    // but i'm so tired. i just want the star maaan
    let mut sum = 0;
        for i in 0..5 {
            for j in 0..5{
                let val = board[i][j];
                if val != -1 {
                    sum += val
                }
            }
        }
    println!("=====\n sum: {}\n num: {}\n answer: {}", sum, num, sum*num);

}