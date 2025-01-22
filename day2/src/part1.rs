use crate::file_utils;
/// Executes Part 1 logic and prints the result.
pub fn execute_part1(path: &str) {
    let lines = match file_utils::read_lines(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut safe_counter = 0;
    let mut polarity_counter = 0;
    let mut status: &str;
    for line in lines {
        let report: Vec<&str> = line.split_whitespace().collect();
        println!("Report is: {:?}", report);
        for index in 0..report.len() - 1 {
            let next_index = index + 1;

            let curr_val = report[index]
                .parse::<i32>()
                .map_err(|_| format!("Failed to parse number: {}", report[index]))
                .unwrap();

            let neighbor_value = report[next_index]
                .parse::<i32>()
                .map_err(|_| format!("Failed to parse number: {}", report[next_index]))
                .unwrap();

            let difference = (curr_val - neighbor_value).abs();
            if difference < 1 || difference > 3 {
                continue;
            }
            if neighbor_value > curr_val {
                if polarity_counter == 0 {
                    status = "increasing";
                    safe_counter += 1;
                    polarity_counter += 1;
                    continue;
                } else {
                    safe_counter += 1;
                }
            } else {
                if polarity_counter == 0 {
                    status = "decreasing";
                    safe_counter += 1;
                    polarity_counter += 1;
                    continue;
                } else {
                    safe_counter += 1;
                }
            }
        }
        println!();
    }
}
