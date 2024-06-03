use std::sync::mpsc::Receiver;
use crate::{board::{add_food_to_the_board, free_board_cell, place_snake_in_the_board, Board, BoardCell}, keyboard_input::Directions, snake::Snake};

pub struct GameRunner<'a> {
    snake: Snake,
    board: &'a mut Board,
    game_over: bool,
    direction_receiver: Receiver<Directions>,
    direction: Directions
}


impl GameRunner<'_> {

    pub fn new(snake: Snake, board: &mut Board, direction_receiver: Receiver<Directions>) -> GameRunner {
        GameRunner { snake, board, game_over: false, direction_receiver, direction: Directions::Up }
    }

    pub fn process_events(&mut self) {

        let direction = self.direction_receiver.try_recv().unwrap_or(self.direction);
        self.direction = self.handle_input(direction);

        let mut snake_head_next_position = self.snake.get_snake_next_position(self.direction);

        let board_cell_where_snake_head_will_be_positioned = &self.board[snake_head_next_position.row][snake_head_next_position.column];

        let snake_tail_position = self.snake.body.last().unwrap().clone();

        match board_cell_where_snake_head_will_be_positioned {
            BoardCell::Wall | BoardCell::Snake => {
                self.game_over = true;
            },
            BoardCell::Free => self.snake.update_body_position(&mut snake_head_next_position),
            BoardCell::Food => {
                self.snake.update_body_position(&mut snake_head_next_position);
                self.snake.grow();
                add_food_to_the_board(self.board);
            },
            BoardCell::SnakeHead => panic!("The head is not supposed to be ahead of the head?!?!"),
        }

        free_board_cell(self.board, snake_tail_position);
        place_snake_in_the_board(&mut self.board, &mut self.snake);
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_board(&self) -> Board {
        self.board.to_vec()
    }

    pub fn handle_input(&self, new_direction: Directions) -> Directions {
        match (self.direction, new_direction) {
            (Directions::Up, Directions::Down)
            | (Directions::Down, Directions::Up)
            | (Directions::Left, Directions::Right)
            | (Directions::Right, Directions::Left) => self.direction,
            _ => new_direction
        }
    }

}
