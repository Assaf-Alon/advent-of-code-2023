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
const EXAMPLE3_FILE_PATH: &str = "example3.txt";
const EXAMPLE4_FILE_PATH: &str = "example4.txt";
const EXAMPLE5_FILE_PATH: &str = "example5.txt";
const EXAMPLE6_FILE_PATH: &str = "example6.txt";
const EXAMPLE7_FILE_PATH: &str = "example7.txt";
const EXAMPLE8_FILE_PATH: &str = "example9.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const ASH: char = '.';
const ROCK: char = '#';

fn get_2d_vector_from_multiline_string(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_symetric_locally(
    section: &Vec<Vec<char>>,
    sym_index: usize,
    major_index: usize,
    is_horizontal: bool,
) -> bool {
    let mut start_index = sym_index;
    let mut end_index = sym_index + 1;
    let last_end_index;
    if is_horizontal {
        last_end_index = section[0].len() - 1;
    } else {
        last_end_index = section.len() - 1;
    }
    while end_index <= last_end_index {
        let (row_start, col_start) = get_coords(major_index, start_index, is_horizontal);
        let (row_end, col_end) = get_coords(major_index, end_index, is_horizontal);
        if section[row_start][col_start] != section[row_end][col_end] {
            return false;
        }
        if start_index == 0 {
            break;
        }
        start_index -= 1;
        end_index += 1;
    }
    true
}

fn is_symetric_globally(section: &Vec<Vec<char>>, sym_index: usize, is_horizontal: bool) -> bool {
    let major_range;
    if is_horizontal {
        major_range = section.len();
    } else {
        major_range = section[0].len();
    }

    for i in 0..major_range {
        if !is_symetric_locally(section, sym_index, i, is_horizontal) {
            return false;
        }
    }
    true
}

fn get_mirrors(content: &str) -> Vec<Vec<Vec<char>>> {
    let mut mirror_sections = Vec::new();
    let sections = content.split("\n\n");
    for section in sections {
        mirror_sections.push(get_2d_vector_from_multiline_string(section));
    }
    println!("Made {} mirrors", mirror_sections.len());
    mirror_sections
}

fn get_coords(major: usize, minor: usize, is_horizontal: bool) -> (usize, usize) {
    if is_horizontal {
        return (major, minor);
    }
    (minor, major)
}

fn find_mirror_location(section: &Vec<Vec<char>>, is_horizontal: bool, ignore: usize) -> usize {
    let last_end_index;
    if is_horizontal {
        last_end_index = section[0].len();
    } else {
        last_end_index = section.len();
    }
    for i in 0..last_end_index - 1 {
        if ignore == i + 1 {
            continue;
        }
        if is_symetric_globally(section, i, is_horizontal) {
            return i + 1;
        }
    }
    usize::MAX
}

fn get_flipped_char(c: char) -> char {
    if c == ASH {
        ROCK
    } else {
        ASH
    }
}

fn fix_smudge_location(
    mirror_section: &Vec<Vec<char>>,
    sym_index: usize,
    is_horizontal: bool,
) -> (usize, bool) {
    let mut cloned_mirror_section = mirror_section.clone();
    for row in 0..cloned_mirror_section.len() {
        for col in 0..cloned_mirror_section[0].len() {
            cloned_mirror_section[row][col] = get_flipped_char(cloned_mirror_section[row][col]);
            let ignore = if is_horizontal { sym_index } else { usize::MAX };
            let mirror_location = find_mirror_location(&cloned_mirror_section, true, ignore);
            if mirror_location < usize::MAX && (mirror_location != sym_index || !is_horizontal) {
                return (mirror_location, true);
            }
            let ignore = if is_horizontal { usize::MAX } else { sym_index };
            let mirror_location = find_mirror_location(&cloned_mirror_section, false, ignore);
            if mirror_location < usize::MAX && (mirror_location != sym_index || is_horizontal) {
                return (mirror_location, false);
            }
            cloned_mirror_section[row][col] = get_flipped_char(cloned_mirror_section[row][col]);
        }
    }
    panic!("Nothing worked!");
}

fn solve_day13(file_path: &str, is_part_two: bool) -> usize {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let mirror_sections = get_mirrors(&content);
    let mut sum = 0;
    for (index, section) in mirror_sections.iter().enumerate() {
        println!("Searching in section {}", index);
        let mut mirror_location = find_mirror_location(&section, true, usize::MAX);
        let mut is_horizontal = true;
        if mirror_location < usize::MAX {
            println!(" > Found horizontal mirror on {}", mirror_location);
        } else {
            mirror_location = find_mirror_location(&section, false, usize::MAX);
            if mirror_location < usize::MAX {
                println!(" > Found vertical mirror on {}", mirror_location);
                is_horizontal = false;
            }
        }

        if is_part_two {
            (mirror_location, is_horizontal) =
                fix_smudge_location(section, mirror_location, is_horizontal);
        }

        if is_horizontal {
            sum += mirror_location;
        } else {
            sum += 100 * mirror_location;
        }
    }
    sum
}

fn main() {
    let result = solve_day13(INPUT_FILE_PATH, true);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day13(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 405);
    }
    #[test]
    fn check_example1_p2() {
        let result = solve_day13(EXAMPLE1_FILE_PATH, true);
        assert_eq!(result, 400);
    }

    #[test]
    fn check_example2() {
        let result = solve_day13(EXAMPLE2_FILE_PATH, false);
        assert_eq!(result, 1400);
    }

    #[test]
    fn check_example3() {
        let result = solve_day13(EXAMPLE3_FILE_PATH, false);
        assert_eq!(result, 10);
    }

    #[test]
    fn check_example4() {
        let result = solve_day13(EXAMPLE4_FILE_PATH, false);
        assert_eq!(result, 1);
    }

    #[test]
    fn check_input_p1() {
        let result = solve_day13(INPUT_FILE_PATH, false);
        assert_eq!(result, 29213);
    }

    #[test]
    fn check_input_p2() {
        let result = solve_day13(INPUT_FILE_PATH, true);
        assert_eq!(result, 37453);
    }
}
