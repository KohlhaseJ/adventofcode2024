use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}