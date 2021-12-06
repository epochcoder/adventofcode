use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_lines(file: &str) -> Vec<String> {
    let input = File::open(file.to_string()).unwrap();
    let reader = BufReader::new(input);

    reader.lines()
        .map(|line| line.unwrap())
        .collect()
}
