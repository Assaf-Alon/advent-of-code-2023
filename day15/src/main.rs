use std::hash;
use std::{collections::HashMap, fs, process::Command};

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

fn solve_day15a(file_path: &str) -> u32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let comma_separated_content = content.split(',');
    let mut total_hash = 0;
    for hashable in comma_separated_content {
        total_hash += hash_algorithm(hashable);
    }
    // let matrix = get_2d_vector_from_multiline_string(&content);
    total_hash
}

fn solve_day15b(file_path: &str) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let comma_separated_content = content.split(',');
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    'labels_loop: for label_and_operator in comma_separated_content {
        if label_and_operator.ends_with('-') {
            let label = label_and_operator.split('-').nth(0).unwrap();
            let label_hash = hash_algorithm(label);
            // let hashed_box = &boxes[label_hash as usize];
            for (index, (boxed_label, _)) in boxes[label_hash as usize].clone().iter().enumerate() {
                if label == *boxed_label {
                    boxes[label_hash as usize].remove(index);
                    continue 'labels_loop;
                }
            }
        } else {
            let label = label_and_operator.split('=').nth(0).unwrap();
            let lens_strength: u32 = label_and_operator
                .split('=')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let label_hash = hash_algorithm(label);
            let mut found_label = false;
            for (index, (boxed_label, _)) in boxes[label_hash as usize].clone().iter().enumerate() {
                if label == *boxed_label {
                    boxes[label_hash as usize][index] = (label, lens_strength);
                    found_label = true;
                    break;
                }
            }
            if !found_label {
                boxes[label_hash as usize].push((label, lens_strength));
            }
        }
    }

    let mut total_sum = 0;
    for (box_num, lens_box) in boxes.iter().enumerate() {
        for (lens_num, (_, strength)) in lens_box.iter().enumerate() {
            let current_sum = (box_num + 1) * (lens_num + 1) * (*strength as usize);
            total_sum += current_sum;
        }
    }

    total_sum
}

fn hash_algorithm(in_string: &str) -> u32 {
    let mut hash_value = 0;
    for c in in_string.chars() {
        hash_value += c as u32;
        hash_value *= 17;
        hash_value = hash_value % 256;
    }
    hash_value
}

fn main() {
    let result = solve_day15b(INPUT_FILE_PATH);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hash() {
        let result = hash_algorithm("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn check_example1() {
        let result = solve_day15a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 1320);
    }

    #[test]
    fn check_example1_p2() {
        let result = solve_day15a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 145);
    }

    #[test]
    fn check_input_p2() {
        let result = solve_day15a(INPUT_FILE_PATH);
        assert_eq!(result, 303404);
    }
}
