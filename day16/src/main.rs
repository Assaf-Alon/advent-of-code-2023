use std::collections::VecDeque;
use std::{fs, process::Command};

use std::io::{stdin, stdout, Read, Write};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;
const NOWHERE: usize = 4;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn pprint_3d_arr(arr: Vec<Vec<Vec<bool>>>) {
    for row in 0..arr.len() {
        for col in 0..arr[0].len() {
            if arr[row][col][0] || arr[row][col][1] || arr[row][col][2] || arr[row][col][3] {
                print!("# ");
            } else {
                print!(". ");
            }
        }
    }
}

fn get_2d_vector_from_multiline_string(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn get_new_coords(row: usize, col: usize, direction: usize) -> (usize, usize) {
    if direction == UP && row > 0 {
        return (row - 1, col);
    } else if direction == DOWN {
        return (row + 1, col);
    } else if direction == LEFT && col > 0 {
        return (row, col - 1);
    } else if direction == RIGHT {
        return (row, col + 1);
    } else {
        return (usize::MAX, usize::MAX);
    }
}

fn perform_step(
    curr_direction: usize,
    row: usize,
    col: usize,
    matrix: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
) -> VecDeque<(usize, usize, usize)> {
    // Out of bounds
    if (curr_direction == RIGHT && col > matrix[0].len() - 1)
        || (curr_direction == DOWN && row > matrix.len() - 1)
    {
        return VecDeque::new();
    }
    if visited[row][col][curr_direction] {
        return VecDeque::new();
    }

    visited[row][col][curr_direction] = true;

    let mut new_rays: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut new_direction: usize = NOWHERE;

    match matrix[row][col] {
        '.' => new_direction = curr_direction,
        '\\' => {
            new_direction = match curr_direction {
                RIGHT => DOWN,
                LEFT => UP,
                UP => LEFT,
                DOWN => RIGHT,
                _ => NOWHERE,
            };
        }
        '/' => {
            new_direction = match curr_direction {
                RIGHT => UP,
                LEFT => DOWN,
                UP => RIGHT,
                DOWN => LEFT,
                _ => NOWHERE,
            };
        }
        '-' => {
            if curr_direction == LEFT || curr_direction == RIGHT {
                new_direction = curr_direction;
            } else {
                if col > 0 {
                    new_rays.push_back((LEFT, row, col - 1));
                }
                new_rays.push_back((RIGHT, row, col + 1));
            }
        }
        '|' => {
            if curr_direction == UP || curr_direction == DOWN {
                new_direction = curr_direction;
            } else {
                if row > 0 {
                    new_rays.push_back((UP, row - 1, col));
                }
                new_rays.push_back((DOWN, row + 1, col));
            }
        }
        _ => new_direction = NOWHERE,
    }

    if new_direction != NOWHERE {
        let (new_row, new_col) = get_new_coords(row, col, new_direction);
        if new_row != usize::MAX {
            new_rays.push_back((new_direction, new_row, new_col));
        }
    }

    new_rays
}

fn solve_day16a(file_path: &str, start_row: usize, start_col: usize, direction: usize) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut visited = vec![vec![vec![false; 4]; cols]; rows];
    let mut active_rays: VecDeque<(usize, usize, usize)> = VecDeque::new();
    active_rays.push_back((direction, start_row, start_col));
    while active_rays.len() > 0 {
        let current_ray = active_rays.pop_back().unwrap();
        let new_rays = perform_step(
            current_ray.0,
            current_ray.1,
            current_ray.2,
            &matrix,
            &mut visited,
        );
        active_rays.extend(new_rays);
    }

    let count_true = visited
        .iter()
        .flatten()
        .filter(|&directions| directions.clone().contains(&true))
        .count();
    count_true
}

fn solve_day16b(file_path: &str) -> usize {
    let mut best_energy = 0;
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows {
        best_energy = usize::max(best_energy, solve_day16a(file_path, row, 0, RIGHT));
        best_energy = usize::max(best_energy, solve_day16a(file_path, row, cols - 1, LEFT));
    }

    for col in 0..cols {
        best_energy = usize::max(best_energy, solve_day16a(file_path, 0, col, DOWN));
        best_energy = usize::max(best_energy, solve_day16a(file_path, rows - 1, col, UP));
    }
    println!("Best energy = {best_energy}");
    best_energy
}

fn main() {
    let result = solve_day16b(INPUT_FILE_PATH);
    println!("Result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day16a(EXAMPLE1_FILE_PATH, 0, 0, RIGHT);
        assert_eq!(result, 46);
    }

    #[test]
    fn check_example1_p2() {
        let result = solve_day16b(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 51);
    }
}
