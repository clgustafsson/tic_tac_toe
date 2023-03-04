use crate::game::*;

pub struct Node {
    pub game: Game,
    children: Vec<Node>,
}

impl Node {
    pub fn new(game: Game) -> Self {
        Self { game: game, children: vec![] }
    } 

    fn create_tree(&mut self) {
        for cord in self.game.legal_moves.iter() {
            let mut game_clone = self.game.clone();
            game_clone.play_move(*cord);
            self.children.push(Self::new(game_clone));
        }
        for child in self.children.iter_mut() {
            child.create_tree();
        }
    }

    fn drop_bad_children(&mut self) -> bool{
        let mut best_result: State = self.game.other_player();
        for child in self.children.iter_mut() {
            if child.game.win == State::None {
                child.drop_bad_children();
                return true;
            }
            else if child.game.win == self.game.turn {
                best_result = self.game.turn;
                break
            }
            else if child.game.win == State::Draw {best_result = State::Draw}
        }
        self.children.retain(|x| x.game.win == best_result);
        self.game.win = best_result;
        false
    }

    pub fn go(&mut self) {
        self.create_tree();
        while self.drop_bad_children() {}
        println!("{:?}",self.children.last().unwrap().game.win);
        self.children.last().unwrap().game.print();
    }

}