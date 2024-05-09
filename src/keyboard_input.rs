use std::sync::mpsc::Sender;
use std::io::stdin;
use strum_macros::Display;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Display, Clone, Copy)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub fn read_key_down_event(sender: Sender<Directions>) {
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                sender.send(Directions::Up).unwrap();
            },
            Key::Down => {
                sender.send(Directions::Down).unwrap();
            },
            Key::Left => {
                sender.send(Directions::Left).unwrap();
            },
            Key::Right => {
                sender.send(Directions::Right).unwrap();
            },
            _ => (),

        }
    }
}
