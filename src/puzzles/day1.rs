use std::path::Path;

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents: Vec<String> = filehelper::lines_from_file(file_path);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in contents.iter() {
        let splitted : Vec<&str> = line.split("   ").collect();
        left.push(splitted[0].parse().unwrap());
        right.push(splitted[1].parse().unwrap());
    }
    left.sort();
    right.sort();

    let mut distance : i32 = 0;
    let mut similarity : i32 = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
        let count : i32 = right.iter().filter(|n: &&i32| **n == left[i]).count() as i32;
        similarity += count * left[i];
    }

    return format!("distance {}, similarity {}", distance.to_string(), similarity.to_string());
}