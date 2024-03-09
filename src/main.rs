mod board;
mod view;
mod snake;
mod settings;
mod keyboard_input;

use std::sync::Arc;
use std::sync::Mutex;
use crate::settings::Settings;
use crate::keyboard_input::read_key_down_event;
use board::Board;
use keyboard_input::Directions;
use view::display_board;



fn main() {
    let direction = Arc::new(Mutex::new(Directions::Up));
    let direction_clone = direction.clone();
    let settings = Settings { board_width: 50, board_height: 25 };
    let board = Board::new(settings.board_height, settings.board_width);
    display_board(board);
    std::thread::spawn(move || read_key_down_event(direction_clone));

    // loop {
    //     let dir = direction.lock().unwrap();
    //     println!("{}", *dir);
    // }
}
