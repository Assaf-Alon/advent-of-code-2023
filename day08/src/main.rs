use num_integer::lcm;
use std::{collections::HashMap, fs};

const INPUT_FILE_PATH: &str = "input.txt";

fn get_directions_map(content: &str) -> HashMap<String, Vec<String>> {
    let mut map_directions: HashMap<String, Vec<String>> = HashMap::new();
    for line in content.lines() {
        if !line.contains("=") {
            continue;
        }
        let strip_line = line.replace("(", "").replace(")", "");
        let key = strip_line
            .as_str()
            .split("=")
            .nth(0)
            .unwrap()
            .strip_suffix(" ")
            .unwrap()
            .to_string();
        println!("{}", strip_line);
        let left = strip_line
            .split("=")
            .nth(1)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .strip_prefix(" ")
            .unwrap()
            .to_string();

        let right = strip_line
            .split("=")
            .nth(1)
            .unwrap()
            .split(",")
            .nth(1)
            .unwrap()
            .strip_prefix(" ")
            .unwrap()
            .to_string();
        // println!("Insrting {} , {:?}", key, (left, right));
        map_directions.insert(key, vec![left, right]);
    }
    map_directions
}

fn get_start_locations(content: &str) -> Vec<String> {
    let mut start_locations: Vec<String> = Vec::new();
    for line in content.lines() {
        if !line.contains("=") {
            continue;
        }
        let location = line.split("=").nth(0).unwrap().trim();
        if location.ends_with("A") {
            start_locations.push(String::from(location));
        }
    }
    return start_locations;
}

fn get_steps_from_start(
    map_directions: &HashMap<String, Vec<String>>,
    turns: &Vec<i32>,
    start_location: &str,
) -> u64 {
    let mut reached_end = false;
    let mut current_location = start_location;
    let mut total_steps = 0;

    while !reached_end {
        println!("Started iterating on turns");
        for turn in turns.iter() {
            println!(" > Current location: {current_location}");
            println!(" >> Steps so far: {total_steps}");
            println!(" >> Turning {}", if *turn == 0 { "Left" } else { "Right" });
            current_location = map_directions[current_location][*turn as usize].as_str();
            println!(" >> Reached {}", current_location);
            total_steps += 1;
            if current_location.ends_with("Z") {
                reached_end = true;
                break;
            }
        }
    }
    println!("Total steps: {total_steps}");
    total_steps
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");

    let turns: Vec<i32> = content
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    let map_directions = get_directions_map(&content);
    println!("{:?}", map_directions);

    let start_locations = get_start_locations(&content);

    let mut lcm_so_far: u64 = 1;
    for start in start_locations {
        let steps_from_start = get_steps_from_start(&map_directions, &turns, start.as_str());
        lcm_so_far = lcm(lcm_so_far, steps_from_start);
    }

    println!("Total steps: {lcm_so_far}"); // 21389
}
