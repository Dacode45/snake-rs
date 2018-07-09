use super::cell::*;
use super::direction::*;

pub struct Snake {
    dir: Direction,
    body: Vec<Cell>,
}

impl Snake {

    pub fn new(start: Cell, len: usize, direction: Direction) -> Snake {
        let mut body = Vec::new();
        for _ in 0..len {
            body.push(start);
        }
        Snake {
            dir: direction,
            body: body
        }
    }

    pub fn update(&mut self) {
        let next = self.head().next(&self.dir);
        self.body.push(next);
        self.body = self.body[1..].to_vec();
    }

    pub fn eat(&mut self) {
        let cell = *self.head();
        self.body.push(cell);
    }

    pub fn head(&self) -> &Cell {
        self.body.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
use super::Snake;
use super::Cell;
use super::Direction;

    #[test]
    fn snake_create() {
        let snake = Snake::new(Cell::new(5, 5), 5, Direction::Right);
        assert_eq!(snake.body.len(), 5);
        assert_eq!(snake.dir, Direction::Right);
    }

    #[test]
    fn snake_update() {
        let mut snake = Snake::new(Cell::new(5, 5), 5, Direction::Right);
        snake.update();
        assert_eq!(*snake.head(), Cell::new(6, 5));
        assert_eq!(snake.body.len(), 5);

        // do 4 more updates
        for _ in 0..4 {
            snake.update();
        }

        assert_eq!(snake.body, vec![
            Cell::new(6, 5),
            Cell::new(7, 5),
            Cell::new(8, 5),
            Cell::new(9, 5),
            Cell::new(10, 5),
        ]);
    } 

    #[test]
    fn snake_eat() {
        let mut snake = Snake::new(Cell::new(5, 5), 5, Direction::Right);
        snake.update();
        assert_eq!(*snake.head(), Cell::new(6, 5));
        assert_eq!(snake.body.len(), 5);

        snake.eat();
        assert_eq!(*snake.head(), Cell::new(6, 5));
        assert_eq!(snake.body[snake.body.len() - 2], Cell::new(6, 5));
        assert_eq!(snake.body.len(), 6);
    } 
}