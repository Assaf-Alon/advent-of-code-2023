use std::fs;

const EXAMPLE_FILE_PATH: &str = "example.txt";
const INPUT_FILE_PATH: &str = "input.txt";

fn get_full_number_and_end_index(line: &str, start_index: usize) -> (i32, usize) {
    let mut index = start_index;
    let mut number: i32 = 0;
    let mut is_negative = false;
    while index < line.len()
        && (line.chars().nth(index).unwrap().is_digit(10)
            || line.chars().nth(index).unwrap() == '-')
    {
        if line.chars().nth(index).unwrap() == '-' {
            is_negative = true;
            index += 1;
            continue;
        }
        number = 10 * number + line.chars().nth(index).unwrap().to_digit(10).unwrap() as i32;
        index += 1;
    }
    if is_negative {
        number = -number;
    }
    return (number, index - 1);
}

fn get_num_vector_from_line(line: &str) -> Vec<i32> {
    let mut index = 0;
    let mut num_vec = Vec::new();
    while index < line.len() {
        let number;
        (number, index) = get_full_number_and_end_index(line, index);
        num_vec.push(number);
        index += 2;
    }
    num_vec
}

fn extrapolate_step(numbers: &Vec<i32>) -> Vec<i32> {
    let mut extrapolated_numbers = Vec::new();
    for i in 1..numbers.len() {
        extrapolated_numbers.push(numbers[i] - numbers[i - 1]);
    }
    extrapolated_numbers
}

// TODO - consider finding closed formula for this
fn get_history_value(line: &str, is_part_two: bool) -> i32 {
    let line_numbers = get_num_vector_from_line(line);
    let mut current_numbers = line_numbers;
    let mut history_value = 0;
    let mut add_multiplier = 1;
    while current_numbers.len() > 0 {
        let mut all_zeros = true;
        for &num in current_numbers.iter() {
            if num != 0 {
                all_zeros = false;
                break;
            }
        }
        if all_zeros {
            break;
        }
        if is_part_two {
            history_value += add_multiplier * current_numbers.first().unwrap().clone();
        } else {
            history_value += current_numbers.last().unwrap().clone();
        }
        current_numbers = extrapolate_step(&current_numbers);
        if is_part_two {
            add_multiplier = -add_multiplier;
        }
    }

    history_value
}

fn solve_day09(file_path: &str, is_part_two: bool) -> i32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let mut sum = 0;
    for line in content.lines() {
        let history_value = get_history_value(line, is_part_two);
        println!("History value: {:?}", history_value);
        sum += history_value;
    }
    return sum;
}

fn main() {
    let result = solve_day09(INPUT_FILE_PATH, true);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_num_vectors_from_line() {
        let content: String =
            fs::read_to_string(EXAMPLE_FILE_PATH).expect("Failed to read file content :/");
        assert_eq!(
            get_num_vector_from_line(content.lines().nth(0).unwrap()),
            vec![0, 3, 6, 9, 12, 15]
        );
        assert_eq!(
            get_num_vector_from_line(content.lines().nth(1).unwrap()),
            vec![1, 3, 6, 10, 15, 21]
        );
        assert_eq!(
            get_num_vector_from_line(content.lines().nth(2).unwrap()),
            vec![10, 13, 16, 21, 30, 45]
        );

        assert_eq!(
            get_num_vector_from_line("-4 -8 -12 5 -16"),
            vec![-4, -8, -12, 5, -16]
        );
    }

    #[test]
    fn check_example_part1() {
        let result = solve_day09(EXAMPLE_FILE_PATH, false);
        assert_eq!(result, 114);
    }
    #[test]
    fn check_input_part1() {
        let result = solve_day09(INPUT_FILE_PATH, false);
        assert_eq!(result, 2008960228);
    }
    #[test]
    fn check_example_part2() {
        let result = solve_day09(EXAMPLE_FILE_PATH, true);
        println!("Result : {}", result);
        assert_eq!(result, 2);
    }
    #[test]
    fn check_input_part2() {
        let result = solve_day09(EXAMPLE_FILE_PATH, true);
        println!("Result : {}", result);
        assert_eq!(result, 2);
    }
}
