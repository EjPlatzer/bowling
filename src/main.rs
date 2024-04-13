mod frame;
mod game;

use std::io::{self};

use game::Game;

fn main() -> Result<(), String> {
    println!("Welcome to Evan's Bowling Alley!");
    println!("This program will help you score your game.");

    let mut game = Game::new();
    while !game.is_complete() {
        println!("How many pins did you bowl?");

        let result = read_pins().and_then(|pins| game.bowl(pins));

        if let Err(e) = result {
            println!("Error reading pins bowled: {}", e);
            continue;
        };

        println!("Your score is now {}", game.score())
    }

    let score = game.score();

    println!("You scored {}!", score);
    Ok(())
}

fn read_pins() -> Result<u8, String> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| format!("Error reading input: {}", e.kind()))?;

    line.trim_end().parse::<u8>().map_err(|e| format!("{}", e))
}
