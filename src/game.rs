use std::io;

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
    pub last_move: Option<[usize; 2]>,
    pub win: State,
}

impl Game {
    pub fn from_move_str(str: &str) -> Result<Game, io::Error> {
        let str_len = str.len();
        if str_len > 16 || str_len < 2|| str_len % 2 != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input str",));
        }
        else {
            let mut game = Game::empty(); 
            const INVALID_MOVE: usize = 3;
            for i in 1..str_len/2+1 {
                let x: usize = str[2*i-2..2*i-1].parse::<usize>().unwrap_or_else(|e|{
                    eprintln!("Error when parsing slice: {}", e);
                    return INVALID_MOVE} );
                let y: usize = str[2*i-1..2*i].parse::<usize>().unwrap_or_else(|e|{
                    eprintln!("Error when parsing slice: {}", e);
                    return INVALID_MOVE} );
                if x == INVALID_MOVE || y == INVALID_MOVE {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input str",));
                }
                let cord = [x, y];
                game.play_move(cord).unwrap_or_else(|e| {
                    eprintln!("Error when playing move: {}", e)});
            }
            Ok(game)
        }
    }

    pub fn empty() -> Game {
        Game {
            board: [[State::None; 3]; 3],
            turn: State::X,
            legal_moves: vec![[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]],
            last_move: None,
            win: State::None,
        }     
    }

    pub fn other_player(&self) -> State {
        match self.turn {
            State::X => {State::O},
            State::O => {State::X},
            _ => {State::None},
        }
    }

    pub fn play_move(&mut self, cord: [usize; 2]) -> Result<(), io::Error> {
        if self.legal_moves.contains(&cord) {
            self.last_move = Some(cord);
            let x = cord[0];
            let y = cord[1];
            self.legal_moves.retain(|&xy| xy != cord);
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
        else {
             return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{:?} is not a legal move", cord)));
        }
        Ok(())
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