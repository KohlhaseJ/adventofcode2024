use std::path::Path;
use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {
    let content = filehelper::string_from_file(file_path);
    let (mut map, movements) = split_map_and_movements(&content);
    
    let mut current_position = map.iter().enumerate().find_map(|(y, row)| {
        row.iter().position(|&c| c == '@').map(|x| (x, y))
    }).unwrap();

    for movement in movements {
        move_robot(&mut map, movement, &mut current_position);
    }

    let box_positions: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == 'O' {
                Some((x, y))
            }
            else {
                None
            }
        })
    }).collect();

    let score = box_positions.iter().map(|(x, y)| 100 * y + x).sum::<usize>();

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    return format!("GPS sum: {}", score);
}

fn move_robot(map: &mut Vec<Vec<char>>, movement: char, current_position: &mut (usize, usize)) {

    let (new_position, is_valid_move) = try_get_next_position(*current_position, movement);
    if !is_valid_move {
        return;
    }

    if map[new_position.1][new_position.0] == '#' {
        return;
    }

    if map[new_position.1][new_position.0] == 'O' {
        if !move_box(map, movement, new_position) {
            return;
        }
    }

    map[current_position.1][current_position.0] = '.';
    map[new_position.1][new_position.0] = '@';
    current_position.0 = new_position.0;
    current_position.1 = new_position.1;
}

fn move_box(map: &mut Vec<Vec<char>>, movement: char, position: (usize, usize)) -> bool {
    let (new_position, is_valid_move) = try_get_next_position(position, movement);
    if !is_valid_move {
        return false;
    }

    if map[new_position.1][new_position.0] == '#' {
        return false;
    }

    if map[new_position.1][new_position.0] == 'O' {
        let box_moved = move_box(map, movement, new_position);
        if !box_moved {
            return false;
        }
    }

    map[position.1][position.0] = '.';
    map[new_position.1][new_position.0] = 'O';

    return true;
}

fn split_map_and_movements(content: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();

    let mut in_movement_section = false;
    for line in content.lines() {
        if line.trim().is_empty() {
            in_movement_section = true;
            continue;
        }

        if in_movement_section {
            movements.extend(line.chars());
        }
        else {
            map.push(line.chars().collect());
        }
    }

    return (map, movements);
}

fn try_get_next_position(current_position: (usize, usize), movement: char) -> ((usize, usize), bool) {
    let mut new_position = current_position.clone();
    let mut is_valid_move = true;
    match movement {
        '^' => {
            if current_position.1 == 0 {
                is_valid_move = false;
            }
            else {
                new_position.1 -= 1;
            }
        },
        'v' => {
            if current_position.1 == 0 {
                is_valid_move = false;
            }
            else {
                new_position.1 += 1;
            }
        },
        '<' => {
            if current_position.0 == 0 {
                is_valid_move = false;
            }
            else {
                new_position.0 -= 1;
            }
        },
        '>' => {
            if current_position.0 == 0 {
                is_valid_move = false;
            }
            else {
                new_position.0 += 1;
            }
        },
        _ => {
            is_valid_move = false;
        }
    }

    return (new_position, is_valid_move);
}