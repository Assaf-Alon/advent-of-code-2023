use regex::Regex;
use std::{collections::HashMap, fs};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";

fn get_parsed_input(
    content: &str,
) -> (
    HashMap<&str, Vec<(usize, char, i64, String)>>,
    Vec<Vec<i64>>,
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
            let x: i64 = captures[1].parse().unwrap();
            let m: i64 = captures[2].parse().unwrap();
            let a: i64 = captures[3].parse().unwrap();
            let s: i64 = captures[4].parse().unwrap();
            parts.push(vec![x, m, a, s]);
        }
    }
    (map, parts)
}

fn activate_workflow(part: &Vec<i64>, checks: &Vec<(usize, char, i64, String)>) -> String {
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

fn solve_day19a(file_path: &str) -> i64 {
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

fn main() {
    let result = solve_day19a(INPUT_FILE_PATH);
    println!("Result = {}", result);
}
