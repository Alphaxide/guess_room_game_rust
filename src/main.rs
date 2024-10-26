mod game;
mod player;
mod room;
mod item;

use game::Game;
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();
    println!("welcome to the text based adventure game");

    loop {
        game.display_current_room();

        print!(">");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let command = input.trim();

        if !game.handle_command(command) {
            println!("thanks for playing ");
            break;
        }
    }
}