use std::convert::TryFrom;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq, Eq)]
struct SubVec {
    direction: Direction,
    amount: i32
}

impl TryFrom<&String> for SubVec {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(up|down|forward)\\s+(\\d+)").unwrap();
        }

        if !RE.is_match(value.as_str()) {
            return Err("invalid input");
        }

        match RE.captures(value.as_str()) {
            Some(caps) => {
                let dir = caps.get(1).unwrap().as_str();
                let amount = caps.get(2).unwrap().as_str();

                Ok(SubVec::new(match dir {
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    "forward" => Direction::Forward,
                    _ => panic!("unknown direction matched")
                }, amount.parse::<i32>().unwrap()))
            }
            None => Err("could not get capturing groups")
        }
    }
}

impl SubVec {
    fn new(dir: Direction, amount: i32) -> Self {
        Self {
            direction: dir,
            amount
        }
    }
}

fn read_lines() -> Vec<SubVec> {
    utils::read_lines("resources/day_2.txt")
        .iter()
        .map(|line | SubVec::try_from(line))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect()
}

fn calculate_position() -> i32 {
    let mut depth: i32 = 0;
    let mut position: i32 = 0;

    for sub in read_lines() {
        match sub.direction {
            Direction::Up => depth -= sub.amount,
            Direction::Down => depth += sub.amount,
            Direction::Forward => position += sub.amount,
        }
    }

    println!("depth: {:?}, position: {:?}", depth, position);
    depth * position
}

fn calculate_position_aim() -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut position: i32 = 0;

    for sub in read_lines() {
        match sub.direction {
            Direction::Down => aim += sub.amount,
            Direction::Up => aim -= sub.amount,
            Direction::Forward => {
                position += sub.amount;
                depth += aim * sub.amount;
            },
        }
    }

    println!("depth: {:?}, position: {:?}", depth, position);
    depth * position
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_answer() {
        println!("{:?}", calculate_position());
    }

    #[test]
    fn get_answer_extra() {
        println!("{:?}", calculate_position_aim());
    }

    #[test]
    fn test_parse_good() {
        let result: Result<SubVec, &str> = SubVec::try_from(&"down 32".to_string());
        assert!(result.is_ok());

        let vector = result.unwrap();
        assert_eq!(Direction::Down, vector.direction);
        assert_eq!(32, vector.amount);
    }

    #[test]
    fn test_parse_bad() {
        let result: Result<SubVec, &str> = SubVec::try_from(&"blah 32".to_string());
        assert!(result.is_err());
        assert_eq!("invalid input", result.unwrap_err());
    }
}
