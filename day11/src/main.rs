use std::{collections::HashSet, fs};

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const INPUT_FILE_PATH: &str = "input.txt";

// Returns:
// - HashSet that indicates for each coordinate whether there's a galaxy there
// - Binary Vector that keeps track of how many galaxies there are in a specific row
// - Binary Vector that keeps track of how many galaxies there are in a specific col
fn get_formatted_input(file_path: &str) -> (HashSet<(usize, usize)>, Vec<i32>, Vec<i32>) {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let mut galaxies_locations = HashSet::new();
    let mut amount_row_galaxies = vec![0; content.len()];
    let mut amount_col_galaxies = vec![0; content.lines().nth(0).unwrap().len()];
    for (i, row) in content.lines().enumerate() {
        for (j, symbol) in row.chars().enumerate() {
            if symbol == '#' {
                galaxies_locations.insert((i, j));
                amount_row_galaxies[i] += 1;
                amount_col_galaxies[j] += 1;
            }
        }
    }

    (galaxies_locations, amount_row_galaxies, amount_col_galaxies)
}

fn get_distance_gained_from_universe_expansion(
    src: usize,
    dst: usize,
    amount_vec: &Vec<i32>,
    is_part_two: bool,
) -> usize {
    let mut distance_increase = 0;
    for i in src..dst {
        if amount_vec[i] == 0 {
            if is_part_two {
                distance_increase += 999999;
            } else {
                distance_increase += 1;
            }
        }
    }
    distance_increase
}

fn get_galaxies_distance(
    galaxy1: (usize, usize),
    galaxy2: (usize, usize),
    amount_row_galaxies: &Vec<i32>,
    amount_col_galaxies: &Vec<i32>,
    is_part_two: bool,
) -> usize {
    let row1 = usize::min(galaxy1.0, galaxy2.0);
    let row2 = usize::max(galaxy1.0, galaxy2.0);
    let col1 = usize::min(galaxy1.1, galaxy2.1);
    let col2 = usize::max(galaxy1.1, galaxy2.1);

    let mut distance = (row2 - row1) + (col2 - col1); // Manhattan

    // Universe Expansion
    distance +=
        get_distance_gained_from_universe_expansion(row1, row2, amount_row_galaxies, is_part_two);
    distance +=
        get_distance_gained_from_universe_expansion(col1, col2, amount_col_galaxies, is_part_two);

    distance
}

fn solve_day11(file_path: &str, is_part_two: bool) -> usize {
    let (all_galaxies, amount_row_galaxies, amount_col_galaxies) = get_formatted_input(file_path);
    let mut total_distance = 0;
    for &galaxy1 in &all_galaxies {
        for &galaxy2 in &all_galaxies {
            let distance = get_galaxies_distance(
                galaxy1,
                galaxy2,
                &amount_row_galaxies,
                &amount_col_galaxies,
                is_part_two,
            );
            total_distance += distance;
            // println!("G1 = {:?}, G2 = {:?}", galaxy1, galaxy2);
            // println!(" > Distance: {distance}");
            // println!(" > Total distance: {total_distance}");
        }
    }

    total_distance / 2
}

fn main() {
    let result = solve_day11(INPUT_FILE_PATH, true);
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day11(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 374);
    }

    #[test]
    fn check_example2() {
        let result = solve_day11(EXAMPLE1_FILE_PATH, true);
        assert_eq!(result, 82000292);
    }

    #[test]
    fn check_input_p1() {
        let result = solve_day11(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 9214785);
    }

    #[test]
    fn check_input_p2() {
        let result = solve_day11(EXAMPLE1_FILE_PATH, false);
        assert_eq!(result, 613686987427);
    }
}
