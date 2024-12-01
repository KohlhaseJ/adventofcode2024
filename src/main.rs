use std::path::{Path};

mod puzzles;

fn main() {
    
    let resources_directory = Path::new("./src/resources");
    let result = puzzles::day1::solve(resources_directory.join( "day1.txt"));
    println!("day 1 result {result}");
}
