use std::{borrow::Borrow, process::exit, sync::mpsc::Receiver};

use crate::{board::{Board, BoardCell}, keyboard_input::Directions, snake::Snake};

pub struct GameRunner {
    snake: Snake,
    board: Board,
    game_over: bool,
    direction_receiver: Receiver<Directions>
}


impl GameRunner {

    pub fn new(snake: Snake, board: Board, direction_receiver: Receiver<Directions>) -> GameRunner {
        GameRunner { snake, board, game_over: false, direction_receiver }
    }

    pub fn process_events(&mut self) {
        // let Ok(snake_head_position) = self.snake.get_head_position() else {
        //     panic!("Something went wrong while getting head position!")
        // };

        let snake_head_position =self.snake.get_head_position().expect("Something went wrong while getting head position!");

        let board_cell_where_snake_head_is_positioned = &self.board[snake_head_position.row][snake_head_position.column];
        let direction = self.direction_receiver.try_recv().unwrap_or(self.snake.direction);

        match board_cell_where_snake_head_is_positioned {
            BoardCell::Wall => {
                println!("Oops, you just banged your head in the wall! Game Over!");
                exit(0);
            },
            BoardCell::Free => self.snake.update_body_position(snake_head_position, direction),
            BoardCell::Food => {
                self.snake.add_body()
            },
            BoardCell::Snake => todo!(),
            BoardCell::SnakeHead => todo!(),

        }
    }
}
