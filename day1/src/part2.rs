use std::collections::HashMap;

use crate::{number_utils, part1};

pub fn execute_part2(path: &str) -> i32 {
    let input_file = "input.txt";
    let pairs = part1::execute_part1(input_file);
    let (first, second) = pairs;
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for &number in &first {
        let frequency = second.iter().filter(|&&x| x == number).count() as i32;
        counter.insert(number, frequency);
    }

    let res = number_utils::calculate_sum_product(counter);
    println!("Part 2 - Sum of number x frequency for each num: {}", res);
    res
}
