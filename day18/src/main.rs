use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const UP: i32 = 0;
const DOWN: i32 = 1;
const LEFT: i32 = 2;
const RIGHT: i32 = 3;

fn get_visited_set_and_coords(content: &str) -> (HashSet<(i64, i64)>, i64, i64, i64, i64) {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut min_row = 0;
    let mut min_col = 0;
    let mut max_row = 0;
    let mut max_col = 0;
    let mut current_row = 0;
    let mut current_col = 0;
    for line in content.lines() {
        let direction = line.split(' ').nth(0).unwrap();
        let steps: i64 = line.split(' ').nth(1).unwrap().parse().unwrap();
        match direction {
            "R" => {
                for col in current_col..=current_col + steps {
                    visited.insert((current_row, col));
                    println!("Inserting {}, {}", current_row, col);
                }
                current_col += steps;
            }
            "L" => {
                for col in (current_col - steps)..=current_col {
                    visited.insert((current_row, col));
                }
                current_col -= steps;
            }
            "D" => {
                for row in current_row..=(current_row + steps) {
                    visited.insert((row, current_col));
                }
                current_row += steps;
            }
            "U" => {
                for row in (current_row - steps)..=current_row {
                    visited.insert((row, current_col));
                }
                current_row -= steps;
            }
            _ => panic!("No such step"),
        }
        max_row = i64::max(max_row, current_row);
        max_col = i64::max(max_col, current_col);
        min_row = i64::min(min_row, current_row);
        min_col = i64::min(min_col, current_col);
    }

    (visited, min_row, max_row, min_col, max_col)
}

fn pprint_visited(
    visited: &HashSet<(i64, i64)>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
) {
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if visited.contains(&(row, col)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!("");
    }
}

fn count_filled_tiles(
    visited: &HashSet<(i64, i64)>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64,
) -> i64 {
    let mut total_in = 0;
    for row in min_row..=max_row {
        let mut is_in = false;
        let mut total_in_row = 0;
        for col in min_col..=max_col {
            if visited.contains(&(row, col)) {
                total_in_row += 1;
                if visited.contains(&(row - 1, col)) {
                    is_in = !is_in;
                    println!(" > Flipping is_in");
                }
            } else if is_in {
                total_in_row += 1;
            }
        }
        println!("Total in row #{}: {}", row + 1, total_in_row);
        total_in += total_in_row;
    }
    println!("Total: {total_in}");
    total_in
}

fn solve_day18a(file_path: &str) -> i64 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let (visited, min_row, max_row, min_col, max_col) = get_visited_set_and_coords(&content);
    pprint_visited(&visited, min_row, max_row, min_col, max_col);
    let result = count_filled_tiles(&visited, min_row, max_row, min_col, max_col);
    result
}

//-----------------------------

fn get_locations_and_total_length(content: &str) -> (Vec<(i64, i64)>, i64) {
    let mut current_row = 0;
    let mut current_col = 0;
    let mut total_length = 0;
    let mut locations = Vec::new();
    for line in content.lines() {
        let offset;
        if line.len() > 13 {
            offset = 1;
        } else {
            offset = 0;
        }
        let amount2 = &line[6 + offset..11 + offset]; //.parse().unwrap();
        println!("amount2 = {amount2}");
        let amount: i64 = i64::from_str_radix(amount2, 16).unwrap();
        println!("amount  = {amount}");
        total_length += amount;
        let turn = line.chars().nth(11 + offset).unwrap();
        if turn == '0' {
            // R
            current_col += amount;
        } else if turn == '1' {
            // D
            current_row += amount;
        } else if turn == '2' {
            // L
            current_col -= amount;
        } else if turn == '3' {
            // U
            current_row -= amount;
        }
        locations.push((current_row, current_col));
        println!(" > Added ({current_row}, {current_col})");
    }
    (locations, total_length)
}

fn solve_day18b(file_path: &str) -> i64 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let (locations, total_length) = get_locations_and_total_length(&content);

    let mut sum = 0;
    for index in 1..locations.len() - 1 {
        let row = locations[index].0;
        let col = locations[index].1;
        let next_row = locations[index + 1].0;
        let next_col = locations[index + 1].1;
        sum += (col + next_col) * (row - next_row);
    }
    i64::abs(sum / 2) + total_length
}

fn main() {
    let result = solve_day18b(EXAMPLE1_FILE_PATH);
    println!("Result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day18a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 62);
    }

    // #[test]
    // fn check_example1_p2() {
    //     let result = solve_day18b(EXAMPLE1_FILE_PATH);
    //     assert_eq!(result, 94);
    // }
    // #[test]
    // fn check_example2_p2() {
    //     let result = solve_day18b(EXAMPLE2_FILE_PATH);
    //     assert_eq!(result, 71);
    // }
}
