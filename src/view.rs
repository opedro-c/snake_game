use crate::board::Board;

pub fn display_board(board: Board) {
    for i in 0..board.board.len() {
        for j in 0..board.board[i].len() {
            print!("{}", board.board[i][j]);
        }
        println!();
    }
}
