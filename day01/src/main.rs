use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn get_first_digit(line: &str, reverse: bool) -> u32 {

    let digit = if reverse {
        line.chars().rev().find(|c: &char| c.is_digit(10))
    } else {
        line.chars().find(|c: &char| c.is_digit(10))
    };

    match digit {
        Some(d) => d.to_digit(10).unwrap(),
        None => 0,
    }
}

fn get_calibration_value_from_line(line: &str) -> u32 {
    let first_digit = get_first_digit(line, false);
    let last_digit = get_first_digit(line, true);
    return first_digit * 10 + last_digit;
}

fn main() {
    let content: String = fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum: u32 = 0;

    for line in content.lines() {
        // println!("Line: {}", line);
        let num_from_line = get_calibration_value_from_line(line);
        sum += num_from_line;
        // println!("{}", num_from_line);
    }
    println!("Sum: {}", sum); // Sum: 53921
}
