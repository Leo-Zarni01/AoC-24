mod file_utils;
mod number_utils;
mod part1;
mod part2;

fn main() {
    let input_file = "input.txt";
    println!("Running Part 1...");
    part1::execute_part1(input_file);

    println!("Running Part 2...");
    part2::execute_part2(input_file);
}
