use utils::read_lines;

static WIDTH: usize = 12;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Line {
    line: String
}

impl Line {

    fn new(line: String) -> Self {
        Self {
            line
        }
    }

    fn as_decimal(&self) -> isize {
        isize::from_str_radix(&*self.line, 2).unwrap()
    }

    fn char_at_column(&self, col: usize, character: char) -> bool {
        self.line.chars().nth(col).unwrap() == character
    }
}

fn calculate_life_support_rating() -> i32 {
    let lines: Vec<Line> = read_lines("resources/day_3.txt")
        .into_iter()
        .map(|line| Line::new(line))
        .collect();

    let oxygen_rating = calculate_rating(&lines, |ones_count, zeroes_count| {
        ones_count >= zeroes_count
    });

    let co2_rating = calculate_rating(&lines, |ones_count, zeroes_count| {
        zeroes_count > ones_count
    });

    (oxygen_rating * co2_rating) as i32
}

fn calculate_rating(lines: &Vec<Line>, filter: impl Fn(usize, usize) -> bool) -> isize {
    let mut lines = lines.clone();
    let mut filtered_lines = lines.clone();

    for i in 0..WIDTH {
        // partition by zero's and ones
        let (ones, zeroes): (Vec<&Line>, Vec<&Line>) = lines.iter()
            .partition(|line| line.char_at_column(i, '1'));

        // based on the predicate, keep only the lines with the most or least
        if filter(ones.len(), zeroes.len()) {
            // indicator to keep lines which contain ones
            filtered_lines.retain(|line| ones.contains(&line));
        } else {
            // indicator to keep lines which contain zeroes
            filtered_lines.retain(|line| zeroes.contains(&line));
        }

        // reduce set of candidates for next iteration
        lines.retain(|line | filtered_lines.contains(line));

        if filtered_lines.len() == 1 {
            break;
        }
    }

    filtered_lines.get(0).unwrap().as_decimal()
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate_life_support_rating() {
        println!("life_support_rating: {:?}", calculate_life_support_rating());
    }

    #[test]
    fn test_binary_shift() {
        let binary = "10111";
        let decimal = isize::from_str_radix(&binary, 2).unwrap();

        println!("bin: {}, dec: {:?}", binary, decimal);

        // bin  : 10101111
        // i<<0 : 00000001
        // ===  : 00000001
        // i<<1 : 00000010
        // i<<2 : 00000100
        // i<<3 : 00001000
        // i<<4 : 00010000
        // ===  : 00000000

        for i in (0..5).rev() {
            let dec = decimal & (1 << i);
            println!("bin: {:<12b}, dec: {:?}", dec, dec);
        }
    }
}
