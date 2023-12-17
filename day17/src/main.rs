use std::fs;

use pathfinding::directed::dijkstra::dijkstra;

const EXAMPLE1_FILE_PATH: &str = "example1.txt";
const EXAMPLE2_FILE_PATH: &str = "example2.txt";
const INPUT_FILE_PATH: &str = "input.txt";
const UP: i32 = 0;
const DOWN: i32 = 1;
const LEFT: i32 = 2;
const RIGHT: i32 = 3;
const MAX_STRAIGHT_STREAK_PART1: i32 = 3;
const MIN_STRAIGHT_STREAK_PART2: i32 = 4;
const MAX_STRAIGHT_STREAK_PART2: i32 = 10;

fn get_new_positions_part1(position: &Pos, rows: usize, cols: usize) -> Vec<Pos> {
    let row = position.0;
    let col = position.1;
    let latest_direction = position.2;
    let straight_streak = position.3;
    let mut new_pos = Vec::new();
    match latest_direction {
        UP => {
            if col > 0 {
                new_pos.push(Pos(row, col - 1, LEFT, 1));
            }
            if col < cols - 1 {
                new_pos.push(Pos(row, col + 1, RIGHT, 1));
            }
            if row > 0 && straight_streak < MAX_STRAIGHT_STREAK_PART1 {
                new_pos.push(Pos(row - 1, col, UP, straight_streak + 1));
            }
        }
        DOWN => {
            if col > 0 {
                new_pos.push(Pos(row, col - 1, LEFT, 1));
            }
            if col < cols - 1 {
                new_pos.push(Pos(row, col + 1, RIGHT, 1));
            }
            if row < rows - 1 && straight_streak < MAX_STRAIGHT_STREAK_PART1 {
                new_pos.push(Pos(row + 1, col, DOWN, straight_streak + 1));
            }
        }
        LEFT => {
            if row > 0 {
                new_pos.push(Pos(row - 1, col, UP, 1));
            }
            if row < rows - 1 {
                new_pos.push(Pos(row + 1, col, DOWN, 1));
            }
            if col > 0 && straight_streak < MAX_STRAIGHT_STREAK_PART1 {
                new_pos.push(Pos(row, col - 1, LEFT, straight_streak + 1));
            }
        }
        RIGHT => {
            if row > 0 {
                new_pos.push(Pos(row - 1, col, UP, 1));
            }
            if row < rows - 1 {
                new_pos.push(Pos(row + 1, col, DOWN, 1));
            }
            if col < cols - 1 && straight_streak < MAX_STRAIGHT_STREAK_PART1 {
                new_pos.push(Pos(row, col + 1, RIGHT, straight_streak + 1));
            }
        }
        _ => println!("Shouldn't get here..."),
    }

    new_pos
}

fn get_new_positions_part2(position: &Pos, rows: usize, cols: usize) -> Vec<Pos> {
    let row = position.0;
    let col = position.1;
    let latest_direction = position.2;
    let straight_streak = position.3;
    let mut new_pos = Vec::new();
    match latest_direction {
        UP => {
            if straight_streak >= MIN_STRAIGHT_STREAK_PART2 {
                if col > 0 {
                    new_pos.push(Pos(row, col - 1, LEFT, 1));
                }
                if col < cols - 1 {
                    new_pos.push(Pos(row, col + 1, RIGHT, 1));
                }
            }
            if row > 0 && straight_streak < MAX_STRAIGHT_STREAK_PART2 {
                new_pos.push(Pos(row - 1, col, UP, straight_streak + 1));
            }
        }
        DOWN => {
            if straight_streak >= MIN_STRAIGHT_STREAK_PART2 {
                if col > 0 {
                    new_pos.push(Pos(row, col - 1, LEFT, 1));
                }
                if col < cols - 1 {
                    new_pos.push(Pos(row, col + 1, RIGHT, 1));
                }
            }
            if row < rows - 1 && straight_streak < MAX_STRAIGHT_STREAK_PART2 {
                new_pos.push(Pos(row + 1, col, DOWN, straight_streak + 1));
            }
        }
        LEFT => {
            if straight_streak >= MIN_STRAIGHT_STREAK_PART2 {
                if row > 0 {
                    new_pos.push(Pos(row - 1, col, UP, 1));
                }
                if row < rows - 1 {
                    new_pos.push(Pos(row + 1, col, DOWN, 1));
                }
            }
            if col > 0 && straight_streak < MAX_STRAIGHT_STREAK_PART2 {
                new_pos.push(Pos(row, col - 1, LEFT, straight_streak + 1));
            }
        }
        RIGHT => {
            if straight_streak >= MIN_STRAIGHT_STREAK_PART2 || straight_streak == 0 {
                if row > 0 {
                    new_pos.push(Pos(row - 1, col, UP, 1));
                }
                if row < rows - 1 {
                    new_pos.push(Pos(row + 1, col, DOWN, 1));
                }
            }
            if col < cols - 1 && straight_streak < MAX_STRAIGHT_STREAK_PART2 {
                new_pos.push(Pos(row, col + 1, RIGHT, straight_streak + 1));
            }
        }
        _ => println!("Shouldn't get here..."),
    }

    new_pos
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, i32, i32);

fn get_2d_vector_from_multiline_string(content: &str) -> Vec<Vec<u32>> {
    content
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve_day17a(file_path: &str) -> u32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    let rows = matrix.len();
    let cols = matrix[0].len();

    let start = Pos(0, 0, RIGHT, 0);
    let mut best_dijkstra = u32::MAX;
    for direction in vec![RIGHT, DOWN] {
        for streak in 1..=MAX_STRAIGHT_STREAK_PART1 {
            println!(" > Trying Dijkstra with target: direction={direction}, streak={streak}");
            let dijkstra_result = dijkstra(
                &start,
                |p| {
                    (get_new_positions_part1(p, rows, cols)
                        .iter()
                        .map(|s| (s.clone(), matrix[s.0][s.1])))
                    .collect::<Vec<_>>()
                },
                |p| p.0 == rows - 1 && p.1 == cols - 1 && p.2 == direction && p.3 == streak,
            )
            .unwrap_or(((vec![]), u32::MAX))
            .1;
            if dijkstra_result == u32::MAX {
                println!(" >> No Dijkstra for target");
            } else if dijkstra_result < best_dijkstra {
                println!(" >> Found better Dijkstra! {dijkstra_result}");
                best_dijkstra = dijkstra_result;
            } else {
                println!(" >> Dijkstra is no better! {dijkstra_result}");
            }
        }
    }
    println!("Best Dijkstra = {best_dijkstra}");
    best_dijkstra
}

fn solve_day17b(file_path: &str) -> u32 {
    let content: String = fs::read_to_string(file_path).expect("Failed to read file content :/");
    let matrix = get_2d_vector_from_multiline_string(&content);
    let rows = matrix.len();
    let cols = matrix[0].len();

    let start = Pos(0, 0, RIGHT, 0);
    let mut best_dijkstra = u32::MAX;
    for direction in UP..=RIGHT {
        for streak in MIN_STRAIGHT_STREAK_PART2..=MAX_STRAIGHT_STREAK_PART2 {
            println!(" > Trying Dijkstra with target: direction={direction}, streak={streak}");
            let dijkstra_result = dijkstra(
                &start,
                |p| {
                    (get_new_positions_part2(p, rows, cols)
                        .iter()
                        .map(|s| (s.clone(), matrix[s.0][s.1])))
                    .collect::<Vec<_>>()
                },
                |p| p.0 == rows - 1 && p.1 == cols - 1 && p.2 == direction && p.3 == streak,
            )
            .unwrap_or(((vec![]), u32::MAX));

            if dijkstra_result.1 == u32::MAX {
                println!(" >> No Dijkstra for target");
            } else if dijkstra_result.1 < best_dijkstra {
                println!(" >> Found better Dijkstra! {}", dijkstra_result.1);
                best_dijkstra = dijkstra_result.1;
            } else {
                println!(" >> Dijkstra is no better! {}", dijkstra_result.1);
            }
        }
    }
    println!("Best Dijkstra = {best_dijkstra}");
    best_dijkstra
}

fn main() {
    let result = solve_day17b(INPUT_FILE_PATH);
    println!("Result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example1() {
        let result = solve_day17a(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 102);
    }

    #[test]
    fn check_example1_p2() {
        let result = solve_day17b(EXAMPLE1_FILE_PATH);
        assert_eq!(result, 94);
    }
    #[test]
    fn check_example2_p2() {
        let result = solve_day17b(EXAMPLE2_FILE_PATH);
        assert_eq!(result, 71);
    }
}
