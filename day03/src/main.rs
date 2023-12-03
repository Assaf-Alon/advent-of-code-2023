use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn multiline_string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_full_number_of_coords(content: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    // Not a number
    if !content.get(row).unwrap().get(col).unwrap().is_digit(10) {
        return 0;
    }

    let mut col_iter = col;
    let main_row = content.get(row).unwrap();
    let mut num_value = 0;

    // Not a start of a number
    while col_iter > 0
        && content
            .get(row)
            .unwrap()
            .get(col_iter - 1)
            .unwrap()
            .is_digit(10)
    {
        col_iter -= 1;
    }
    // A start of a number
    while col_iter < main_row.len() && main_row.get(col_iter).unwrap().is_digit(10) {
        num_value = num_value * 10 + main_row.get(col_iter).unwrap().to_digit(10).unwrap();
        col_iter += 1;
    }
    // println!("Returning {} from {}, {}", num_value, row, col);
    return num_value;
}

// skip=0 for first, skip=1 for second
fn find_number_that_touches_coords(
    content_as_vectors: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    skip: u32,
) -> u32 {
    let mut last_found_number = 0;
    let mut skip = skip;
    let mut last_found_row = usize::MAX;
    let mut last_found_col = usize::MAX;
    for row_diff in 0..=2 {
        for col_diff in 0..=2 {
            let num_in_coords = get_full_number_of_coords(
                &content_as_vectors,
                row + row_diff - 1,
                col + col_diff - 1,
            );
            if num_in_coords > 0 {
                // Different rows OR skipped a col
                if last_found_row != row + row_diff - 1
                    || (last_found_col != usize::MAX && last_found_col + 1 < col + col_diff - 1)
                {
                    // println!("Found a good number. current SKIP = {}", skip);
                    last_found_number = num_in_coords;
                    if skip == 0 {
                        return last_found_number;
                    } else {
                        skip -= 1;
                        println!("Skipping {}", last_found_number);
                    }
                }
                last_found_row = row + row_diff - 1;
                last_found_col = col + col_diff - 1;
            }
        }
    }
    println!(
        "SECRET VALUE: {}",
        content_as_vectors
            .get(row + 1)
            .unwrap()
            .get(col + 1)
            .unwrap()
    );
    return 0;
}

fn main() {
    let content: String =
        fs::read_to_string(INPUT_FILE_PATH).expect("Failed to read file content :/");
    let mut sum = 0;

    let content_as_vectors = multiline_string_to_2d_vector(&content);

    let mut total_asterisks = 0;
    let mut good_asterisks = 0;

    let mut row_num = 0;
    for rows in content_as_vectors.clone() {
        let mut col_num = 0;
        // println!("Currently scanning row: {:?}", a);
        for current_char in rows.clone().into_iter() {
            if current_char == '*' {
                total_asterisks += 1;
                println!("Found asterisk at {}, {}", row_num, col_num);

                // Try to find exactly two numbers
                let first_number =
                    find_number_that_touches_coords(&content_as_vectors, row_num, col_num, 0);
                if first_number == 0 {
                    println!("Touches nothing");
                    col_num += 1;
                    continue;
                }
                let second_number =
                    find_number_that_touches_coords(&content_as_vectors, row_num, col_num, 1);
                if second_number == 0 {
                    println!("Only touches {}", first_number);
                    col_num += 1;
                    continue;
                }
                if find_number_that_touches_coords(&content_as_vectors, row_num, col_num, 2) > 0 {
                    println!("PANIC!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                }
                good_asterisks += 1;
                println!(
                    "THE ACTUAL MATH: {} * {} = {}",
                    first_number,
                    second_number,
                    first_number * second_number
                );
                println!("---------------------------");
                sum += first_number * second_number;
            }
            col_num += 1;
        }
        row_num += 1;
    }
    println!("Total: {}, Good: {}", total_asterisks, good_asterisks);
    println!("Sum: {}", sum); // Sum: 73074886
}
