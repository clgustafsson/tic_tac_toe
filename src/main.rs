use crate::game::*;
use crate::engine::*;

mod game;
mod engine;

fn main() {
    let my_game: Game = Game {..Default::default()};
    let mut root: Node = Node::new(my_game);
        

    root.game.play_move([2, 2]);
    root.game.play_move([1, 2]);

    root.go();
}
