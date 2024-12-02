use std::{collections::HashMap, path::Path};

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents: Vec<String> = filehelper::lines_from_file(file_path);

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .iter()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .map(|splitted| splitted.iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .map(|splitted| (splitted[0], splitted[1]))
        .unzip();

    left.sort();
    right.sort();

    let mut right_occurences: HashMap<i32, i32> = HashMap::new();
    for value in &right {
        *right_occurences.entry(*value).or_default() += 1;
    }

    let mut distance : i32 = 0;
    let mut similarity : i32 = 0;
    for i in 0..left.len() {
        let left_value = left[i];
        let right_value = right[i];

        distance += (left_value - right_value).abs();
        similarity += left_value * *right_occurences.entry(left_value).or_default();
    }

    return format!("distance {}, similarity {}", distance.to_string(), similarity.to_string());
}