// PART 2 IS BUGGED >.<
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
const STEPS_P2: usize = 327;

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

fn get_starting_coordinates(content: &str, is_padded: bool) -> (usize, usize) {
    let offset = if is_padded { 1 } else { 0 };
    for (row_index, line) in content.lines().enumerate() {
        let col_index = line.find(STARTING_SYMBOL);
        if col_index.is_some() {
            return (row_index + offset, col_index.unwrap() + offset);
        }
    }
    panic!("Failed to find starting symbol");
}

fn is_empty(c: char) -> bool {
    c == EMPTY_SYMBOL || c == STARTING_SYMBOL
}

fn solve_day21a(file_path: &str, steps: usize, start_row: usize, start_col: usize) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = pad_2d_vector(get_2d_vector_from_multiline_string(&content));
    let mut current_locations = HashSet::new();

    current_locations.insert((start_row, start_col));
    for _ in 0..steps {
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
    // for row in 0..matrix.len() {
    //     for col in 0..matrix[0].len() {
    //         if matrix[row][col] == STARTING_SYMBOL && current_locations.contains(&(row, col)) {
    //             print!("$ ");
    //         } else if matrix[row][col] == STARTING_SYMBOL {
    //             print!("{} ", STARTING_SYMBOL);
    //         } else if matrix[row][col] == ROCK_SYMBOL {
    //             print!("{} ", ROCK_SYMBOL);
    //         } else if current_locations.contains(&(row, col)) {
    //             print!("O ");
    //         } else {
    //             print!("{} ", EMPTY_SYMBOL);
    //         }
    //     }
    //     println!("");
    // }

    current_locations.len()
}

fn get_final_board_info(file_path: &str) -> (usize, usize, usize) {
    // <steps until board is full>, <full locations odd>, <full locations even>
    println!("Getting final board info...");
    // let mut double_prev = 0;
    // let mut prev = 0;
    // let mut result = 1;
    // let mut steps = 0;
    // let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    // let (start_row, start_col) = get_starting_coordinates(&content, true);
    // while double_prev != result {
    //     double_prev = prev;
    //     prev = result;
    //     result = solve_day21a(file_path, steps, start_row, start_col);
    //     steps += 1;
    // }
    // println!("{double_prev}, {prev}, {result}");
    // let full_locations_odd;
    // let full_locations_even;
    // if steps % 2 == 0 {
    //     full_locations_even = result;
    //     full_locations_odd = prev;
    // } else {
    //     full_locations_even = prev;
    //     full_locations_odd = result;
    // }
    // (steps, full_locations_odd, full_locations_even)
    (130, 7541, 7553)
}

fn solve_day21b(file_path: &str) {
    // MAINLY MATH *~*
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let (steps, full_locations_odd, full_locations_even) = get_final_board_info(file_path);
    let rows = content.lines().count();
    let cols = content.lines().nth(0).unwrap().len();
    assert_eq!(rows, cols);
    let steps_to_other_corner = rows + ((rows - 1) / 2);

    // Full boards
    let amount_full_boards_main_axis;
    if STEPS_P2 < rows {
        amount_full_boards_main_axis = 0;
    } else {
        amount_full_boards_main_axis = 1 + 4 * ((STEPS_P2 - rows + 1) / rows);
    }

    let amount_full_boards_projected_axis;
    if STEPS_P2 < rows + steps_to_other_corner {
        amount_full_boards_projected_axis = 0;
    } else {
        let n = (STEPS_P2 - steps_to_other_corner) / rows;
        amount_full_boards_projected_axis = 4 * (n * n + n) / 2; // Arithmetic progression formula
    }

    let amount_full_boards = amount_full_boards_main_axis + amount_full_boards_projected_axis;

    // How much steps will remain on the final board when
    // advancing on the main axis (all right / all left / ...)
    let initial_offset = (rows - 1) / 2;
    let main_axis_remainder = (STEPS_P2 + initial_offset) % rows;
    let projected_axis_remainder = (STEPS_P2 + rows - 1) % (rows);

    let amount_full_boards_even_state;
    if STEPS_P2 % 2 == 0 {
        amount_full_boards_even_state = (amount_full_boards + 1) / 2;
    } else {
        amount_full_boards_even_state = amount_full_boards / 2;
    }
    let amount_full_boards_odd_state = amount_full_boards - amount_full_boards_even_state;

    let mut total_possible_locations = 0;
    total_possible_locations += amount_full_boards_even_state * full_locations_even;
    total_possible_locations += amount_full_boards_odd_state * full_locations_odd;

    let leftover_right = solve_day21a(file_path, main_axis_remainder, (rows + 1) / 2, cols);
    let leftover_left = solve_day21a(file_path, main_axis_remainder, (rows + 1) / 2, 1);
    let leftover_up = solve_day21a(file_path, main_axis_remainder, rows, (cols + 1) / 2);
    let leftover_down = solve_day21a(file_path, main_axis_remainder, 1, (cols + 1) / 2);

    total_possible_locations += leftover_down + leftover_left + leftover_up + leftover_right;

    let n = (STEPS_P2 - 1) / rows;
    let leftover_bottomright = solve_day21a(file_path, projected_axis_remainder, 1, 1);
    let leftover_bottomleft = solve_day21a(file_path, projected_axis_remainder, 1, cols);
    let leftover_upperleft = solve_day21a(file_path, projected_axis_remainder, rows, cols);
    let leftover_upperright = solve_day21a(file_path, projected_axis_remainder, rows, 1);

    total_possible_locations +=
        (leftover_bottomright + leftover_bottomleft + leftover_upperleft + leftover_upperright) * n;

    println!("Steps, Odd, Even:           {steps}, {full_locations_odd}, {full_locations_even}");
    println!("Amount full boards:         {amount_full_boards}");
    println!(" > Boards Main / Proj:      {amount_full_boards_main_axis}, {amount_full_boards_projected_axis}");
    println!("Main Axis Remainder:        {main_axis_remainder}");
    println!("Projected axis remainder:   {projected_axis_remainder}");
    println!("Amount full odds:           {amount_full_boards_odd_state}");
    println!("Amount full evens:          {amount_full_boards_even_state}");
    println!("Leftovers U/D/L/R:          {leftover_up}, {leftover_down}, {leftover_left}, {leftover_right}");
    println!(
        "Leftovers n:UL/UR/DR/DL:    {n}: {leftover_upperleft}, {leftover_upperright}, {leftover_bottomright}, {leftover_bottomleft}"
    );
    println!("Total possible locations:   {total_possible_locations}");
}

// fn solve_day21b(file_path: &str) -> usize {
//     let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
//     let matrix = get_2d_vector_from_multiline_string(&content);
//     let mut current_locations = HashMap::new();
//     let (start_row, start_col) = get_starting_coordinates(&content, false);

//         current_locations = new_locations;
//     }

//     current_locations.len()
// }

fn main() {
    // let file = INPUT_FILE_PATH;
    // let content: String = fs::read_to_string(file).expect("Failed to read file content :/");
    // let (start_row, start_col) = get_starting_coordinates(&content, true);
    // let result = solve_day21a(file, STEPS_P1, start_row, start_col);
    // println!("Result = {}, expected 3733", result);

    let file = INPUT_FILE_PATH;
    let content: String = fs::read_to_string(file).expect("Failed to read file content :/");
    let (start_row, start_col) = get_starting_coordinates(&content, true);
    let result = solve_day21b(file);
    // println!("Result = {}, expected 3733", result);
}

// Time to fill from center: 130 steps

// Time to fill from side: 195

// TOO LOW:  617729380780137
// Actual:   617729401414635
// TOO HIGH: 617730151134729
