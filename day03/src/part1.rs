use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn multiline_string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_first_digit(line: Vec<char>, col: usize) -> bool {
    if col == 0 {
        return true;
    }

    if !line.get(col - 1).unwrap().is_digit(10) {
        return true;
    }
    return false;
}

fn is_symbol(c: &char) -> bool {
    if c.is_digit(10) || c == &'.' {
        return false;
    }
    return true;
}

fn return_part_number_or_zero(content: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut num_value = 0;
    let mut col_iter = col;
    let mut is_part_number = false;
    let main_row = content.get(row).unwrap();

    // Check for symbol left
    if col > 0
        && (is_symbol(main_row.get(col - 1).unwrap())
            || (row > 0 && is_symbol(content.get(row - 1).unwrap().get(col - 1).unwrap()))
            || (row < content.len() - 1
                && is_symbol(content.get(row + 1).unwrap().get(col - 1).unwrap())))
    {
        is_part_number = true;
    }

    while col_iter < main_row.len() && main_row.get(col_iter).unwrap().is_digit(10) {
        num_value = num_value * 10 + main_row.get(col_iter).unwrap().to_digit(10).unwrap();

        // Check for symbol up
        if row > 0 && is_symbol(content.get(row - 1).unwrap().get(col_iter).unwrap()) {
            is_part_number = true;
        }

        // Check for symbol down
        if row < content.len() - 1
            && is_symbol(content.get(row + 1).unwrap().get(col_iter).unwrap())
        {
            is_part_number = true;
        }
        col_iter += 1;
    }

    // Check for symbol right
    if col_iter < main_row.len()
        && (is_symbol(main_row.get(col_iter).unwrap())
            || (row > 0 && is_symbol(content.get(row - 1).unwrap().get(col_iter).unwrap()))
            || (row < content.len() - 1
                && is_symbol(content.get(row + 1).unwrap().get(col_iter).unwrap())))
    {
        is_part_number = true;
    }

    if is_part_number {
        num_value
    } else {
        0
    }
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum = 0;

    let content_as_vectors = multiline_string_to_2d_vector(&content);

    let mut row = 0;
    for a in content_as_vectors.clone() {
        let mut col = 0;
        println!("Currently scanning row: {:?}", a);
        for b in a.clone().into_iter() {
            if b.is_digit(10) && is_first_digit(a.clone(), col) {
                println!("{} is the first digit!", b);
                let part_number = return_part_number_or_zero(&content_as_vectors, row, col);
                sum += part_number;
                if part_number > 0 {
                    println!("Found an actual part number - {}", part_number);
                }
            }
            col += 1;
        }
        row += 1;
    }
    println!("Sum: {}", sum); // Sum: 527369
}
