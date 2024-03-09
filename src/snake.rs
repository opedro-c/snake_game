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
        if self.head {
            match direction {
                Directions::Up => {
                    self.repr = SnakeBodyParts::HeadUp;
                    todo!("Update positions on body parts");
                },
                Directions::Down => self.repr = SnakeBodyParts::HeadDown,
                Directions::Left => self.repr = SnakeBodyParts::HeadLeft,
                Directions::Right => self.repr = SnakeBodyParts::HeadRight,
            }

        }
        self.position = new_position;
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
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { todo!() }
}

struct Position {
    x: u8,
    y: u8
}
