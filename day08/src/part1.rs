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

    // println!("{:?}", turns);
    let map_directions = get_directions_map(&content);
    println!("{:?}", map_directions);

    let mut reached_end = false;
    let mut current_location = "AAA";
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
            if current_location == "ZZZ" {
                reached_end = true;
                break;
            }
        }
    }
    println!("Total steps: {total_steps}"); // 21389
}
