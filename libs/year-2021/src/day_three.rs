use std::collections::HashMap;

use utils::read_lines;

static WIDTH: usize = 12;

#[derive(Debug)]
struct ColumnCount {
    ones: i32,
    zeros: i32,
}

impl ColumnCount {
    fn new() -> Self {
        Self { ones: 0, zeros: 0 }
    }

    fn update(&mut self, ch: &char) {
        match ch {
            '0' => self.zeros += 1,
            '1' => self.ones += 1,
            _ => {}
        }
    }

    fn get_most_significant(&self) -> char {
        if self.zeros > self.ones {
            '0'
        } else {
            '1'
        }
    }

    fn get_binary(column_counts: HashMap<usize, ColumnCount>) -> String {
        let counts: Vec<&ColumnCount> = (0..WIDTH)
            .into_iter()
            .map(|i| column_counts.get(&(i as usize)).unwrap())
            .collect();

        counts
            .iter()
            .map(|cc| cc.get_most_significant())
            .collect::<String>()
            .to_string()
    }
}

fn calculate_gamma_rate(bin: &String) -> isize {
    isize::from_str_radix(&*bin, 2).unwrap()
}

fn calculate_epsilon_rate(bin: &String) -> isize {
    let inverse: String = bin
        .chars()
        .map(|ch| if ch == '1' { '0' } else { '1' })
        .collect();

    isize::from_str_radix(&*inverse, 2).unwrap()
}

fn calculate_power_consumption() -> i32 {
    let mut columns: HashMap<usize, ColumnCount> = HashMap::with_capacity(WIDTH);

    read_lines("resources/day_3.txt").iter().for_each(|line| {
        for (idx, ch) in line.chars().enumerate() {
            let entry = columns.entry(idx).or_insert(ColumnCount::new());

            entry.update(&ch);

            //println!("line: {:?}, idx: {:?}, ch: {:?}, debug: {:?}", line, idx, ch, entry);
        }
    });

    let binary = ColumnCount::get_binary(columns);
    let gamma_rate = calculate_gamma_rate(&binary);
    let epsilon_rate = calculate_epsilon_rate(&binary);

    println!(
        "bin: {:?}, gamma: {:?}, epsilon: {:?}",
        &binary, gamma_rate, epsilon_rate
    );

    (gamma_rate * epsilon_rate) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("power consumption: {:?}", calculate_power_consumption());
    }
}
