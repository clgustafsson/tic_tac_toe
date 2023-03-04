use crate::engine::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    None,
    X,
    O,
    Draw,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: [[State; 3]; 3],
    pub turn: State,
    pub legal_moves: Vec<[usize; 2]>,
    pub win: State,
}

impl Default for Game{
    fn default() -> Game {
        Game {
            board: [[State::None; 3]; 3],
            turn: State::X,
            legal_moves: vec![[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]],
            win: State::None,
        }     
    }
}

impl Game {
    pub fn go(self) {
        let mut root: Node = Node::new(self);
        root.go();
    }
    
    pub fn other_player(&self) -> State {
        match self.turn {
            State::X => {State::O},
            State::O => {State::X},
            _ => {State::None},
        }
    }

    pub fn play_move(&mut self, cord: [usize; 2]) {
        let x = cord[0];
        let y = cord[1];
        self.legal_moves.retain(|&x| x != cord);
        match self.turn {
            State::X => {
                self.board[x][y] = State::X;
                self.turn = State::O;
            }
            State::O => {
                self.board[x][y] = State::O;
                self.turn = State::X;
            }
            _ => {}
        }
        self.check_for_win();
    } 

    pub fn check_for_win(&mut self) {
        let players = [State::X, State::O];
        for player in players.iter() {
            if 
            self.board[0][0] == *player && self.board[0][1] == *player && self.board[0][2] == *player || 
            self.board[1][0] == *player && self.board[1][1] == *player && self.board[1][2] == *player || 
            self.board[2][0] == *player && self.board[2][1] == *player && self.board[2][2] == *player || 
            self.board[0][0] == *player && self.board[1][0] == *player && self.board[2][0] == *player || 
            self.board[0][1] == *player && self.board[1][1] == *player && self.board[2][1] == *player || 
            self.board[0][2] == *player && self.board[1][2] == *player && self.board[2][2] == *player || 
            self.board[0][0] == *player && self.board[1][1] == *player && self.board[2][2] == *player || 
            self.board[0][2] == *player && self.board[1][1] == *player && self.board[2][0] == *player {
                self.win = *player;
                self.legal_moves = vec![];
            }
            
        }
        if self.legal_moves.is_empty() && self.win == State::None {
            self.win = State::Draw;
        }
    }

    pub fn print(&self) {
        println!("{:?} {:?} {:?}\n{:?} {:?} {:?}\n{:?} {:?} {:?}" ,self.board[0][0], self.board[0][1], self.board[0][2], self.board[1][0], self.board[1][1], self.board[1][2], self.board[2][0], self.board[2][1], self.board[2][2],)
    }
    }