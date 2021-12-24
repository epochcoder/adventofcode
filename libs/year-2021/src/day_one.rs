use std::cmp::Ordering;
use utils;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Depth(i32);

impl Depth {

    fn new(line: &String) -> Self {
        Depth(line.parse::<i32>().unwrap())
    }
}

impl PartialOrd<Depth> for Depth {
    fn partial_cmp(&self, other: &Depth) -> Option<Ordering> {
        if self.0 < other.0 {
            return Option::Some(Ordering::Less)
        } else if self.0 > other.0 {
            return Option::Some(Ordering::Greater)
        }

        Option::Some(Ordering::Equal)
    }
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn read_lines() -> Vec<Depth> {
    utils::read_lines("resources/day_1.txt")
        .iter()
        .map(|line | Depth::new(line))
        .collect()
}
/// To do this, count the number of times a depth measurement increases from the previous measurement.
/// (There is no measurement before the first measurement.)
fn count_increases(depths: Vec<Depth>) -> i32 {
    if depths.is_empty() {
        return 0
    }

    let mut counter = 0;
    let mut last: Option<&Depth> = Option::None;
    for dep in &depths {
        if last.is_none() {
            last = Option::Some(dep);
            continue
        }

        if dep > last.unwrap() {
            counter = counter + 1;
        }

        last = Option::Some(dep);
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("increases: {:?}", count_increases(read_lines()));
    }
}
