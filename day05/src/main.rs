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

fn get_seed_intervals(line: &str) -> Vec<(u64, u64)> {
    let only_seeds = line.split(':').nth(1).unwrap().trim_start();
    println!("{only_seeds}");
    let mut seeds_vec = Vec::new();
    let mut index = 0;
    while index < only_seeds.len() {
        let interval_start: u64;
        let interval_length: u64;
        (interval_start, index) = get_full_number_and_end_index(only_seeds, index);
        if interval_start == 0 {
            index += 1 // Space,
        } else {
            index += 2;
            (interval_length, index) = get_full_number_and_end_index(only_seeds, index);
            seeds_vec.push((interval_start, interval_start + interval_length - 1));
        }
        index += 1;
    }
    seeds_vec
}

fn is_valid_interval(interval: (u64, u64)) -> bool {
    return interval.0 <= interval.1;
}

fn slice_interval(
    current_interval: (u64, u64),
    src_interval: (u64, u64),
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
    let mut non_intersecting_intervals = Vec::new();
    let mut mapped_intervals: Vec<(u64, u64)> = Vec::new();
    // Not intersecting
    if current_interval.1 < src_interval.0 || src_interval.1 < current_interval.0 {
        println!(" >> 1 Pushed {:?} to non-intersecting", current_interval);
        non_intersecting_intervals.push(current_interval);
    }
    // Fully contained in new intersection
    else if current_interval.0 >= src_interval.0 && current_interval.1 <= src_interval.1 {
        println!(" >> 2 Pushed {:?} to     intersecting", current_interval);
        mapped_intervals.push(current_interval);
        // mapped_intervals.push(current_interval);
    }
    // New intersection is fully contained in interval
    else if src_interval.0 >= current_interval.0 && src_interval.1 <= current_interval.1 {
        let left_non_intersect = (current_interval.0, src_interval.0 - 1);
        let right_non_intersect = (src_interval.1 + 1, current_interval.1);
        if is_valid_interval(right_non_intersect) {
            println!(" >> 3 Pushed {:?} to non-intersecting", right_non_intersect);
            non_intersecting_intervals.push(right_non_intersect);
        }
        if is_valid_interval(left_non_intersect) {
            println!(" >> 4 Pushed {:?} to non-intersecting", left_non_intersect);
            non_intersecting_intervals.push(left_non_intersect);
        }
        mapped_intervals.push(src_interval);
    }
    // Partially contained (left)
    else if current_interval.0 >= src_interval.0
    /*&& current_interval.1 > new_interval.1*/
    {
        let left_part = (current_interval.0, src_interval.1);
        let right_part = (src_interval.1 + 1, current_interval.1);
        if is_valid_interval(left_part) {
            println!(" >> 5 Pushed {:?} to     intersecting", left_part);
            mapped_intervals.push(left_part);
        } else {
            println!("Shouldn't get here I believe (Partially contained (left))");
        }
        if is_valid_interval(right_part) {
            println!(" >> 6 Pushed {:?} to non-intersecting", right_part);
            non_intersecting_intervals.push(right_part);
        }
    }
    // Partially contained (right)
    else
    /* current_interval.1 <= new_interval.1 && current_interval.0 < new_interval.0 */
    {
        let left_part = (current_interval.0, src_interval.0 - 1);
        let right_part = (src_interval.0, current_interval.1);
        if is_valid_interval(right_part) {
            println!(" >> 7 Pushed {:?} to     intersecting", right_part);
            mapped_intervals.push(right_part);
        }
        if is_valid_interval(left_part) {
            println!(" >> 8 Pushed {:?} to non-intersecting", left_part);
            non_intersecting_intervals.push(left_part);
        }
    }

    return (non_intersecting_intervals, mapped_intervals);
}

// ASSUME!!! interval is fully contained inside mapping
fn map_single_interval(interval: (u64, u64), mapping: (u64, u64, u64)) -> (u64, u64) {
    let src_start = mapping.1;
    let dst_start = mapping.0;
    let length = mapping.2;

    let diff_from_start = interval.0 - src_start;
    return (
        dst_start + diff_from_start,
        dst_start + diff_from_start + (interval.1 - interval.0),
    );
}

fn map_intervals(interval: (u64, u64), mappings: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut unmapped_intervals: Vec<(u64, u64)> = vec![interval];
    let mut out_intervals: Vec<(u64, u64)> = Vec::new();
    for mapping in mappings {
        println!(" > Looking into mapping {:?}", mapping);
        let src_start = mapping.1;
        let length = mapping.2;
        let src_end = src_start + length - 1;

        for (index, unmapped_interval) in unmapped_intervals.clone().iter().enumerate() {
            // println!(
            //     " > Checking if interval {:?} fits in {:?}",
            //     unmapped_interval, mapping
            // );
            let (mut non_intersecting_intervals, intersecting_intervals) =
                slice_interval(*unmapped_interval, (src_start, src_end));

            if intersecting_intervals.len() == 0 {
                // println!(" >> No match");
                continue;
            }
            // println!(" >> Match");
            unmapped_intervals.remove(index);
            unmapped_intervals.append(non_intersecting_intervals.as_mut());
            for interval_to_map in intersecting_intervals {
                // println!(" >>> Mapping interval {:?}", interval_to_map);
                let mapped_interval = map_single_interval(interval_to_map, *mapping);
                // println!(" >>> Mapped to {:?}", mapped_interval);
                out_intervals.push(mapped_interval);
            }
        }
    }

    for unmapped_interval in unmapped_intervals {
        out_intervals.push(unmapped_interval);
    }
    assert!(out_intervals.len() > 0);
    return out_intervals;
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
    let mut seeds: Vec<(u64, u64)> = get_seed_intervals(seeds_line);
    println!("{:?}", seeds);
    let all_mappings = get_all_mappings(&content);
    println!("{:?}", all_mappings);
    for (index, mapping) in all_mappings.iter().enumerate() {
        println!(" #{index} Started with {:?}", seeds);
        println!("Mappings are {:?}", mapping);
        let mut new_seeds: Vec<(u64, u64)> = Vec::new();
        for i in 0..seeds.len() {
            // println!("Mapped {} to {}", seeds[i], map_x_to_y(seeds[i], &mapping));
            let mut new_intervals = map_intervals(seeds[i], &mapping);
            new_seeds.append(new_intervals.as_mut());
            // seeds[i] = map_x_to_y(seeds[i], &mapping);
        }
        seeds = new_seeds;
        println!(" #{index} Ended with {:?}", seeds);
    }
    let mut min_location = u64::MAX;
    for interval in seeds {
        min_location = std::cmp::min(min_location, interval.0);
    }
    println!("The minimum location is {min_location}");
}
