use crate::game::*;
use crate::engine::*;

mod game;
mod engine;

fn main() {
    //Short user guide
    //1: Create a Game
    //use Game::empty() for a new game, use Game::from_move_str for an ongoing game
    //2: Create a root with let mut root = Node::new(Game)
    //3: root.go() returns the best move as a String
    //if there are multiple equally good moves .go() chooses a random move

    //Example:
    let my_game = Game::from_move_str("00").unwrap_or_else(|e| {
        eprintln!("Error when creating game: {}", e);
        Game::empty()});
    let mut root= Node::new(my_game);
        
    root.go(true);
}
