use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
};

// Read file and receive vector of strings
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}