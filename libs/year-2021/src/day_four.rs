use utils::read_lines;

#[derive(Debug, Clone)]
struct Piece {
    num: i32,
    chosen: bool,
}

impl Piece {
    fn new() -> Self {
        Self {
            num: 0,
            chosen: false
        }
    }
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    content: Vec<Piece>
}

impl Board {

    fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            content: vec![Piece::new(); w * h]
        }
    }

    fn piece_idx(&self, x: usize, y: usize) -> usize {
        utils::map_idx(self.width as i32, x as i32, y as i32)
    }

    fn piece_at_mut(&mut self, x: usize, y: usize) -> &mut Piece {
        let idx = self.piece_idx(x, y);
        self.content.get_mut(idx).unwrap()
    }

    fn piece_at(&self, x: usize, y: usize) -> &Piece {
        let idx = self.piece_idx(x, y);
        self.content.get(idx).unwrap()
    }

    fn intialize_row(&mut self, row: usize, line: &str) {
        let row_nums = split_by_whitespace(line);
        for (i, num) in row_nums.iter().enumerate() {
            let mut piece = self.piece_at_mut(i, row);

            piece.num = *num;
        }
    }

    fn mark_board(&mut self, num: i32) {
        self.content.iter_mut()
            .filter(|piece| piece.num == num)
            .for_each(|piece| piece.chosen = true);
    }

    fn has_row_marked(&self) -> bool {
        let mut full_row = true;

        for y in 0..self.height {
            full_row = true;

            for x in 0..self.width {
                if !self.piece_at(x, y).chosen {
                    full_row = false;
                    break;
                }
            }

            if full_row {
                break;
            }
        }

        full_row
    }

    fn has_column_marked(&self) -> bool {
        let mut full_column = true;
        for x in 0..self.width {
            full_column = true;

            for y in 0..self.height {
                if !self.piece_at(x, y).chosen {
                    full_column = false;
                    break;
                }
            }

            if full_column {
                break;
            }
        }

        full_column
    }

    fn bingo(&self) -> bool {
        self.has_row_marked() || self.has_column_marked()
    }
}

fn split_by_comma(line: &str) -> Vec<i32> {
    line
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn split_by_whitespace(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn bingo_baby() {
    let lines = read_lines("resources/day_4.txt");
    let random_nums = split_by_comma(&lines.get(0).unwrap());

    let mut boards = load_boards(lines);
    for chosen in random_nums {
        //println!("Playing bingo with: {:?}", chosen);

        let winning_boards: Vec<(i32, &mut Board)> = boards.iter_mut().filter_map(|board| {
            board.mark_board(chosen);

            if board.bingo() {
                Some((chosen, board))
            } else {
                None
            }
        }).collect();

        if winning_boards.len() > 0 {
            let (winning_num, board) = winning_boards.get(0).unwrap();
            let sum: i32 = board.content.iter()
                .filter(|piece| !piece.chosen)
                .map(|piece| piece.num)
                .sum();

            // Found winning board with number: 68, sum: 859, answer: 58412
            println!("Found winning board with number: {}, sum: {}, answer: {}",
                     winning_num, sum, sum * winning_num);
            break;
        }
    }
}

fn load_boards(lines: Vec<String>) -> Vec<Board> {
    let w = 5;
    let h = 5;

    let mut boards = Vec::new();
    let mut row = 0;

    for i in 1..lines.len() {
        let line = lines.get(i).unwrap();
        if line.is_empty() {
            boards.push(Board::new(w, h));
            row = 0;

            continue;
        }

        let size = &boards.len();
        boards.get_mut(*size - 1)
            .unwrap()
            .intialize_row(row, line);
        row += 1;
    }

    boards
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        bingo_baby();
    }
}
