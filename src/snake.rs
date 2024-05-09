use std::collections::LinkedList;

use crate::{keyboard_input::Directions, settings::Settings};

pub struct Snake {
    body: LinkedList<Position>
}

impl Snake {

    pub fn new(settings: Settings) -> Snake {
        let mut body: LinkedList<Position> = LinkedList::new();
        body.push_front(Position { row: settings.board_height / 2, column: settings.board_width / 2 });
        Snake { body }
    }

    pub fn update_body_position(&mut self, new_position: Position, direction: Directions) {
        match direction {
                Directions::Up => {
                    self.body.front().unwrap().row = self.body.front().unwrap().row - 1;
                },
                Directions::Down => {
                    self.body.front().unwrap().row = self.body.front().unwrap().row + 1;
                },
                Directions::Left => {
                    self.body.front().unwrap().column = self.body.front().unwrap().column - 1;
                },
                Directions::Right => {
                    self.body.front().unwrap().column = self.body.front().unwrap().column + 1;
                }
            };
    }

    pub fn add_body(&mut self) {
        let mut last_snake_body = &self.body;
        loop {
            let mut body = self.next();
            if body.take().is_some() {
                last_snake_body = &body.expect("Not able to get snake body");
            } else {
                break;
            }
        }
        last_snake_body.take().unwrap().body = Option::Some(Box::from(Snake{ head: false, direction: self.direction, position: todo!(), body: todo!() }))
    }

    pub fn get_head_position(&self) -> Result<Position, &str> {
        if !self.head {
            Err("Not head, so cannot get head position")
        } else {
            Ok(self.position.clone())
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize
}
