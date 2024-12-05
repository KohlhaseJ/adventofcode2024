use std::{
    fs::{read_to_string, File},
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}

pub fn string_from_file(file_path: impl AsRef<Path>) -> String {
    return read_to_string(file_path).expect("error reading file content as string");
}