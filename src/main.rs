mod common;
mod screen;
mod game;

use game::game::*;
use screen::screen::*;

fn main() {
    println!("{}", Game::new());
    println!("{}", Screen::new())
}