use std::path::{Path};

mod puzzles;

fn main() {
    
    let resources_directory = Path::new("./src/resources");
    let result1 = puzzles::day1::solve(resources_directory.join( "day1.txt"));
    let result2 = puzzles::day2::solve(resources_directory.join( "day2.txt"));

    println!("day 1 result {result1}");
    println!("day 2 result {result2}");
}
