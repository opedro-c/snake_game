use std::sync::mpsc::Receiver;
use crate::{board::{place_snake_in_the_board, Board, BoardCell}, keyboard_input::Directions, snake::Snake};

pub struct GameRunner {
    snake: Snake,
    board: Board,
    game_over: bool,
    direction_receiver: Receiver<Directions>,
    direction: Directions
}


impl GameRunner {

    pub fn new(snake: Snake, board: &Board, direction_receiver: Receiver<Directions>) -> GameRunner {
        GameRunner { snake, board: board.to_vec(), game_over: false, direction_receiver, direction: Directions::Up }
    }

    pub fn process_events(&mut self) {

        self.direction = self.direction_receiver.try_recv().unwrap_or(self.direction);
        let mut snake_head_next_position = self.snake.get_snake_next_position(self.direction);

        let board_cell_where_snake_head_will_be_positioned = &self.board[snake_head_next_position.row][snake_head_next_position.column];

        match board_cell_where_snake_head_will_be_positioned {
            BoardCell::Wall | BoardCell::Snake => {
                self.game_over = true;
            },
            BoardCell::Free => self.snake.update_body_position(&mut snake_head_next_position),
            BoardCell::Food => {
                self.snake.update_body_position(&mut snake_head_next_position);
                self.snake.grow();
            },
            &BoardCell::SnakeHead => panic!("The head is not supposed to be ahead of the head?!?!"),
        }

        place_snake_in_the_board(&mut self.board, &mut self.snake);
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
