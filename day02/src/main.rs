use std::{collections::HashMap, fs};

const INPUT_FILE_PATH: &str = "input.txt";

fn get_selection_colors(line: &str) -> HashMap<&str, u32> {
    let balls: std::str::Split<'_, char> = line.split(',');
    let mut colors: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for ball_selection in balls {
        let ball_selection = ball_selection.strip_prefix(' ').unwrap();
        let ball_amount: u32 = ball_selection
            .split(' ')
            .nth(0)
            .unwrap()
            .parse()
            .expect("This should have been a number..");

        let balls_color: &str = ball_selection.split(' ').nth(1).unwrap();
        colors.insert(balls_color, ball_amount);
    }
    return colors;
}

fn get_game_power(line: &str) -> u32 {
    let mut max_reds = 0;
    let mut max_greens = 0;
    let mut max_blues = 0;

    let selections = line.split(';');
    for selection in selections {
        let colors = get_selection_colors(selection);
        max_reds = std::cmp::max(max_reds, colors["red"]);
        max_greens = std::cmp::max(max_greens, colors["green"]);
        max_blues = std::cmp::max(max_blues, colors["blue"]);
    }
    return max_reds * max_greens * max_blues;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum: u32 = 0;

    for line in content.lines() {
        // println!("Line: {}", line);
        let raw_game = line.split(':').nth(1).unwrap(); // Strip the 'Game ##:' part
        let game_power = get_game_power(raw_game);
        // println!("Game power: {}", game_power);
        sum += game_power;
    }
    println!("Sum: {}", sum); // Sum: 54699
}
