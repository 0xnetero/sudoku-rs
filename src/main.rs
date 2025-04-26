mod board;
mod game;

use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "SUDOKU GAME".green().bold());
    println!("{}", "============".green());
    
    let mut game = game::Game::new();
    game.run();
}
