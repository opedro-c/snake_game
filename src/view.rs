use std::io::{Stdout, Write};
use termion::cursor;
use termion::raw::RawTerminal;
use crate::board::Board;

pub fn display_board(board: &Board, output: &mut RawTerminal<Stdout>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{}", board[i][j]);
        }
        println!();
        write!(output, "{}", cursor::Left(board[0].len().try_into().unwrap())).unwrap();
        output.flush().unwrap();
    }
}
