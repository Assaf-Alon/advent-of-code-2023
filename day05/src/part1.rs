use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn get_full_number_and_end_index(line: &str, start_index: usize) -> (u64, usize) {
    let mut index = start_index;
    let mut number: u64 = 0;
    while index < line.len() && line.chars().nth(index).unwrap().is_digit(10) {
        number = 10 * number + line.chars().nth(index).unwrap().to_digit(10).unwrap() as u64;
        index += 1;
    }
    return (number, index - 1);
}

fn get_seeds(line: &str) -> Vec<u64> {
    let only_seeds = line.split(':').nth(1).unwrap().trim_start();
    let mut seeds_vec = Vec::new();
    let mut index = 0;
    while index < line.len() {
        println!("Entered loop with index = {}", index);
        let number: u64;
        (number, index) = get_full_number_and_end_index(only_seeds, index);
        if number == 0 {
            index += 1 // Space,
        } else {
            seeds_vec.push(number);
        }
        index += 1;
    }
    seeds_vec
}

fn map_x_to_y(x: u64, mappings: &Vec<(u64, u64, u64)>) -> u64 {
    for mapping in mappings {
        let dst_start = mapping.0;
        let src_start = mapping.1;
        let length = mapping.2;
        let src_end = src_start + length - 1;
        if x < src_start || x > src_end {
            continue;
        }
        let diff_from_start = x - src_start;

        return dst_start + diff_from_start;
    }
    return x;
}

fn get_all_mappings(content: &str) -> Vec<Vec<(u64, u64, u64)>> {
    let new_content: Vec<&str> = content.lines().collect();
    let mut mappings_vec: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut line_index = 0;
    while line_index < new_content.len() {
        while line_index < new_content.len() && !new_content[line_index].contains("map") {
            line_index += 1;
        }
        line_index += 1;
        let mut current_line = new_content[line_index];
        println!("Found map {}", line_index);
        let mut mappings: Vec<(u64, u64, u64)> = Vec::new();
        while line_index < new_content.len() && !current_line.is_empty() {
            println!("{current_line}");
            let (dst_start, start_index) = get_full_number_and_end_index(current_line, 0);
            let (src_start, start_index) =
                get_full_number_and_end_index(current_line, start_index + 2);
            let (length, _) = get_full_number_and_end_index(current_line, start_index + 2);
            mappings.push((dst_start, src_start, length));
            line_index += 1;
            if line_index < new_content.len() {
                current_line = new_content[line_index];
            }
        }
        mappings_vec.push(mappings);
    }
    return mappings_vec;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let seeds_line = content.lines().next().unwrap();
    let mut seeds: Vec<u64> = get_seeds(seeds_line);
    println!("{:?}", seeds);

    let all_mappings = get_all_mappings(&content);
    for mapping in all_mappings {
        println!("Started with {:?}", seeds);
        for i in 0..seeds.len() {
            // println!("Mapped {} to {}", seeds[i], map_x_to_y(seeds[i], &mapping));
            seeds[i] = map_x_to_y(seeds[i], &mapping);
        }
        println!("Ended with {:?}", seeds);
    }
    let min_location = seeds.iter().min().unwrap();
    println!("The minimum location is {min_location}");
}
