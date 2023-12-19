use regex::Regex;
use std::{collections::HashMap, fs};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";

fn get_parsed_input(
    content: &str,
) -> (
    HashMap<&str, Vec<(usize, char, u64, String)>>,
    Vec<Vec<u64>>,
) {
    let content_workflows = content.split("\n\n").nth(0).unwrap();
    let content_parts = content.split("\n\n").nth(1).unwrap();

    let mut map = HashMap::new();
    let mut parts = Vec::new();

    for line in content_workflows.lines() {
        let key = line.split('{').nth(0).unwrap();
        let mut checks = Vec::new();
        let line_checks = line.split('{').nth(1).unwrap().replace('}', "");
        for check in line_checks.split(',') {
            println!("Check: {check}");
            if !check.contains(':') {
                checks.push((0, '*', 0, check.to_string()));
                break;
            }
            let chr = check.chars().nth(0).unwrap();
            let chr_index = match chr {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => 4,
            };
            let sign = check.chars().nth(1).unwrap();
            let value = (&(check.split(':').nth(0).unwrap())[2..]).parse().unwrap();
            let where_to: String = check.split(':').nth(1).unwrap().to_string();
            checks.push((chr_index, sign, value, where_to));
        }
        map.insert(key, checks);
    }

    let line_regex: Regex = Regex::new(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"#).unwrap();
    for line in content_parts.lines() {
        if let Some(captures) = line_regex.captures(line) {
            let x: u64 = captures[1].parse().unwrap();
            let m: u64 = captures[2].parse().unwrap();
            let a: u64 = captures[3].parse().unwrap();
            let s: u64 = captures[4].parse().unwrap();
            parts.push(vec![x, m, a, s]);
        }
    }
    (map, parts)
}

fn activate_workflow(part: &Vec<u64>, checks: &Vec<(usize, char, u64, String)>) -> String {
    for check in checks {
        if check.1 == '*' {
            return check.3.clone();
        }
        if check.1 == '>' && part[check.0] > check.2 {
            return check.3.clone();
        } else if check.1 == '<' && part[check.0] < check.2 {
            return check.3.clone();
        }
    }
    return "ERROR".to_string();
}

fn solve_day19a(file_path: &str) -> u64 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let (map, parts) = get_parsed_input(&content);
    println!("Map: {:?}", map);
    println!("Parts: {:?}", parts);

    let mut sum = 0;
    for part in parts {
        let mut current_workflow = "in".to_string();
        while current_workflow != "A" && current_workflow != "R" {
            println!(" > Activating workflow on {current_workflow}");
            let checks = map.get(current_workflow.as_str()).unwrap();
            current_workflow = activate_workflow(&part, &checks);
        }
        println!(" > Ended up with {current_workflow}");
        if current_workflow == "A" {
            sum += part[0] + part[1] + part[2] + part[3];
        }
    }
    sum
}

// -----------------------

fn get_accepted_ranges(
    map: &HashMap<&str, Vec<(usize, char, u64, String)>>,
    current_workflow: &str,
    interval_to_check: &Vec<(u64, u64)>,
) -> Vec<Vec<(u64, u64)>> {
    // The vector has 4-tuples of intervals
    // Check format: (chr_index [xmas], sign[><], value[1..4000], where_to[id]);

    if current_workflow == "A" {
        return vec![interval_to_check.clone()];
    }

    if current_workflow == "R" {
        return vec![];
    }

    let mut collected_intervals = Vec::new();

    let mut remining_interval = interval_to_check.clone();
    for (chr_index, sign, value, where_to) in map[current_workflow].clone() {
        let changed_range = remining_interval[chr_index];

        // MATCH
        if sign == '*' {
            collected_intervals.extend(get_accepted_ranges(
                map,
                where_to.as_str(),
                &remining_interval,
            ));
            break;
        }

        // Current accepted ranges don't have anything in common with something that would pass here
        if (sign == '<' && changed_range.0 > value) || (sign == '>' && changed_range.1 < value) {
            continue;
        }

        // FULL MATCH - finished
        if (sign == '<' && changed_range.1 < value) || (sign == '>' && changed_range.0 > value) {
            collected_intervals.extend(get_accepted_ranges(
                map,
                where_to.as_str(),
                &remining_interval,
            ));
            break;
        }

        // Partial intersection
        if sign == '<' {
            let mut passed_interval = remining_interval.clone();
            passed_interval[chr_index].1 = value - 1;
            collected_intervals.extend(get_accepted_ranges(
                map,
                where_to.as_str(),
                &passed_interval,
            ));
            remining_interval[chr_index].0 = value;
        } else {
            // sign == '>'
            let mut passed_interval = remining_interval.clone();
            passed_interval[chr_index].0 = value + 1;
            collected_intervals.extend(get_accepted_ranges(
                map,
                where_to.as_str(),
                &passed_interval,
            ));
            remining_interval[chr_index].1 = value;
        }
    }

    collected_intervals
}

fn solve_day19b(file_path: &str) -> u64 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let (map, _) = get_parsed_input(&content);
    println!("Map: {:?}", map);
    let initial_intervals = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let intervals = get_accepted_ranges(&map, "in", &initial_intervals);
    let mut total_options = 0;
    for interval in intervals {
        let mut current_options = 1;
        for dimension in &interval {
            current_options *= dimension.1 - dimension.0 + 1;
        }
        println!("Another interval:");
        println!("{:?}", interval);
        println!("Addes {current_options} options");
        total_options += current_options;
    }
    total_options
}

fn main() {
    let result = solve_day19b(INPUT_FILE_PATH);
    println!("Result = {}", result);
}
