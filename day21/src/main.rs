use std::{
    collections::{HashMap, HashSet},
    fs,
};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const STARTING_SYMBOL: char = 'S';
const EMPTY_SYMBOL: char = '.';
const ROCK_SYMBOL: char = '#';
const STEPS_P1: usize = 64;
const STEPS_P2: usize = 26501365;

fn get_2d_vector_from_multiline_string(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn pad_2d_vector(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut padded_matrix = vec![vec![ROCK_SYMBOL; cols + 2]; rows + 2];

    for i in 0..rows {
        for j in 0..cols {
            padded_matrix[i + 1][j + 1] = matrix[i][j];
        }
    }

    padded_matrix
}

fn get_starting_coordinates(content: &str) -> (usize, usize) {
    for (row_index, line) in content.lines().enumerate() {
        let col_index = line.find(STARTING_SYMBOL);
        if col_index.is_some() {
            return (row_index + 1, col_index.unwrap() + 1);
        }
    }
    panic!("Failed to find starting symbol");
}

fn is_empty(c: char) -> bool {
    c == EMPTY_SYMBOL || c == STARTING_SYMBOL
}

fn solve_day21a(file_path: &str) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = pad_2d_vector(get_2d_vector_from_multiline_string(&content));
    let mut current_locations = HashSet::new();
    let (start_row, start_col) = get_starting_coordinates(&content);

    current_locations.insert((start_row, start_col));
    for _ in 0..STEPS_P1 {
        let mut new_locations: HashSet<(usize, usize)> = HashSet::new();
        for (row, col) in current_locations.iter() {
            if is_empty(matrix[row + 1][*col]) {
                new_locations.insert((row + 1, *col));
            }
            if is_empty(matrix[row - 1][*col]) {
                new_locations.insert((row - 1, *col));
            }
            if is_empty(matrix[*row][col + 1]) {
                new_locations.insert((*row, (col + 1)));
            }
            if is_empty(matrix[*row][col - 1]) {
                new_locations.insert((*row, (col - 1)));
            }
        }
        current_locations = new_locations;
    }

    // Print
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == STARTING_SYMBOL && current_locations.contains(&(row, col)) {
                print!("$ ");
            } else if matrix[row][col] == STARTING_SYMBOL {
                print!("{} ", STARTING_SYMBOL);
            } else if matrix[row][col] == ROCK_SYMBOL {
                print!("{} ", ROCK_SYMBOL);
            } else if current_locations.contains(&(row, col)) {
                print!("O ");
            } else {
                print!("{} ", EMPTY_SYMBOL);
            }
        }
        println!("");
    }

    current_locations.len()
}

fn main() {
    let result = solve_day21a(INPUT_FILE_PATH);
    println!("Result = {}", result);
}
