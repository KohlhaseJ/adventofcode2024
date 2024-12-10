use std::{collections::HashSet, path::Path};
use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let content = filehelper::string_from_file(file_path);
    let map = generate_map(&content);
    let trail_heads = find_trail_heads(&map);

    let mut trail_heads_score = 0;
    let mut trail_heads_rating = 0;
    for trail_head in trail_heads {
        let tails = get_trail_tails(&map, trail_head);
        trail_heads_score += tails.iter().cloned().collect::<HashSet<(usize, usize)>>().len();
        trail_heads_rating += tails.len();
    }

    return format!("sum of trail scores {}, sum of trail rating {}", trail_heads_score, trail_heads_rating);
}

fn generate_map(content: &String) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        map.push(row);
    }

    return map;
}

fn find_trail_heads(map: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut trail_heads: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                trail_heads.insert((i, j));
            }
        }
    }

    return trail_heads;
}

fn get_trail_tails(map: &Vec<Vec<usize>>, current_position: (usize, usize)) -> Vec<(usize, usize)> {

    let current_value = map[current_position.0][current_position.1];
    if current_value == 9 {
        let mut tails = Vec::new();
        tails.push((current_position.0, current_position.1));
        return tails;
    }
    
    let left_position: (i32, i32) = (current_position.0 as i32, current_position.1 as i32 - 1);
    let right_position: (i32, i32) = (current_position.0 as i32, current_position.1 as i32 + 1);
    let up_position: (i32, i32) = (current_position.0 as i32 - 1, current_position.1 as i32);
    let down_position: (i32, i32) = (current_position.0 as i32 + 1, current_position.1 as i32);

    let mut tails: Vec<(usize, usize)> = Vec::new();
    if in_map(left_position, map) && map[left_position.0 as usize][left_position.1 as usize] == current_value + 1 {
        tails.extend(&get_trail_tails(map, (left_position.0 as usize, left_position.1 as usize)));
    }

    if in_map(right_position, map) && map[right_position.0 as usize][right_position.1 as usize] == current_value + 1 {
        tails.extend(&get_trail_tails(map, (right_position.0 as usize, right_position.1 as usize)));
    }

    if in_map(up_position, map) && map[up_position.0 as usize][up_position.1 as usize] == current_value + 1 {
        tails.extend(&get_trail_tails(map, (up_position.0 as usize, up_position.1 as usize)));
    }

    if in_map(down_position, map) && map[down_position.0 as usize][down_position.1 as usize] == current_value + 1 {
        tails.extend(&get_trail_tails(map, (down_position.0 as usize, down_position.1 as usize)));
    }

    return tails;
}

fn in_map(position: (i32, i32), map: &Vec<Vec<usize>>) -> bool {
    return position.0 >= 0 && position.0 < map.len() as i32 && position.1 >= 0 && position.1 < map[0].len() as i32;
}