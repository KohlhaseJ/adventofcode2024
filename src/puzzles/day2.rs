use std::path::Path;

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents: Vec<String> = filehelper::lines_from_file(file_path);

    let reports: Vec<Vec<i32>> = contents
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|splitted| splitted.iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .collect();

    let mut safe_count = 0;
    let mut damped_safe_count = 0;

    for report in reports {

        let (is_safe, last_index) = is_safe_report(&report);
        if is_safe {
            safe_count += 1;
            damped_safe_count += 1;
        } else {
            let (is_now_safe, _) = is_safe_report(&clone_and_remove(&report, last_index));
            if is_now_safe {
                damped_safe_count += 1;
            } else if last_index + 1 < report.len() {
                let (is_now_safe, _) = is_safe_report(&clone_and_remove(&report, last_index + 1));
                if is_now_safe {
                    damped_safe_count += 1;
                }
            }
        }
    }

    return format!("safe reports {}, damped safe reports {}", safe_count, damped_safe_count);
}

fn is_safe_report(report: &Vec<i32>) -> (bool, usize) {
    let mut is_safe : bool = true;
        let mut last_number = report[0];
        let mut last_index : usize = 0;
        let mut direction : &str = "";
        for i in 1..report.len() {
            let number = report[i];

            if number > last_number {
                if direction == "" {
                    direction = "UP";
                } else if direction != "UP" {
                    is_safe = false;
                }
                is_safe = is_safe && number - last_number <= 3;
            } else if number < last_number {
                if direction == "" {
                    direction = "DOWN";
                } else if direction != "DOWN" {
                    is_safe = false;
                }
                is_safe = is_safe && last_number - number <= 3;
            } else {
                is_safe = false;
            }

            if !is_safe {
                break;
            }
            last_number = number;
            last_index = i;
        }

        if is_safe {
            return (true, last_index);
        }
        return (false, last_index);
}

fn clone_and_remove(report: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut cloned = report.clone();
    cloned.remove(index);
    return cloned;
}