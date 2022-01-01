use utils::*;

fn fish_ages() {
    let mut ages: Vec<i32> = read_lines("resources/day_6.txt")
        .iter()
        .flat_map(|line| {
            line.split(",")
                .map(|age| age.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let days = 80;
    (1..=days).for_each(|day| {
        let new_fish = ages.iter().filter(|age| **age == 0).count();

        ages.iter_mut().for_each(|age| {
            *age -= 1;
            if *age < 0 {
                *age = 6
            }
        });

        (0..new_fish).for_each(|_| ages.push(8));
        //println!("Day: {}, Ages: {:?}", day, ages);
    });

    println!("all fish: {}", ages.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fish_ages();
    }
}
