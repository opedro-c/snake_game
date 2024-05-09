mod board;
mod view;
mod snake;
mod keyboard_input;
mod settings;
mod game_runner;

use crate::settings::Settings;
use std::sync::mpsc::channel;
use crate::keyboard_input::read_key_down_event;
use board::new_board;
use game_runner::GameRunner;
use keyboard_input::Directions;
use snake::Snake;
use std::io::stdout;
use termion::raw::IntoRawMode;
use view::display_board;


fn main() {
    let (sender, receiver) = channel::<Directions>();
    let settings = Settings { board_width: 50, board_height: 25 };
    let mut stdout = stdout().into_raw_mode().unwrap();
    let board = new_board(settings.board_height, settings.board_width);
    let snake = Snake::new(settings);
    let game_runner = GameRunner::new(snake, board.clone(), receiver);
    std::thread::spawn(move || read_key_down_event(sender));
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
