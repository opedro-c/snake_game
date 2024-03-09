use rand::Rng;

#[derive(Debug)]
pub struct Board {
    rows: u8,
    columns: u8,
    pub board: Vec<Vec<String>>
}

impl Board {
    pub fn new(rows: u8, columns: u8) -> Self {
        let mut board: Vec<Vec<String>> = vec![vec![" ".to_string(); columns.into()]; rows.into()];

        for i in 0..rows as usize {
            board[i][0] = "#".to_string();
            board[i][(columns - 1) as usize] = "#".to_string();
        }

        for i in 0..columns as usize {
            board[0][i] = "#".to_string();
            board[(rows - 1) as usize][i] = "#".to_string();
        }

        board[rand::thread_rng().gen_range(1..rows - 1) as usize][rand::thread_rng().gen_range(1..columns - 1) as usize] = "*".to_string();
        Board { rows, columns, board }
    }
}
