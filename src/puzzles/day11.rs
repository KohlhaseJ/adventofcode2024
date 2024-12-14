use std::path::Path;
use super::filehelper;
use rayon::prelude::*;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let content = filehelper::string_from_file(file_path);
    let numbers : Vec<usize> = content.split(" ").map(|str| str.parse::<usize>().unwrap()).collect();
    let blinked : Vec<usize> =  numbers.par_iter()
        .flat_map(|number| blink(*number, 0))
        .collect();

    let stone_count = blinked.len();
    return format!("stone count {}", stone_count);
}


fn blink(number: usize, depth: usize) -> Vec<usize> {

    if depth == 75 {
        return vec![number];
    }

    if number == 0 {
        return blink(1, depth + 1);
    }
    
    let number_of_digits = number.to_string().len();
    if number_of_digits % 2 == 0 {
        let size = number_of_digits / 2;
        let left = number.to_string()[0..size].parse::<usize>().unwrap();
        let right = number.to_string()[size..].parse::<usize>().unwrap();
        let mut blinked = blink(left, depth + 1);
        blinked.append(&mut blink(right, depth + 1));
        return blinked;
    }

    return blink(number * 2024, depth + 1);
}