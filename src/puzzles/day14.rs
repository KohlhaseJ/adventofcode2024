use std::path::Path;
use super::filehelper;

const N : i32 = 101;
const M : i32 = 103;
const SECONDS : usize = 100;

pub fn solve(file_path: impl AsRef<Path>) -> String {
    let mut map_string = String::new();
    let mut robots : Vec<((i32, i32), (i32, i32))>= filehelper::lines_from_file(file_path).iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let position = parts[0].replace("p=", "").split(",")
                                            .map(|v| v.parse::<i32>().unwrap())
                                            .collect::<Vec<i32>>();
            let velocity = parts[1].replace("v=", "").split(",")
                                            .map(|v| v.parse::<i32>().unwrap())
                                            .collect::<Vec<i32>>();

            ((position[0], position[1]), (velocity[0], velocity[1]))
        }).collect();

    for second in 0..SECONDS {
        for robot in robots.iter_mut() {
            move_robot(robot);
        }

        map_string.push_str(&format!("second: {}\n", second));
        map_string.push_str(&to_string(&robots));
        map_string.push_str("\n");
    }
    let resources_directory = Path::new("./src/resources");
    std::fs::write(resources_directory.join("output.txt"), &map_string).expect("Unable to write file");

    let map = get_robot_map(&robots);
    let safety_factor = calculate_safety_factor(&map);



    return format!("safe factor: {}", safety_factor);
}

fn move_robot(robot: &mut ((i32, i32), (i32, i32))) {

    let mut x = robot.0.0;
    let mut y = robot.0.1;
    let vx = robot.1.0;
    let vy = robot.1.1;

    x += vx;
    y += vy;

    if x < 0 {
        x = N + x
    }

    if x >= N {
        x = x - N
    }

    if y < 0 {
        y = M + y
    }

    if y >= M {
        y = y - M
    }

    robot.0 = (x, y);
}

fn get_robot_map(robots: &Vec<((i32, i32), (i32, i32))>) -> Vec<Vec<i32>> {

    let mut grid = vec![vec![0; N as usize]; M as usize];

    for robot in robots.iter() {
        grid[robot.0.1 as usize][robot.0.0 as usize] += 1;
    }

    return grid;
}

fn to_string(robots: &Vec<((i32, i32), (i32, i32))>) -> String {

    let grid = get_robot_map(robots);
    let mut result = String::new();

    for row in grid.iter() {
        for cell in row.iter() {
            result.push_str(&format!("{}", if *cell > 0 { "#" } else { "." }));
        }
        result.push_str("\n");
    }
    
    return result;
}

fn calculate_safety_factor(map: &Vec<Vec<i32>>) -> i32 {
    let n_center = (N as usize - 1) / 2;
    let m_center = (M as usize - 1) / 2;

    let top_left_quadrant = map.iter().take(m_center)
                                    .map(|row| row.iter().take(n_center).sum::<i32>()).sum::<i32>();
    let top_right_quadrant = map.iter().take(m_center)
                                    .map(|row| row.iter().skip(n_center + 1).sum::<i32>()).sum::<i32>();
    let bottom_left_quadrant = map.iter().skip(m_center + 1)
                                    .map(|row| row.iter().take(n_center).sum::<i32>()).sum::<i32>();
    let bottom_right_quadrant = map.iter().skip(m_center + 1)
                                    .map(|row| row.iter().skip(n_center + 1).sum::<i32>()).sum::<i32>();

    return top_left_quadrant * top_right_quadrant * bottom_left_quadrant * bottom_right_quadrant;
}