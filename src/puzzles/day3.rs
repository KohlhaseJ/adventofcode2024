use std::{collections::HashSet, path::Path};
use regex::Regex;

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let content= &filehelper::string_from_file(file_path);

    let multiplication_sum : i32 = extract_multiplications(content).iter()
                        .map(|command| solve_multiplication(command))
                        .sum();
    
    let mut conditioned_content = content.to_string().clone();
    let skip_indices = find_skip_indices(content);
    for skip_tuple in skip_indices.iter().rev() {
        conditioned_content.replace_range(skip_tuple.0..skip_tuple.1, "");
    }
    let conditioned_multiplication_sum : i32 = extract_multiplications(&conditioned_content).iter()
                        .map(|command| solve_multiplication(command))
                        .sum();

    return format!("multiplication sum {}, conditioned multiplication sum {}", multiplication_sum, conditioned_multiplication_sum);
}

fn extract_multiplications(line: &str) -> HashSet<&str> {
    let regex = Regex::new(
            r"mul\(\d{1,3},\d{1,3}\)"
        ).unwrap();
    return regex.find_iter(line).map(|mat| mat.as_str()).collect()
}

fn find_skip_indices(line: &str) -> Vec<(usize, usize)> {
    let regex = Regex::new(
        r"don't\(\).*?do\(\)"
    ).unwrap();
    return regex.find_iter(line).map(|mat| (mat.start(), mat.end())).collect()
}

fn solve_multiplication(command: &str) -> i32 {
    let numbers = command.replace("mul(", "").replace(")", "").split(",")
                                         .collect::<Vec<&str>>().iter()
                                         .map(|s| s.parse().unwrap())
                                         .collect::<Vec<i32>>();

    assert!(numbers.len() == 2);
    return numbers[0] * numbers[1];
}