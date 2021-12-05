use std::io;
use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::alloc::System;

fn day_1() -> () {
    let time = std::time::SystemTime::now();
    let input = File::open("day_1.txt".to_string()).unwrap();
    let reader = BufReader::new(input);

    let numbers: HashSet<i32> = reader.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let target_num = 2020;
    for num in &numbers {
        let find_num = target_num - num; // determine number we are looking for

        if numbers.contains(&find_num) {
            println!("[2] found target num: {}, val: {}", &find_num, num * &find_num);
            break;
        }
    }

    // how to do for three numbers (fixme: make this algorithmic)
    'outer: for num in &numbers {
        for num_1 in &numbers {
            let find_num = target_num - num - num_1;

            if numbers.contains(&find_num) {
                println!("[3] found target num: {}, val: {}", &find_num, num * num_1 * &find_num);
                break 'outer;
            }
        }
    }

    println!("completed in: {:?}", time.elapsed().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    mod day_1 {
        use super::*;

        #[test]
        fn test_day_one() {
            day_1()
        }
    }
}
