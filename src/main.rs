use std::io::Write;
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
use termion::{clear, cursor, raw::IntoRawMode};
use view::display_board;


fn main() {
    let (sender, receiver) = channel::<Directions>();
    let settings = Settings { board_width: 50, board_height: 25 };
    let mut stdout = stdout().into_raw_mode().unwrap();
    let board = new_board(settings.board_height, settings.board_width);
    let mut snake = Snake::new(settings);
    let mut game_runner = GameRunner::new(&mut snake,&board, receiver);
    std::thread::spawn(move || read_key_down_event(sender));
    display_board(&board, &mut stdout);

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        game_runner.process_events();
        write!(stdout, "{} {}", cursor::Goto(1,1), clear::All).unwrap();
        display_board(&board, &mut stdout);
        if game_runner.is_game_over() {
            println!("Game over!");
            break;
        }
        stdout.flush().unwrap();
    }
}
