use std::collections::HashMap;

/// Parses each line of input into two lists of integers.
///
/// # Arguments
/// - `lines`: An iterator over lines, where each line contains two space-separated values.
///
/// # Returns
/// - `Result<(Vec<i32>, Vec<i32>), String>`:
///   - On success: A tuple of two vectors, one for the first numbers and one for the second numbers.
///   - On failure: A string error message if a line doesn't contain exactly two numbers or if any number fails to parse.
///
pub fn parse_numbers<I>(lines: I) -> Result<(Vec<i32>, Vec<i32>), String>
where
    I: Iterator<Item = String>,
{
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != 2 {
            return Err(format!("Invalid line format: {}", line));
        }

        let first_number = numbers[0]
            .parse::<i32>()
            .map_err(|_| format!("Failed to parse number: {}", numbers[0]))?;
        let second_number = numbers[1]
            .parse::<i32>()
            .map_err(|_| format!("Failed to parse number: {}", numbers[1]))?;

        first_list.push(first_number);
        second_list.push(second_number);
    }

    Ok((first_list, second_list))
}

/// Calculates the sum of absolute differences between corresponding elements of two lists.
///
/// # Arguments
/// - `first_list`: A slice of integers.
/// - `second_list`: A slice of integers.
///
/// # Returns
/// - `i32`: The sum of absolute differences between the corresponding elements of the two lists.
///
pub fn calculate_sum_difference(first_list: &[i32], second_list: &[i32]) -> i32 {
    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

/// Calculates the sum of the product of keys and their corresponding frequencies from a frequency map.
///
/// # Arguments
/// - `frequency_counter`: A `HashMap<i32, i32>` where keys are integers and values are their frequencies.
///
/// # Returns
/// - `i32`: The sum of the product of each key and its frequency.
///
pub fn calculate_sum_product(frequency_counter: HashMap<i32, i32>) -> i32 {
    let mut sum_product = 0;
    for each_pair in frequency_counter {
        let (key, freq) = each_pair;
        let combined = key * freq;
        sum_product += combined;
    }
    sum_product
}
