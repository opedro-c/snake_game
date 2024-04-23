mod board;
mod view;
mod snake;
mod settings;
mod keyboard_input;

use std::sync::mpsc::channel;
use crate::settings::Settings;
use crate::keyboard_input::read_key_down_event;
use board::new_board;
use keyboard_input::Directions;
use std::io::stdout;
use termion::raw::IntoRawMode;
use view::display_board;


fn main() {
    let (sender, receiver) = channel::<Directions>();
    let settings = Settings { board_width: 50, board_height: 25 };
    let mut stdout = stdout().into_raw_mode().unwrap();
    let board = new_board(settings.board_height, settings.board_width);
    std::thread::spawn(move || read_key_down_event(sender));
    let mut dir = Directions::Up;
    display_board(&board, &mut stdout);

    // loop {
    //     if let Ok(input_value) = receiver.try_recv() {
    //         dir = input_value;
    //     }
    //     display_board(&board, &mut stdout);
    //     display_board(&board, &mut stdout);


    //     write!(stdout, "{} {}", cursor::Goto(1,1), clear::All).unwrap();
    //     stdout.flush().unwrap();
    // }
}
