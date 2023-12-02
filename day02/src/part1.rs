use std::fs;
use phf::phf_map;

const INPUT_FILE_PATH: &str = "input.txt";

// phf::map<&str, i32> = phf_map!
static BALLS_AMOUNT: phf::Map<&'static str, i32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

// const AMOUNT_RED: i32 = 12;
// const AMOUNT_GREEN: i32 = 13;
// const AMOUNT_BLUE: i32 = 14;
// const BALLS_AMOUNT: HashMap<&str, i32> = HashMap::from([
//     ("red", 12),
//     ("green", 13),
//     ("blue", 14),
// ]);

fn is_selection_possible(line: &str) -> bool {
    // let balls_amount: HashMap<&str, i32> = HashMap::from([
    //     ("red", 12),
    //     ("green", 13),
    //     ("blue", 14),
    // ]);
    let balls = line.split(',');
    for ball_selection in balls {
        let ball_selection =  ball_selection.strip_prefix(' ').unwrap();
        let ball_amount: i32 = ball_selection.split(' ').nth(0).unwrap().parse().expect("This should have been a number..");
        let balls_color: &str = ball_selection.split(' ').nth(1).unwrap();
        if *BALLS_AMOUNT.get(balls_color).unwrap() < ball_amount {
            return false;
        }
    }
    return true;
}

fn is_game_possible(line: &str) -> bool {
    let selections = line.split(';');
    for selection in selections {
        if !is_selection_possible(selection) {
            return false;
        }
    }
    return true;
}

fn main() {
    let content: String = fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum: u32 = 0;

    for (line_id, line) in content.lines().enumerate() {
        println!("Line: {}", line);
        let raw_game = line.split(':').nth(1).unwrap(); // Strip the 'Game ##:' part
        if is_game_possible(raw_game) {
            println!("Game possible :)");
            sum += (line_id as u32) + 1;
        }
    }
    println!("Sum: {}", sum); // Sum: 2593
}
