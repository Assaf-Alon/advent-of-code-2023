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

fn get_number_in_line(line: &str) -> u64 {
    let mut line_as_vec: Vec<char> = line.chars().collect();
    let mut nums_in_line: Vec<u64> = Vec::new();
    let mut index = 0;
    while index < line_as_vec.len() {
        if line_as_vec[index] == ' ' {
            index += 1;
            continue;
        }
        let (number, end_index) = get_full_number_and_end_index(line, index);
        nums_in_line.push(number);
        index = end_index + 1;
    }
    let mut final_num = 0;
    for num in nums_in_line {
        // let num_digits = numeric::math::log10(num).floor();
        let num_digits = f64::log10(num as f64).ceil() as u32;
        println!("num digits of {} is {}", num, num_digits);
        let base: u64 = 10;
        final_num = final_num * base.pow(num_digits) + num;
    }
    println!("Final num: {}", final_num);
    // return nums_in_line;
    final_num
}

fn get_race_times_and_records(content: &str) -> Vec<(u64, u64)> {
    let mut races_and_records: Vec<(u64, u64)> = Vec::new();

    let time_line = content.lines().nth(0).unwrap().split(':').nth(1).unwrap();
    let dist_line = content.lines().nth(1).unwrap().split(':').nth(1).unwrap();

    let time: u64 = get_number_in_line(time_line);
    let distance: u64 = get_number_in_line(dist_line);

    races_and_records.push((time, distance));
    races_and_records
}

fn get_amount_of_ways_to_beat_record(race_time: u64, race_record: u64) -> u64 {
    let mut first_good_charge_time = 0;
    for charge_time in 0..race_time {
        if (race_time - charge_time) * charge_time > race_record {
            first_good_charge_time = charge_time;
            break;
        }
    }
    if first_good_charge_time == 0 {
        return 0;
    }

    for time_spent_advancing in 0..race_time {
        let charge_time = race_time - time_spent_advancing;
        if (race_time - charge_time) * charge_time > race_record {
            return 1 + charge_time - first_good_charge_time;
        }
    }

    println!("SHOULDN'T GET HERE!!!!!!!!!!!!!!!!");
    return 0;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");

    let mut total: u64 = 0;
    let races_and_records = get_race_times_and_records(&content);
    println!("Races and records: {:?}", races_and_records);
    for (race_time, race_record) in races_and_records {
        let amount_of_ways_to_beat = get_amount_of_ways_to_beat_record(race_time, race_record);
        if total == 0 {
            total = amount_of_ways_to_beat;
        } else if amount_of_ways_to_beat > 0 {
            total *= amount_of_ways_to_beat;
        }
    }
    println!("Total: {}", total);
}
