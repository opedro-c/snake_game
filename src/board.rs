use std::fmt;
use rand::Rng;

#[derive(Clone)]
pub enum BoardCell {
    Wall,
    Free,
    Food,
}

pub type Board = Vec<Vec<BoardCell>>;

pub fn new_board(rows: u8, columns: u8) -> Board {
    let mut board: Board = vec![vec![BoardCell::Free; columns.into()]; rows.into()];

    for i in 0..rows as usize {
        board[i][0] = BoardCell::Wall;
        board[i][(columns - 1) as usize] = BoardCell::Wall;
    }

    for i in 0..columns as usize {
        board[0][i] = BoardCell::Wall;
        board[(rows - 1) as usize][i] = BoardCell::Wall;
    }

    board[rand::thread_rng().gen_range(1..rows - 1) as usize][rand::thread_rng().gen_range(1..columns - 1) as usize] = BoardCell::Food;
    board
}

impl fmt::Display for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardCell::Wall => write!(f, "#"),
            BoardCell::Free => write!(f, " "),
            BoardCell::Food => write!(f, "ó"),
        }
    }
}
