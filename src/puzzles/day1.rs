use std::path::Path;

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

    let distance : i32 = left.iter()
                             .zip(right.iter())
                             .map(|(l, r)| (l-r).abs())
                             .sum();

    let similarity : i32 = left.iter()
                               .map(|l| right.iter().filter(|r| **r == *l).count() as i32 * l)
                               .sum();

    return format!("distance {}, similarity {}", distance.to_string(), similarity.to_string());
}