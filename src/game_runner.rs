use std::sync::mpsc::Receiver;
use crate::{board::{place_snake_in_the_board, Board, BoardCell}, keyboard_input::Directions, snake::Snake};

pub struct GameRunner<'a> {
    snake: &'a mut Snake,
    board: Board,
    game_over: bool,
    direction_receiver: Receiver<Directions>,
    direction: Directions
}


impl GameRunner<'_> {

    pub fn new<'a>(snake: &'a mut Snake, board: &Board, direction_receiver: Receiver<Directions>) -> GameRunner<'a> {
        GameRunner { snake, board: board.to_vec(), game_over: false, direction_receiver, direction: Directions::Up }
    }

    pub fn process_events(&mut self) {

        let direction = self.direction_receiver.try_recv().unwrap_or(self.direction);
        let snake_head_next_position = self.snake.get_snake_next_position(direction);

        let board_cell_where_snake_head_will_be_positioned = &self.board[snake_head_next_position.row][snake_head_next_position.column];

        match board_cell_where_snake_head_will_be_positioned {
            BoardCell::Wall | BoardCell::Snake => {
                println!("Oops, you just banged your head in the wall! Game Over!");
                self.game_over = true;
            },
            BoardCell::Free => self.snake.update_body_position(snake_head_next_position),
            BoardCell::Food => {
                self.snake.update_body_position(snake_head_next_position);
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
