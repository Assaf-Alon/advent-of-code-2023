use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const EXAMPLE3_FILE_PATH: &str = "example3.txt";
const EXAMPLE4_FILE_PATH: &str = "example4.txt";
const EXAMPLE5_FILE_PATH: &str = "example5.txt";
const EXAMPLE6_FILE_PATH: &str = "example6.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const STARTING_SYMBOL: &str = "S";
const NO_PIPE_SYMBOL: char = '.';

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn get_2d_vector_from_multiline_string(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn pad_2d_vector(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut padded_matrix = vec![vec![NO_PIPE_SYMBOL; cols + 2]; rows + 2];

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
        println!("{}", line);
        if col_index.is_some() {
            return (row_index + 1, col_index.unwrap() + 1);
        }
    }
    panic!("Failed to find starting symbol");
}

fn can_go(direction: Direction, value: char) -> bool {
    match direction {
        Direction::UP => return value == 'F' || value == '|' || value == '7',

        Direction::DOWN => return value == 'J' || value == '|' || value == 'L',

        Direction::LEFT => return value == '-' || value == 'F' || value == 'L',

        Direction::RIGHT => return value == '-' || value == 'J' || value == '7',
    }
}

fn handle_first_step(
    coords_to_visit: &mut VecDeque<(usize, usize)>,
    visited: &mut Vec<Vec<i32>>,
    matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) {
    // Determine possible directions
    let mut possible_directions = HashSet::new();
    possible_directions.insert('7');
    possible_directions.insert('J');
    possible_directions.insert('F');
    possible_directions.insert('L');
    possible_directions.insert('-');
    possible_directions.insert('|');

    if !can_go(Direction::RIGHT, matrix[row][col + 1]) {
        possible_directions.remove(&'-');
        possible_directions.remove(&'L');
        possible_directions.remove(&'F');
    }

    if !can_go(Direction::DOWN, matrix[row + 1][col]) {
        possible_directions.remove(&'|');
        possible_directions.remove(&'F');
        possible_directions.remove(&'7');
    }

    if !can_go(Direction::LEFT, matrix[row][col - 1]) {
        possible_directions.remove(&'-');
        possible_directions.remove(&'J');
        possible_directions.remove(&'7');
    }

    if !can_go(Direction::UP, matrix[row - 1][col]) {
        possible_directions.remove(&'|');
        possible_directions.remove(&'J');
        possible_directions.remove(&'L');
    }

    if possible_directions.len() > 1 {
        println!(" > Possible directions: {:?}", possible_directions);
        panic!("Directions ambiguous");
    } else if possible_directions.len() == 0 {
        panic!("No possible direction");
    }
    // Steps
    let symbol = possible_directions.iter().nth(0).unwrap();
    println!(" > Assumed S is {}", symbol);
    match symbol {
        '7' => {
            coords_to_visit.push_back((row, col - 1));
            coords_to_visit.push_back((row + 1, col));
        }
        'J' => {
            coords_to_visit.push_back((row - 1, col));
            coords_to_visit.push_back((row, col - 1));
        }
        'F' => {
            coords_to_visit.push_back((row, col + 1));
            coords_to_visit.push_back((row + 1, col));
        }
        'L' => {
            coords_to_visit.push_back((row, col + 1));
            coords_to_visit.push_back((row - 1, col));
        }
        '-' => {
            coords_to_visit.push_back((row, col - 1));
            coords_to_visit.push_back((row, col + 1));
        }
        '|' => {
            coords_to_visit.push_back((row - 1, col));
            coords_to_visit.push_back((row + 1, col));
        }
        _ => panic!("Shouldn't get here!! symbol not found"),
    }
}

fn bfs_step(
    coords_to_visit: &mut VecDeque<(usize, usize)>,
    visited: &mut Vec<Vec<i32>>,
    matrix: &Vec<Vec<char>>,
) {
    let (row, col) = coords_to_visit.pop_front().unwrap();
    println!(" > Performing BFS step, ({}, {})", row, col);
    if visited[row][col] >= 0 {
        return;
    }
    match matrix[row][col] {
        'S' => {
            if visited[row][col] >= 0 {
                return;
            }
            visited[row][col] = 0;
            handle_first_step(coords_to_visit, visited, matrix, row, col);
        }
        '7' => {
            if visited[row][col - 1] >= 0 {
                // Left --> Down
                visited[row][col] = visited[row][col - 1] + 1;
                coords_to_visit.push_back((row + 1, col));
            } else {
                // Down --> Left
                visited[row][col] = visited[row + 1][col] + 1;
                coords_to_visit.push_back((row, col - 1));
            }
        }

        'L' => {
            if visited[row - 1][col] >= 0 {
                // Up --> Right
                visited[row][col] = visited[row - 1][col] + 1;
                coords_to_visit.push_back((row, col + 1));
            } else {
                // Right --> Up
                visited[row][col] = visited[row][col + 1] + 1;
                coords_to_visit.push_back((row - 1, col));
            }
        }

        'F' => {
            if visited[row + 1][col] >= 0 {
                // Down --> Right
                visited[row][col] = visited[row + 1][col] + 1;
                coords_to_visit.push_back((row, col + 1));
            } else {
                // Right --> Down
                visited[row][col] = visited[row][col + 1] + 1;
                coords_to_visit.push_back((row + 1, col));
            }
        }

        'J' => {
            if visited[row][col - 1] >= 0 {
                // Left --> Up
                visited[row][col] = visited[row][col - 1] + 1;
                coords_to_visit.push_back((row - 1, col));
            } else {
                // Up --> Left
                visited[row][col] = visited[row - 1][col] + 1;
                coords_to_visit.push_back((row, col - 1));
            }
        }

        '-' => {
            if visited[row][col - 1] >= 0 {
                // Left --> Right
                visited[row][col] = visited[row][col - 1] + 1;
                coords_to_visit.push_back((row, col + 1));
            } else {
                // Right --> Left
                visited[row][col] = visited[row][col + 1] + 1;
                coords_to_visit.push_back((row, col - 1));
            }
        }

        '|' => {
            if visited[row - 1][col] >= 0 {
                // Up --> Down
                visited[row][col] = visited[row - 1][col] + 1;
                coords_to_visit.push_back((row + 1, col));
            } else {
                // Down --> Up
                visited[row][col] = visited[row + 1][col] + 1;
                coords_to_visit.push_back((row - 1, col));
            }
        }
        _ => println!("Shouldn't get here! matrix match"),
    }

    // if matrix[row][col] == '.' {
    //     println!("Shouldn't get here!!!!!!!!!!!!");
    //     return;
    // }
}

fn solve_day10a(file_path: &str) -> i32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = pad_2d_vector(get_2d_vector_from_multiline_string(&content));
    let rows = content.len();
    let cols = content.lines().nth(0).unwrap().len();
    let mut coords_to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    coords_to_visit.push_back(get_starting_coordinates(&content));
    let mut visited = vec![vec![-1; cols + 2]; rows + 2];
    let mut length_from_start = 0;

    while !coords_to_visit.is_empty() {
        let &front_coors = coords_to_visit.front().unwrap();
        bfs_step(&mut coords_to_visit, &mut visited, &matrix);
        length_from_start = i32::max(visited[front_coors.0][front_coors.1], length_from_start);
        println!(
            "Performed step. tovisit size = {}, max_bfs = {}",
            coords_to_visit.len(),
            length_from_start
        );
    }

    return length_from_start;
}

fn solve_day10b(file_path: &str) -> i32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = pad_2d_vector(get_2d_vector_from_multiline_string(&content));
    let rows = content.len();
    let cols = content.lines().nth(0).unwrap().len();
    let mut coords_to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    coords_to_visit.push_back(get_starting_coordinates(&content));
    let mut visited = vec![vec![-1; cols + 2]; rows + 2];
    let mut length_from_start = 0;

    while !coords_to_visit.is_empty() {
        let &front_coors = coords_to_visit.front().unwrap();
        bfs_step(&mut coords_to_visit, &mut visited, &matrix);
        length_from_start = i32::max(visited[front_coors.0][front_coors.1], length_from_start);
        println!(
            "Performed step. tovisit size = {}, max_bfs = {}",
            coords_to_visit.len(),
            length_from_start
        );
    }

    let mut stuck_tiles = 0;
    for row in 0..rows {
        let mut total_ups_to_line = 0;
        let mut total_downs_to_line = 0;
        for col in 0..cols {
            if visited[row][col] >= 0 {
                if matrix[row][col] == 'J' || matrix[row][col] == 'L' {
                    total_downs_to_line += 1;
                } else if matrix[row][col] == '7' || matrix[row][col] == 'F' {
                    total_ups_to_line += 1;
                } else if matrix[row][col] == '|' {
                    total_downs_to_line += 1;
                    total_ups_to_line += 1;
                }
            }
        }

        let mut ups_so_far = 0;
        let mut downs_so_far = 0;
        for col in 0..cols {
            if visited[row][col] < 0 && downs_so_far % 2 == 1 && ups_so_far % 2 == 1 {
                stuck_tiles += 1;
                println!("Stuck tile: ({}, {})", row, col);
            }
            if visited[row][col] >= 0 {
                if matrix[row][col] == 'J' || matrix[row][col] == 'L' {
                    downs_so_far += 1;
                } else if matrix[row][col] == '7' || matrix[row][col] == 'F' {
                    ups_so_far += 1;
                } else if matrix[row][col] == '|' {
                    downs_so_far += 1;
                    ups_so_far += 1;
                }
            }
        }
    }
    println!("Total stuck tiles: {}", stuck_tiles);
    return stuck_tiles;
}

fn main() {
    let result = solve_day10b(INPUT_FILE_PATH);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day10a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 4);
    }
    #[test]
    fn check_example2() {
        let result = solve_day10a(EXAMPLE2_FILE_PATH);
        assert_eq!(result, 8);
    }
    #[test]
    fn check_example3() {
        let result = solve_day10b(EXAMPLE3_FILE_PATH);
        assert_eq!(result, 4);
    }
    #[test]
    fn check_example4() {
        let result = solve_day10b(EXAMPLE4_FILE_PATH);
        assert_eq!(result, 4);
    }
    #[test]
    fn check_example5() {
        let result = solve_day10b(EXAMPLE5_FILE_PATH);
        assert_eq!(result, 8);
    }
    #[test]
    fn check_example6() {
        let result = solve_day10b(EXAMPLE6_FILE_PATH);
        assert_eq!(result, 10);
    }
    #[test]
    fn check_input_p1() {
        let result = solve_day10b(INPUT_FILE_PATH);
        assert_eq!(result, 6882);
    }
}
