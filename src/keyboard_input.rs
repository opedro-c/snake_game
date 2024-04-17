use std::sync::{Arc, Mutex};
use std::io::stdin;
use strum_macros::Display;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Display)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub fn read_key_down_event(direction: Arc<Mutex<Directions>>) {
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                let mut dir = direction.lock().unwrap();
                *dir = Directions::Up;
            }
            Key::Down => {
                let mut dir = direction.lock().unwrap();
                *dir = Directions::Down;
            }
            Key::Left => {
                let mut dir = direction.lock().unwrap();
                *dir = Directions::Left;
            }
            Key::Right => {
                let mut dir = direction.lock().unwrap();
                *dir = Directions::Right;
            },
            _ => todo!()

        }
    }
}
