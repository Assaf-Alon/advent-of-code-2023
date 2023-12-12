use std::{collections::VecDeque, fs};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const EMPTY_SYMBOL: char = '.';
const UNKNOWN_SYMOBL: char = '?';
const SPRING_SYMBOL: char = '#';

fn get_numbers_from_line(line: &str) -> VecDeque<i32> {
    let numbers: VecDeque<i32> = line
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    numbers
}

fn can_springs_fit_in_slot(springs: i32, line: &VecDeque<char>) -> bool {
    // Not enough space
    if springs as usize > line.len() {
        return false;
    }

    for i in 0..springs {
        // There's an empty slot in the way
        if line[i as usize] == EMPTY_SYMBOL {
            return false;
        }
    }

    // Just right
    if springs as usize == line.len() {
        return true;
    }

    // true iff the next one is not stricly a spring
    return line[springs as usize] != SPRING_SYMBOL;
}

fn get_amount_of_springs(line: &VecDeque<char>) -> i32 {
    let mut springs = 0;
    for &ch in line.iter() {
        if ch == SPRING_SYMBOL {
            springs += 1;
        } else {
            break;
        }
    }
    return springs;
}

fn get_possible_arrangements(numbers: &mut VecDeque<i32>, line: &mut VecDeque<char>) -> i32 {
    // Stop condition 1
    if numbers.is_empty() {
        // There are still springs, even though the numbers state otherwise
        if line.iter().filter(|c| **c == SPRING_SYMBOL).count() > 0 {
            return 0;
        }

        return 1;
    }

    // Stop condition 2
    if line.is_empty() {
        return 0;
    }

    // Early exit 1 (too many numbers to put, nowhere to put them)
    let remaining_sum: i32 = numbers.iter().sum();
    if remaining_sum as usize
        > line
            .iter()
            .filter(|c| **c == UNKNOWN_SYMOBL || **c == SPRING_SYMBOL)
            .count()
    {
        return 0;
    }

    // No option here, keep going
    if line[0] == EMPTY_SYMBOL {
        line.pop_front();
        let arrangements = get_possible_arrangements(numbers, line);
        line.push_front(EMPTY_SYMBOL);
        return arrangements;
    }

    let mut total_possible_arrangements = 0;

    // Put '.' instead of '?'
    if line[0] == UNKNOWN_SYMOBL {
        line.pop_front();
        let possible_from_here = get_possible_arrangements(numbers, line);
        total_possible_arrangements += possible_from_here;
        line.push_front(UNKNOWN_SYMOBL);
    }

    let first_number = numbers[0];

    // Put '#' instead of '?'
    if can_springs_fit_in_slot(first_number, line) {
        let backup_line = line.clone();
        for _ in 0..first_number {
            line.pop_front();
        }

        // Popping the next one as well, as it won't have a spring
        if line.len() > 0 {
            line.pop_front();
        }

        numbers.pop_front();
        total_possible_arrangements += get_possible_arrangements(numbers, line);
        numbers.push_front(first_number);
        *line = backup_line.to_owned(); // Restore line for later usage
    } else if line[0] == SPRING_SYMBOL {
        // Can't fit even though we should fit here..
        return 0;
    }

    total_possible_arrangements
}

fn solve_day12(file_path: &str, is_part_two: bool) -> i32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let mut total_possible_arrangements = 0;
    for line in content.lines() {
        println!("Line: {}", line);
        let mut numbers: VecDeque<i32> = get_numbers_from_line(line);
        let mut line_as_vec: VecDeque<char> = line.split(' ').nth(0).unwrap().chars().collect();
        if is_part_two {
            let original_line = line_as_vec.clone();
            let original_numbers = numbers.clone();
            for _ in 0..4 {
                line_as_vec.push_back('?');
                line_as_vec.extend(original_line.iter().cloned());
                numbers.extend(original_numbers.iter().cloned());
            }
        }
        println!(" > Used numbers: {:?}", numbers);
        println!(" > Used line:    {:?}", line_as_vec);
        let arrangements = get_possible_arrangements(&mut numbers, &mut line_as_vec);
        println!(" > Arrangements: {}", arrangements);
        total_possible_arrangements += arrangements;
    }
    total_possible_arrangements
}

fn main() {
    let result = solve_day12(INPUT_FILE_PATH, true);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_line() {
        assert_eq!(
            get_numbers_from_line("?#?#?#?#?#?#?#? 1,3,1,6"),
            vec![1, 3, 1, 6]
        );
    }

    #[test]
    fn check_example1() {
        let result = solve_day12(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 21);
    }

    #[test]
    fn check_example2() {
        let result = solve_day12(EXAMPLE1_FILE_PATH, true);
        assert_eq!(result, 525152);
    }

    #[test]
    fn check_input_p1() {
        let result = solve_day12(INPUT_FILE_PATH, false);
        assert_eq!(result, 7032);
    }
}
