use crate::game::*;

mod game;
mod engine;

fn main() {
    let mut my_game: Game = Game {
        ..Default::default()
    };

    my_game.play_move([2, 2]);

    my_game.go();
}
