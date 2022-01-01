#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    chars: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            chars: string.chars().collect(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor)
    }

    pub fn is_done(&self) -> bool {
        self.chars.len() == self.cursor
    }

    pub fn pop(&mut self) -> Option<&char> {
        match self.chars.get(self.cursor) {
            Some(ch) => {
                self.cursor += 1;
                Some(ch)
            }
            None => None,
        }
    }

    /// tries to take the specified character from the current cursor position
    pub fn take(&mut self, target: &char) -> bool {
        match self.peek() {
            Some(character) if target == character => {
                self.pop();
                true
            }
            _ => false,
        }
    }

    /// consumes the specified character from the scanner until take returns false
    pub fn consume(&mut self, target: &char) {
        loop {
            if !self.take(target) {
                break;
            }
        }
    }

    /// transforms a character into a value of T
    pub fn transform<T>(&mut self, callback: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.chars.get(self.cursor) {
            Some(character) => match callback(character) {
                Some(result) => {
                    self.cursor += 1;
                    Some(result)
                }
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Debug)]
    enum Test {
        A,
        B,
        C,
    }

    #[test]
    fn test_peek() {
        let scanner = Scanner::new("a");
        assert_eq!(Some(&'a'), scanner.peek());
        assert_eq!(false, scanner.is_done());
        assert_eq!(0, scanner.cursor());
    }

    #[test]
    fn test_pop() {
        let mut scanner = Scanner::new("a");
        assert_eq!(Some(&'a'), scanner.pop());
        assert_eq!(None, scanner.pop());
        assert_eq!(true, scanner.is_done());
        assert_eq!(1, scanner.cursor());
    }

    #[test]
    fn test_take() {
        let mut scanner = Scanner::new("a");
        assert_eq!(false, scanner.take(&'b'));
        assert_eq!(true, scanner.take(&'a'));
        assert_eq!(true, scanner.is_done());
    }

    #[test]
    fn test_transform() {
        let mut scanner = Scanner::new("abc");
        assert_eq!(
            Some(Test::A),
            scanner.transform(|character| match character {
                'a' => Some(Test::A),
                'b' => Some(Test::B),
                'c' => Some(Test::C),
                _ => None,
            })
        );
    }

    #[test]
    fn test_consume() {
        let mut scanner = Scanner::new("          a");
        assert_eq!(false, scanner.take(&'a'));
        scanner.consume(&' ');
        assert_eq!(true, scanner.take(&'a'));
    }
}
