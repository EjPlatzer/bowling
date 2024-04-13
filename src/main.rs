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

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .or_else(|e| Err(format!("Error reading input: {}", e.kind())))?;

        let bowled = match line.trim_end().parse::<u8>() {
            Ok(n) => n,
            Err(_e) => {
                println!("Invalid number of pins {}", line);
                continue;
            }
        };

        let _frame = match game.bowl(bowled) {
            Ok(frame) => frame,
            Err(e) => {
                println!("Error recording pins bowled: {}", e);
                continue;
            }
        };

        println!("Your score is now {}", game.score())
    }

    let score = game.score();

    println!("You scored {}!", score);
    Ok(())
}
