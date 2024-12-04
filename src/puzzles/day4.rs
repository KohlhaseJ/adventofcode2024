use std::{collections::HashMap, path::Path};

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents = filehelper::lines_from_file(file_path);
    let rotated = rotate(&contents);
    let main_diagonals = main_diagonals(&contents);
    let anti_diagonals = anti_diagonals(&contents);

    let count = contents.iter().map(|line| count_line_matches(line)).sum::<i32>();
    let rotated_count = rotated.iter().map(|line| count_line_matches(line)).sum::<i32>();
    let main_diagonals_count = main_diagonals.iter().map(|line| count_line_matches(line)).sum::<i32>();
    let anti_diagonals_count = anti_diagonals.iter().map(|line| count_line_matches(line)).sum::<i32>();

    let xmas_count = count + rotated_count + main_diagonals_count + anti_diagonals_count;
    let x_mas_count = count_x_mas(&contents);

    return format!("xmas count {}, x-mas count {}", xmas_count, x_mas_count);
}

fn rotate(matrix: &Vec<String>) -> Vec<String> {

    let mut rotated: Vec<String> = Vec::new();

    for i in 0..matrix[0].len() {
        let mut new_line : String = "".to_string();
        for j in 0..matrix.len() {
            new_line.insert(0,matrix[j].chars().nth(i).unwrap());
        }
        rotated.push(new_line);
    }

    return rotated;
}

fn main_diagonals(matrix: &Vec<String>) -> Vec<String> {
    let mut main_diagonals: HashMap<i32, String> = HashMap::new();
    let n = matrix.len();
    assert!(matrix.len() == matrix[0].len());
      
    for i in 0..n {  
        for j in 0..n {  
            let diff = i as i32 - j as i32;  
            main_diagonals.entry(diff).or_default().push(matrix[i].chars().nth(j).unwrap());  
        }  
    }
    return main_diagonals.iter().map(|(_, v)| v.clone()).collect();
}



fn anti_diagonals(matrix: &Vec<String>) -> Vec<String> {
    let mut anti_diagonals: HashMap<i32, String> = HashMap::new();
    let n = matrix.len();
    assert!(matrix.len() == matrix[0].len());
      
    for i in 0..n {  
        for j in 0..n {  
            let sum = i + j;  
            anti_diagonals.entry(sum as i32).or_default().push(matrix[i].chars().nth(j).unwrap());  
        }  
    } 
    return anti_diagonals.iter().map(|(_, v)| v.clone()).collect();
}

fn count_line_matches(line : &str) -> i32 {
    return (line.matches("XMAS").count() + line.matches("SAMX").count()).try_into().unwrap();
}

fn count_x_mas(content: &Vec<String>) -> usize {
    let mut x_mas_count : usize = 0;
    for row in 1..content.len()-1 {
        for col in 1..content[row].len()-1 {
            let current = content[row].chars().nth(col).unwrap();
            if current != 'A' {
                continue;
            }

            let top_left = content[row-1].chars().nth(col-1).unwrap();
            let top_right = content[row-1].chars().nth(col+1).unwrap();
            let bottom_left = content[row+1].chars().nth(col-1).unwrap();
            let bottom_right = content[row+1].chars().nth(col+1).unwrap();

            if ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
                && ((top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M')) {
                x_mas_count += 1;
            }
        }
    }
    return x_mas_count;
} 