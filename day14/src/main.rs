use std::{collections::HashMap, fs, process::Command};

use std::io::{stdin, stdout, Read, Write};

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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
const REPS: i32 = 1000000000;

fn get_2d_vector_from_multiline_string(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_day14a(file_path: &str) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    solve_day14a_from_matrix(&matrix)
}

fn solve_day14a_from_matrix(matrix: &Vec<Vec<char>>) -> usize {
    let mut all_sum = 0;
    let col_size = matrix.len();
    for col in 0..matrix[0].len() {
        let mut next_weight = col_size; // Starts as <len>, gets reduced for round rocks by 1, and cube rocks by location
        let mut col_sum = 0;
        for row in 0..matrix.len() {
            if matrix[row][col] == ROUND_ROCK {
                col_sum += next_weight;
                next_weight -= 1;
            } else if matrix[row][col] == CUBE_ROCK {
                next_weight = col_size - (row + 1);
            }
        }
        all_sum += col_sum;
    }
    all_sum
}

fn get_coords(internal: usize, external: usize, direction: &Direction) -> (usize, usize) {
    if *direction == Direction::Up || *direction == Direction::Down {
        return (internal, external);
    }
    return (external, internal);
}

fn tilt_board(matrix: &mut Vec<Vec<char>>, direction: &Direction) {
    let internal_iter_size;
    let external_iter_size;
    if *direction == Direction::Up || *direction == Direction::Down {
        internal_iter_size = matrix.len();
        external_iter_size = matrix[0].len();
    } else {
        internal_iter_size = matrix[0].len();
        external_iter_size = matrix.len();
    }
    let external_range = 0..external_iter_size;

    // When scanning internal iter, scan it bottom-up if everything is pushed up/right
    let internal_range: Vec<usize>;
    let internal_base;
    if *direction == Direction::Up || *direction == Direction::Left {
        internal_range = (0..=internal_iter_size - 1).collect::<Vec<usize>>();
        internal_base = 0;
    } else {
        internal_range = (0..=internal_iter_size - 1).rev().collect::<Vec<usize>>();
        internal_base = internal_iter_size - 1;
    }

    for external in external_range {
        let mut current_internal_base = internal_base;
        for internal in internal_range.clone() {
            let (row, col) = get_coords(internal, external, &direction);
            if matrix[row][col] == ROUND_ROCK {
                let (new_row, new_col) = get_coords(current_internal_base, external, &direction);
                matrix[row][col] = EMPTY;
                matrix[new_row][new_col] = ROUND_ROCK;

                // Update internal base
                if *direction == Direction::Up || *direction == Direction::Left {
                    current_internal_base += 1;
                } else {
                    if current_internal_base > 0 {
                        current_internal_base -= 1;
                    }
                }
            } else if matrix[row][col] == CUBE_ROCK {
                if *direction == Direction::Up || *direction == Direction::Left {
                    current_internal_base = internal + 1;
                } else if internal > 0 {
                    current_internal_base = internal - 1;
                }
            }
        }
    }
}

fn calculate_current_north_load(matrix: &Vec<Vec<char>>) -> usize {
    let mut north_load = 0;
    for col in 0..matrix.len() {
        let mut col_north_load = 0;
        for row in 0..matrix[0].len() {
            if matrix[row][col] == ROUND_ROCK {
                col_north_load += matrix.len() - row;
            }
        }
        north_load += col_north_load;
    }
    north_load
}

fn solve_day14b(file_path: &str) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let mut matrix = get_2d_vector_from_multiline_string(&content);
    let mut all_sum = 0;
    let mut matrices_found = HashMap::new();
    let mut matrices_by_steps = HashMap::new();
    for i in 0..REPS {
        println!("Starting tilting cycle #{}", i);
        tilt_board(&mut matrix, &Direction::Up);
        tilt_board(&mut matrix, &Direction::Left);
        tilt_board(&mut matrix, &Direction::Down);
        tilt_board(&mut matrix, &Direction::Right);
        all_sum = calculate_current_north_load(&matrix);

        // matrix to string
        let mut string_matrix = String::new();
        for row in matrix.iter() {
            let row_string: String = row.iter().collect();
            string_matrix.push_str(&row_string);
        }
        // (BASE - 1) + ((REPS - BASE) % CYCLE_LENGTH)
        match matrices_found.get(&string_matrix) {
            Some(step_first_seen) => {
                println!("I HAVE SEEN THIS MATRIX BEFORE!!");
                let cycle_length = i - step_first_seen;
                println!("Cycle length: {}", cycle_length);
                println!("i: {}", i);
                let offset_from_this_base = (REPS - step_first_seen) % cycle_length;
                let last_step = step_first_seen + offset_from_this_base - 1;
                println!("Last step = {}", last_step);
                let last_matrix = matrices_by_steps.get(&last_step).unwrap();
                println!("Last Matrix is in iter {last_step}");
                return calculate_current_north_load(&last_matrix);
            }
            None => {
                matrices_found.insert(string_matrix, i);
                matrices_by_steps.insert(i, matrix.clone());
            }
        }
    }
    println!("All sum: {}", all_sum);
    all_sum
}

fn main() {
    let result = solve_day14b(INPUT_FILE_PATH);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day14a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 136);
    }

    #[test]
    fn check_input_p1() {
        let result = solve_day14a(INPUT_FILE_PATH);
        assert_eq!(result, 108918);
    }

    #[test]
    fn check_example1_p2() {
        let result = solve_day14b(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 64);
    }

    #[test]
    fn check_input_p2() {
        let result = solve_day14b(INPUT_FILE_PATH);
        assert_eq!(result, 100310);
    }
}
