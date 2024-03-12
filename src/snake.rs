use std::borrow::BorrowMut;

use crate::keyboard_input::Directions;



struct Snake {
    head: bool,
    repr: SnakeBodyParts,
    direction: Directions,
    position: Position,
    body: Option<Box<Snake>>
}

enum SnakeBodyParts {
    HeadUp,
    HeadDown,
    HeadLeft,
    HeadRight,
    Body
}

impl Snake {
    pub fn update_body_position(&mut self, new_position: Position, direction: Directions) {
        let current_position = Position {row: self.position.row, column: self.position.column};
        if self.head {
            match direction {
                Directions::Up => {
                    self.repr = SnakeBodyParts::HeadUp;
                    self.position.row = self.position.row - 1;
                },
                Directions::Down => {
                    self.repr = SnakeBodyParts::HeadDown;
                    self.position.row = self.position.row + 1;
                },
                Directions::Left => {
                    self.repr = SnakeBodyParts::HeadLeft;
                    self.position.column = self.position.column - 1;
                },
                Directions::Right => {
                    self.repr = SnakeBodyParts::HeadRight;
                    self.position.column = self.position.column + 1;
                }
            };
        } else {
            self.position = new_position;
        }
        self.body.take().unwrap().update_body_position(current_position, direction);
        // self.direction = direction;
    }

    pub fn add_body(&mut self, new_body: Option<Box<Snake>>) {
        if let Some(mut body) = self.body.take() {
            body.add_body(new_body);
        } else {
            self.body = new_body;
        }
    }
}

impl Iterator for Snake {

    type Item = Option<Box<Snake>>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { Some(Self::Item::from(self.body.take())) }
}

struct Position {
    row: u8,
    column: u8
}
