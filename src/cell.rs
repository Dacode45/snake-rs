use super::direction::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell{ x, y }
    }

    pub fn next(&self, dir: &Direction) -> Cell {
        match dir {
            Direction::Up => return Cell{x: self.x, y: self.y - 1},
            Direction::Down => return Cell{x: self.x, y: self.y + 1},
            Direction::Left => return Cell{x: self.x - 1, y: self.y},
            Direction::Right => return Cell{x: self.x + 1, y: self.y},
        }
    }
}