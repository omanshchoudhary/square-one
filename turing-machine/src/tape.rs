use std::collections::VecDeque;

use crate::types::Direction;

pub struct Tape {
    cells: VecDeque<char>,
    origin: i64, // logical coordinate sitting at physical index 0
    head: i64,   // infinite tape, hence negative indexes
    blank: char,
}

impl Tape {
    pub fn new(input: &str, blank: char) -> Self {
        Self {
            cells: input.chars().collect(),
            origin: 0,
            head: 0,
            blank,
        }
    }

    fn index(&self) -> i64 {
        self.head - self.origin
    }

    pub fn read(&self) -> char {
        let index = self.index();
        if index < 0 || index >= self.cells.len() as i64 {
            return self.blank;
        }
        self.cells[index as usize]
    }

    pub fn write(&mut self, symbol: char) {
        while self.index() < 0 {
            self.cells.push_front(self.blank);
            self.origin -= 1;
        }
        while self.index() >= self.cells.len() as i64 {
            self.cells.push_back(self.blank);
        }
        let index = self.index() as usize;
        self.cells[index] = symbol;
    }

    pub fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
        }
    }

    pub fn contents(&self) -> String {
        self.cells.iter().collect()
    }

    pub fn blank(&self) -> char {
        self.blank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tape(input: &str) -> Tape {
        Tape::new(input, '_')
    }

    #[test]
    fn reads_input_under_head() {
        assert_eq!(tape("101").read(), '1');
    }

    #[test]
    fn unvisited_cells_are_blank() {
        let mut t = tape("1");
        t.move_head(Direction::Right);
        assert_eq!(t.read(), '_');
        t.move_head(Direction::Left);
        t.move_head(Direction::Left);
        assert_eq!(t.read(), '_');
    }

    #[test]
    fn writes_left_of_origin() {
        let mut t = tape("101");
        t.move_head(Direction::Left);
        t.write('X');
        assert_eq!(t.read(), 'X');
        t.move_head(Direction::Right);
        assert_eq!(t.read(), '1');
    }

    #[test]
    fn writes_past_right_end() {
        let mut t = tape("1");
        t.move_head(Direction::Right);
        t.move_head(Direction::Right);
        t.write('X');
        assert_eq!(t.read(), 'X');
        t.move_head(Direction::Left);
        assert_eq!(t.read(), '_');
    }

    #[test]
    fn writes_to_empty_tape() {
        let mut t = tape("");
        t.write('X');
        assert_eq!(t.read(), 'X');
    }
}
