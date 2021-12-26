pub mod scanner;

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_lines(file: &str) -> Vec<String> {
    let input = File::open(file.to_string()).unwrap();
    let reader = BufReader::new(input);

    reader.lines()
        .map(|line| line.unwrap())
        .collect()
}

/// Considers a map using row-first encoding since we have a single dimension vector
/// indexing for x and y will look like:
/// ```rust
/// let idx = (y * WIDTH) + x;
/// ```
/// the reciprocal (inverse) is calculated by:
/// ```rust
/// let x = index % WIDTH;
/// let y = index / WIDTH; // int division rounds down
/// ```
pub fn map_idx(width: i32, x: i32, y: i32) -> usize {
    ((y * width) + x) as usize
}
