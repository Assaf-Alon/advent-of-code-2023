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

fn get_scratch_card_matches(line: &str) -> i32 {
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
    return amount_of_winning_numbers;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut cards_amount: Vec<i32> = vec![1; content.lines().count()];

    for (index, line) in content.lines().enumerate() {
        let cards_won: i32 = get_scratch_card_matches(line);
        for card_index in index + 1..index + 1 + cards_won as usize {
            println!("Adding {} to card {}", cards_amount[index], card_index);
            cards_amount[card_index] += cards_amount[index];
        }
    }
    println!("{:?}", cards_amount);
    let sum: i32 = cards_amount.iter().sum();
    println!("{}", sum); // Sum: 8477787
}
