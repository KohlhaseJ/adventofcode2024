use std::{char, collections::{HashMap, HashSet}, path::Path};

use super::filehelper;

pub fn solve(file_path: impl AsRef<Path>) -> String {

    let contents = filehelper::lines_from_file(file_path);
    let mut map = contents.iter()
                                .map(|s| s.chars().collect::<Vec<char>>())
                                .collect::<Vec<Vec<char>>>();

    let n = map.len();
    let m = map[0].len();
    let mut path = vec![vec!["".to_string(); m]; n];

    let ((mut x, mut y), mut direction) = find_current_pos(&map).expect("no current position found");
    path[x as usize][y as usize] = direction.to_string();

    let mut loop_positions: HashSet<(i32, i32)> = HashSet::new();
    
    while in_map(x, y, n, m) {
        map[x as usize][y as usize] = 'X';
        path[x as usize][y as usize].push(direction);
        
        let (new_x, new_y) = walk_straight(x, y, direction);
        if in_map(new_x, new_y, n, m) && map[new_x as usize][new_y as usize] == '#' {
            direction = turn_right(direction);
        } else {
            // TODO: loop detection broken
            if !loop_positions.contains(&(x, y)) && causes_loop(&map, (x, y), direction) {
                loop_positions.insert(walk_straight(x, y, direction));
            }
            
            x = new_x;
            y = new_y;
        }
    }

    let visited_count = map.iter().map(|row| row.iter().filter(|c| **c == 'X').count()).sum::<usize>();
    return format!("visited count {}, possible loop count {}", visited_count, loop_positions.len());
}

fn walk_straight (x: i32, y: i32, direction: char) -> (i32, i32) {
    let mut new_x = x.clone();
    let mut new_y = y.clone();

    if direction == '^' {
        new_x -= 1;
    }
    if direction == 'v' {
        new_x += 1;
    }
    if direction == '>' {
        new_y += 1;
    }
    if direction == '<' {
        new_y -= 1;
    }

    return (new_x, new_y);
}

fn turn_right (direction: char) -> char {
    if direction == '^' {
        return '>';
    }
    if direction == 'v' {
        return '<';
    }
    if direction == '>' {
        return 'v';
    }
    if direction == '<' {
        return '^';
    }
    return direction;
}

fn in_map (x: i32, y: i32, n: usize, m: usize) -> bool {
    return x >= 0 && x < n as i32 && y >= 0 && y < m as i32;
}

fn find_current_pos(map: &Vec<Vec<char>>) -> Option<((i32, i32), char)> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' || map[i][j] == 'v' || map[i][j] == '>' || map[i][j] == '<' {
                return Some(((i as i32, j as i32), map[i][j]));
            }
        }
    }
    return None;
}

fn causes_loop(map: &Vec<Vec<char>>, start_pos: (i32, i32), start_direction: char) -> bool {
    let n = map.len();
    let m = map[0].len();
    let (mut x, mut y) = start_pos;
    let mut direction = turn_right(start_direction);

    let (obstacle_x, obstacle_y) = walk_straight(x, y, start_direction);
    if !in_map(obstacle_x, obstacle_y, n, m) {
        return false;
    }

    let mut new_map = map.clone();
    new_map[obstacle_x as usize][obstacle_y as usize] = '#';
    let mut visited: HashMap<(i32, i32), String> = HashMap::new();

    while in_map(x, y, n, m) {
        if visited.get(&(x,y)).unwrap_or(&String::new()).contains(direction) {
            return true;
        }
        visited.entry((x, y)).or_default().push(direction);

        let (new_x, new_y) = walk_straight(x, y, direction);
        if in_map(new_x, new_y, n, m) && new_map[new_x as usize][new_y as usize] == '#' {
            direction = turn_right(direction);
        } else {
            x = new_x;
            y = new_y;
        }
    }

    false
}