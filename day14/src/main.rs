use std::{
    collections::{HashMap, VecDeque},
    fs,
    process::Command,
};

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const EMPTY: char = '.';
const CUBE_ROCK: char = '#';
const ROUND_ROCK: char = 'O';

fn get_2d_vector_from_multiline_string(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_day14(file_path: &str, is_part_two: bool) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    let mut all_sum = 0;
    let col_size = matrix.len();
    println!("col size == {col_size}");
    for col in 0..matrix[0].len() {
        let mut next_weight = col_size; // Starts as <len>, gets reduced for round rocks by 1, and cube rocks by location
        let mut col_sum = 0;
        for row in 0..matrix.len() {
            if matrix[row][col] == ROUND_ROCK {
                println!(" > Adding {next_weight} to sum");
                col_sum += next_weight;
                next_weight -= 1;
            } else if matrix[row][col] == CUBE_ROCK {
                println!(" > Found cube rock :O");
                next_weight = col_size - (row + 1);
            }
        }
        println!("In col {col} found {col_sum}");
        all_sum += col_sum;
    }
    all_sum
}

fn main() {
    let result = solve_day14(INPUT_FILE_PATH, false);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day14(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 136);
    }
}
