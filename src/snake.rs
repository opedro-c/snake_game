use std::{collections::LinkedList, mem::swap};

use crate::{keyboard_input::Directions, settings::Settings};

pub struct Snake {
    pub body: LinkedList<Position>
}

impl Snake {

    pub fn new(settings: Settings) -> Snake {
        let mut body: LinkedList<Position> = LinkedList::new();
        body.push_front(Position { row: settings.board_height / 2, column: settings.board_width / 2 });
        Snake { body }
    }

    pub fn update_body_position(&mut self, new_position: Position) {
        let old_position = self.body.front();

        for (i, mut position) in self.body.iter().enumerate() {
            if i != 0 {
                swap(&mut position, &mut old_position.unwrap());
            } else {
                swap(&mut position, &mut &new_position);
            }
        }
    }

    pub fn get_snake_next_position(&self, direction: Directions) -> Position {
        match direction {
            Directions::Up => Position { row: self.body.front().unwrap().row - 1, column: self.body.front().unwrap().column },
            Directions::Down => Position { row: self.body.front().unwrap().row + 1, column: self.body.front().unwrap().column },
            Directions::Left => Position { row: self.body.front().unwrap().row, column: self.body.front().unwrap().column - 1 },
            Directions::Right => Position { row: self.body.front().unwrap().row, column: self.body.front().unwrap().column + 1 }
        }
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.body.back().unwrap().clone());
    }
}

#[derive(Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize
}
