use std::path::Path;
use std::env;

mod puzzles;
fn main() {
    let day = env::args().nth(1).expect("missing argument 'day'")
                              .parse().expect("error parsing argument 'day' as number");

    let resources_directory = Path::new("./src/resources");
    let file_path = resources_directory.join(&format!("day{}.txt", day));

    let result = match day {
        1 => puzzles::day1::solve(file_path),
        2 => puzzles::day2::solve(file_path),
        3 => puzzles::day3::solve(file_path),
        4 => puzzles::day4::solve(file_path),
        5 => puzzles::day5::solve(file_path),
        _ => "day not implemented".to_string()
    };

    println!("day {} result: {}", day, result);
}
