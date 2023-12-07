use phf::phf_map;
use std::fmt;
use std::fmt::Display;
use std::{collections::HashMap, fs};

const INPUT_FILE_PATH: &str = "input.txt";

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandTypes {
    NoHand,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

static SYMBOL_TO_VALUE: phf::Map<&'static str, u32> = phf_map! {
    "T" => 10,
    "J" => 11,
    "Q" => 12,
    "K" => 13,
    "A" => 14,
};

impl fmt::Display for HandTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandTypes::FiveOfKind => write!(f, "Five of a Kind"),
            HandTypes::FourOfKind => write!(f, "Four of a Kind"),
            HandTypes::FullHouse => write!(f, "Full House"),
            HandTypes::ThreeOfKind => write!(f, "Three of a Kind"),
            HandTypes::TwoPairs => write!(f, "Two Pairs"),
            HandTypes::OnePair => write!(f, "One Pair"),
            HandTypes::HighCard => write!(f, "High Card"),
            _ => write!(f, "ERROR?"),
        }
    }
}

fn get_full_number_and_end_index(line: &str, start_index: usize) -> (u32, usize) {
    let mut index = start_index;
    let mut number: u32 = 0;
    while index < line.len() && line.chars().nth(index).unwrap().is_digit(10) {
        number = 10 * number + line.chars().nth(index).unwrap().to_digit(10).unwrap();
        index += 1;
    }
    return (number, index - 1);
}

fn convert_card_to_value(symbol: char) -> u32 {
    if symbol.is_digit(10) {
        return symbol.to_digit(10).unwrap();
    }
    return *SYMBOL_TO_VALUE.get(symbol.to_string().as_str()).unwrap();
}

fn get_hand_and_bid_from_line(line: &str) -> (&str, u32) {
    let hand = line.split(' ').nth(0).unwrap();
    let (bid, _) = get_full_number_and_end_index(line, 6);
    (hand, bid)
}

fn get_hand_strength(hand: &str) -> (HandTypes, u32) {
    let mut hand_as_map: HashMap<u32, i32> = HashMap::new();
    let base = 20;
    let mut hand_raw_value = 0;
    for card in hand.chars() {
        let card_value = convert_card_to_value(card);
        hand_raw_value = hand_raw_value * base + card_value;
        *hand_as_map.entry(card_value).or_insert(0) += 1;
    }

    // Stronger cards will be placed before weaker ones
    // let hand_raw_value = base.pow(4) * car
    // cards_values.sort_by(|a, b| b.cmp(a));

    // 5, 4, 3, 2, 1
    let mut amount_x_of_a_kind = vec![0, 0, 0, 0, 0];
    let mut hand_type: HandTypes = HandTypes::HighCard;
    let mut sorted_values = hand_as_map.values().cloned().collect::<Vec<i32>>();
    sorted_values.sort_by(|a, b| b.cmp(a));

    match sorted_values[0] {
        5 => hand_type = HandTypes::FiveOfKind,
        4 => hand_type = HandTypes::FourOfKind,
        3 => {
            if sorted_values[1] == 2 {
                hand_type = HandTypes::FullHouse;
            } else {
                hand_type = HandTypes::ThreeOfKind;
            }
        }
        2 => {
            if sorted_values[1] == 2 {
                hand_type = HandTypes::TwoPairs;
            } else {
                hand_type = HandTypes::OnePair;
            }
        }
        1 => hand_type = HandTypes::HighCard,
        _ => println!("{} - SHOULDN'T GET HERE", hand),
    }

    return (hand_type, hand_raw_value);
}

fn main() {
    println!("Hello, world!");
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");

    let mut full_hand_data: Vec<(HandTypes, u32, u32)> = Vec::new();

    for line in content.lines() {
        let (hand, bid) = get_hand_and_bid_from_line(line);
        println!("Hand: {},  Bid: {}", hand, bid);
        let hand_strength = get_hand_strength(hand);
        println!(" > Strength: {}", hand_strength.0);
        full_hand_data.push((hand_strength.0, hand_strength.1, bid));
    }

    let mut total_winnings = 0;
    full_hand_data.sort();
    for (index, hand_data) in full_hand_data.iter().enumerate() {
        println!(
            "Hand #{} - {}, {}, {}",
            index, hand_data.0, hand_data.1, hand_data.2
        );
        total_winnings += ((index + 1) as u32) * hand_data.2;
    }
    println!("Total winnings: {total_winnings}");
}
