use crate::{keyboard_input::Directions, settings::Settings};

pub struct Snake {
    pub body: Vec<Position>
}

impl Snake {

    pub fn new(settings: Settings) -> Snake {
        let mut body: Vec<Position> = Vec::new();
        body.push(Position { row: settings.board_height / 2, column: settings.board_width / 2 });
        Snake { body }
    }

    pub fn update_body_position(&mut self, new_position: &Position) {
        let mut old_position = self.body.first().unwrap().clone();
        let mut aux: Position;

        for i in 0..self.body.len() {
            if i != 0 {
                aux = self.body[i].clone();
                self.body[i] = old_position.clone();
                old_position = aux;
            } else {
                self.body[0] = new_position.clone();
            }
        }
    }

    pub fn get_snake_next_position(&self, direction: Directions) -> Position {
        match direction {
            Directions::Up => Position { row: self.body.first().unwrap().row - 1, column: self.body.first().unwrap().column },
            Directions::Down => Position { row: self.body.first().unwrap().row + 1, column: self.body.first().unwrap().column },
            Directions::Left => Position { row: self.body.first().unwrap().row, column: self.body.first().unwrap().column - 1 },
            Directions::Right => Position { row: self.body.first().unwrap().row, column: self.body.first().unwrap().column + 1 }
        }
    }

    pub fn grow(&mut self) {
        self.body.push(self.body.last().unwrap().clone());
    }
}

#[derive(Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize
}
