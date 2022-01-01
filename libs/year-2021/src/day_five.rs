use std::convert::TryFrom;

use utils::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&str> for Point {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value
            .split(',')
            .into_iter()
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        Ok(Point::new(
            split.get(0).unwrap().parse::<i32>().unwrap(),
            split.get(1).unwrap().parse::<i32>().unwrap(),
        ))
    }
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        // arrange into start and end since we are only given two random points
        let (start, end) = if p1.x < p2.x || p1.y < p2.y {
            (p1, p2)
        } else {
            (p2, p1)
        };

        Self { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// plots this line on the provided map
    /// if a point already existed, increase the intersection count
    fn plot(&self, width: usize, map: &mut Vec<u32>) {
        let mut mark = |x, y| {
            let idx = map_idx(width as i32, x, y);
            *map.get_mut(idx).unwrap() += 1;
        };

        if self.is_vertical() {
            for y in self.start.y..=self.end.y {
                mark(self.start.x, y);
            }
        } else if self.is_horizontal() {
            for x in self.start.x..=self.end.x {
                mark(x, self.start.y);
            }
        } else {
            // diagonal (45 only)
            let mut x = self.start.x;
            let mut y = self.start.y;

            loop {
                mark(x, y);

                if x < self.end.x {
                    x += 1;
                } else if x > self.end.x {
                    x -= 1;
                } else {
                    break;
                }

                if y < self.end.y {
                    y += 1;
                } else if y > self.end.y {
                    y -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

impl TryFrom<&str> for Line {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value
            .split("->")
            .into_iter()
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        Ok(Line::new(
            Point::try_from(*split.get(0).unwrap()).unwrap(),
            Point::try_from(*split.get(1).unwrap()).unwrap(),
        ))
    }
}

fn calculate() {
    let lines: Vec<Line> = read_lines("resources/day_5.txt")
        .iter()
        .map(|line| Line::try_from(line.as_str()).unwrap())
        .collect();

    let (w, h) = (1000, 1000);
    let mut map: Vec<u32> = vec![0; w * h];

    lines.into_iter().for_each(|line| line.plot(w, &mut map));

    let count_intersects = map.into_iter().filter(|int| *int >= 2 as u32).count();
    println!("intersects count: {}", count_intersects);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        calculate();
    }
}
