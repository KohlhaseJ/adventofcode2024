use std::path::Path;

use super::filehelper;

pub fn solve(_file_path: impl AsRef<Path>) -> String {
    let content = filehelper::string_from_file(_file_path);
    let matrix : Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let connected_regions = get_connected_regions(&matrix);
    let sum_of_prices : usize = connected_regions.iter().map(|(_region, connected_region)| {
        let area = calculate_area(connected_region);
        let perimeter = calculate_perimeter(connected_region);
        return area*perimeter;
    }).sum();


    return format!("sum of prices: {}", sum_of_prices);
}

fn get_connected_regions(matrix: &Vec<Vec<char>>) -> Vec<(char, Vec<(usize, usize)>)> {
    let mut connected_regions = Vec::new();
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    let n = matrix.len();
    let m = matrix[0].len();

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] {
                continue;
            }
            let region = matrix[i][j];
            let connected_region = depth_first_search(matrix, &mut visited, i as i32, j as i32, region);
            connected_regions.push((region, connected_region));
        }
    }

    return connected_regions;
}

fn depth_first_search(matrix: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: i32, col: i32, region: char) -> Vec<(usize, usize)> {
    let mut connected_region = Vec::new();
    if !is_safe(matrix, visited, row, col, region) {
        return connected_region;
    }

    visited[row as usize][col as usize] = true;
    connected_region.push((row as usize, col as usize));

    if matrix[row as usize][col as usize] == region {
        connected_region.append(&mut depth_first_search(matrix, visited, row+1, col, region));
        connected_region.append(&mut depth_first_search(matrix, visited, row-1, col, region));
        connected_region.append(&mut depth_first_search(matrix, visited, row, col+1, region));
        connected_region.append(&mut depth_first_search(matrix, visited, row, col-1, region));
    }
    return connected_region;
}

fn is_safe(matrix: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, row: i32, col: i32, region: char) -> bool {
    return row >= 0 && row < matrix.len() as i32 && col >= 0 && col < matrix[0].len() as i32 && !visited[row as usize][col as usize] && matrix[row as usize][col as usize] == region;
}

fn calculate_area(connected_region: &Vec<(usize, usize)>) -> usize {
    return connected_region.len();
}

fn calculate_perimeter(connected_region: &Vec<(usize, usize)>) -> usize {

    let mut perimeter = 0;
    for (row, col) in connected_region {
        let left: (i32, i32) = (row.clone() as i32, col.clone() as i32 - 1);
        let right: (i32, i32) = (row.clone() as i32, col.clone() as i32 + 1);
        let up: (i32, i32) = (row.clone() as i32 - 1, col.clone() as i32);
        let down: (i32, i32) = (row.clone() as i32 + 1, col.clone() as i32);

        for position in vec![left, right, up, down] {
            let in_connected_region = connected_region.iter()
                .filter(|pos| pos.0 == position.0 as usize && pos.1 == position.1 as usize)
                .count() > 0;
            if !in_connected_region {
                perimeter += 1;
            }
        }

    }
    return perimeter;
}