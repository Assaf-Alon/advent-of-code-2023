use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn get_spelled_digit_helper(line: &str, spelled_digit: &str, reverse: bool) -> usize {
    let spelled_digit_index = if reverse {
        line.rfind(spelled_digit)
    } else {
        line.find(spelled_digit)
    };

    match spelled_digit_index {
        None => usize::MAX,
        Some(d) => d.try_into().unwrap(),
    }
}

fn get_first_spelled_digit(line: &str, reverse: bool) -> (u32, usize) {
    let spelled_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first_digit = 0;
    let mut first_index = usize::MAX;
    if reverse {
        first_index = 0;
    }
    let mut digit_value = 0;
    for spelled_digit in spelled_digits {
        digit_value += 1;
        let digit_index = get_spelled_digit_helper(line, spelled_digit, reverse);
        if digit_index < usize::MAX
            && (first_index == usize::MAX
                || (!reverse && first_index > digit_index)
                || (reverse && first_index < digit_index))
        {
            first_digit = digit_value;
            first_index = digit_index;
        }
    }
    return (first_digit, first_index);
}

fn get_first_value(line: &str, reverse: bool) -> u32 {
    let (first_spelled_digit, first_spelled_digit_index) = get_first_spelled_digit(line, reverse);
    let mut last_value_found = first_spelled_digit;
    println!("{}", first_spelled_digit_index);
    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            if index < first_spelled_digit_index && !reverse {
                println!("First value: {}", character.to_digit(10).unwrap());
                return character.to_digit(10).unwrap();
            }

            if index > first_spelled_digit_index && reverse {
                last_value_found = character.to_digit(10).unwrap();
            }
        }
    }
    if reverse {
        println!("Last value: {}", last_value_found);
        return last_value_found;
    }
    return first_spelled_digit;
}

fn get_calibration_value_from_line(line: &str) -> u32 {
    let first_digit = get_first_value(line, false);
    let mut last_digit = get_first_value(line, true);
    if last_digit == 0 {
        last_digit = first_digit;
    }
    return first_digit * 10 + last_digit;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum: u32 = 0;

    for line in content.lines() {
        println!("Line: {}", line);
        let num_from_line = get_calibration_value_from_line(line);
        sum += num_from_line;
        println!("{}", num_from_line);
    }
    println!("Sum: {}", sum); // Sum: 54676
                              // 54736 Option 1
                              // 54676 > Correct option
                              // 53928 Option 2
}
