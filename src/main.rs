mod common;
mod screen;
mod game;

use game::game::*;
use log::{info, LevelFilter};
use screen::screen::*;

fn main() {
    env_logger::Builder::new().filter_level(LevelFilter::Info).init();
    //println!("{}", Game::new());
    let s = Screen::new();
    println!("{}", s);
    info!("Redered Frame");
    println!("{}", s);
}