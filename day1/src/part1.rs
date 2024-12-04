use crate::file_utils;
use crate::number_utils;

/// Executes Part 1 logic and prints the result.
pub fn execute_part1(path: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = match file_utils::read_lines(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return (vec![0], vec![0]);
        }
    };

    let (mut first_list, mut second_list) = match number_utils::parse_numbers(lines) {
        Ok((first, second)) => (first, second),
        Err(e) => {
            eprintln!("Error parsing numbers: {}", e);
            return (vec![0], vec![0]);
        }
    };

    first_list.sort();
    second_list.sort();
    let sum_difference = number_utils::calculate_sum_difference(&first_list, &second_list);

    println!("Part 1 - Sum of absolute differences: {}", sum_difference);
    (first_list, second_list)
}
