mod board;
mod view;
mod snake;
mod settings;
mod keyboard_input;

use std::sync::mpsc::channel;
use crate::settings::Settings;
use crate::keyboard_input::read_key_down_event;
use board::Board;
use keyboard_input::Directions;
use termion::cursor;
use termion::clear;
use std::io::stdout;
use termion::raw::IntoRawMode;
use std::io::Write;
use view::display_board;


fn main() {
    let (sender, receiver) = channel::<Directions>();
    let settings = Settings { board_width: 50, board_height: 25 };
    let mut stdout = stdout().into_raw_mode().unwrap();
    let board = Board::new(settings.board_height, settings.board_width);
    display_board(board);
    std::thread::spawn(move || read_key_down_event(sender));
    let mut dir = Directions::Up;
    loop {
        if let Ok(input_value) = receiver.try_recv() {
            dir = input_value;
        }
        write!(stdout, "{} {}", cursor::Goto(1,1), clear::All).unwrap();
        println!("{}", dir);
        stdout.flush().unwrap();
    }
}
