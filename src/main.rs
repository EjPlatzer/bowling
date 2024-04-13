mod frame;
mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    _ = game.bowl(10);
    _ = game.bowl(5);
    _ = game.bowl(5);
    _ = game.bowl(8);
    _ = game.bowl(1);
    let _score = game.score();
}
