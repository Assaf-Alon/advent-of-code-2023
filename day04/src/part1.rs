use regex::Regex;
use std::{collections::HashSet, fs};
const INPUT_FILE_PATH: &str = "input.txt";

fn clean_card_prefix(line: &str) -> String {
    let prefix_regex = Regex::new(r"^Card\s+\d+:").unwrap();
    let clean_line = prefix_regex.replace_all(line, "").trim().to_string();
    clean_line
}

fn get_full_number_and_end_index(line: &str, start_index: usize) -> (u32, usize) {
    let mut index = start_index;
    let mut number = 0;
    while index < line.len() && line.chars().nth(index).unwrap().is_digit(10) {
        number = 10 * number + line.chars().nth(index).unwrap().to_digit(10).unwrap();
        index += 1;
    }
    return (number, index - 1);
}

fn get_hashset_of_numbers_in_string(line: &str) -> HashSet<u32> {
    let mut index = 0;
    let mut numbers_hash = HashSet::new();
    while index < line.len() {
        while index < line.len() && !line.chars().nth(index).unwrap().is_digit(10) {
            index += 1;
        }

        if index < line.len() {
            let number;
            (number, index) = get_full_number_and_end_index(line, index);
            numbers_hash.insert(number);
        }
        index += 1;
    }
    return numbers_hash;
}

fn get_scratch_card_value(line: &str) -> u32 {
    let clean_line = clean_card_prefix(line);
    println!("Clean line: {}", clean_line);

    let winning_numbers_str = clean_line.split('|').nth(0).unwrap();
    let my_numbers_str = clean_line.split('|').nth(1).unwrap();
    let winning_numbers_set = get_hashset_of_numbers_in_string(winning_numbers_str);
    let my_numbers_set = get_hashset_of_numbers_in_string(my_numbers_str);

    let mut amount_of_winning_numbers = 0;

    for number in winning_numbers_set.iter() {
        if my_numbers_set.contains(number) {
            amount_of_winning_numbers += 1;
        }
    }
    println!("my numbers:      {:?}", my_numbers_set);
    println!("winning numbers: {:?}", winning_numbers_set);
    println!("Winning numbers: {}", amount_of_winning_numbers);
    if amount_of_winning_numbers == 0 {
        return 0;
    }
    let base: u32 = 2;
    return base.pow(amount_of_winning_numbers - 1);
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum: u32 = 0;

    for (index, line) in content.lines().enumerate() {
        // println!("Line: {}", line);
        let num_from_line = get_scratch_card_value(line);
        sum += num_from_line;
        println!("#{} - {}", index + 1, num_from_line);
        println!("Sum: {}", sum); //
        println!("---------------------")
    }
    println!("Sum: {}", sum); // Sum: 17782
}

// TOO HIGH 19449
