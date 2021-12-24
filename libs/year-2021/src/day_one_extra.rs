use utils;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Depth(i32);

impl Depth {
    fn from_string(line: &String) -> Self {
        Depth(line.parse::<i32>().unwrap())
    }
}

fn read_lines() -> Vec<Depth> {
    utils::read_lines("resources/day_1.txt")
        .iter()
        .map(|line | Depth::from_string(line))
        .collect()
}

fn sum_depths(depths: &Vec<&Depth>) -> i32 {
    depths.into_iter()
        .map(|d| d.0)
        .reduce(|a, b | {
            a + b
        })
        .unwrap()
}

/// Start by comparing the first and second three-measurement windows.
/// The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607.
/// The second window is marked B (200, 208, 210); its sum is 618.
/// The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.
//
// Your goal now is to count the number of times the sum of measurements in this sliding window increases from the previous sum.
// So, compare A with B, then compare B with C, then C with D, and so on.
// Stop when there aren't enough measurements left to create a new three-measurement sum.
fn count_increases(depths: Vec<Depth>) -> i32 {
    if depths.is_empty() {
        return 0
    }

    let win_size = 3;

    let mut counter = 0;
    let mut last_win: Option<Vec<&Depth>> = Option::None;
    let mut curr_win: Vec<&Depth> = Vec::new();

    for i in 0..depths.len() {
        if i + win_size <= depths.len() {
            curr_win.clear();
            for x in i..(i + win_size) {
                curr_win.push(depths.get(x).unwrap());
            }
        }

        let sum_last: i32 = match &last_win {
            Some(stack) => {
                sum_depths(stack)
            },
            None => 0
        };

        let sum_curr = sum_depths(&curr_win);
        if sum_last > 0 && sum_curr > sum_last {
            counter = counter + 1;
        }

        last_win = Option::Some(curr_win.clone());
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
