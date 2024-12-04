use std::collections::HashMap;

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

pub fn calculate_sum_difference(first_list: &[i32], second_list: &[i32]) -> i32 {
    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn calculate_sum_product(frequency_counter: HashMap<i32, i32>) -> i32 {
    let mut sum_product = 0;
    for each_pair in frequency_counter {
        let (key, freq) = each_pair;
        let combined = key * freq;
        sum_product += combined;
    }
    sum_product
}
