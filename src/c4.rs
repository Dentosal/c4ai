#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}
impl Player {
    pub fn other(&self) -> Player {
        match *self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
impl Copy for Player {}
impl Clone for Player {
    fn clone(&self) -> Player { *self }
}

pub struct Board {
    pub turn: Player, // turn does not automatically change
    board: [Option<Player>; 6*7],
}

impl Board {
    // Public
    pub fn next_turn(&mut self) {
        self.turn = self.turn.other();
    }
    pub fn is_full(&self) -> bool {
        for i in 0..42 {
            if !self.board[i].is_some() {
                return false;
            }
        }
        true
    }
    pub fn has_won(&self, color: Player) -> bool {
        // vertical winning row? "-"
        for y in 0..6 {
            for x in 0..4 {
                let mut win = true;
                for dx in 0..4 {
                    if !self.test(x+dx, y, color) {
                        win = false;
                        break;
                    }
                }
                if win {
                    return true;
                }
            }
        }
        // horizontal winning row? "|"
        for x in 0..7 {
            for y in 0..3 {
                let mut win = true;
                for dy in 0..4 {
                    if !self.test(x, y+dy, color) {
                        win = false;
                        break;
                    }
                }
                if win {
                    return true;
                }
            }
        }
        // diagonal winning rows "\/"
        for x in 0..4 {
            for y in 0..3 {
                // tl-br "\"
                let mut win = true;
                for d in 0..4 {
                    if !self.test(x+d, y+d, color) {
                        win = false;
                        break;
                    }
                }
                if win {
                    return true;
                }
                // bl-tr "/"
                win = true;
                for d in 0..4 {
                    if !self.test(x+d, y+3-d, color) {
                        win = false;
                        break;
                    }
                }
                if win {
                    return true;
                }
            }
        }
        false
    }
    pub fn get_winner(&self) -> Option<Player> {
        if self.has_won(Player::White) {
            Some(Player::White)
        }
        else if self.has_won(Player::Black) {
            Some(Player::Black)
        }
        else {
            None
        }
    }
    pub fn is_over(&self) -> bool {
        self.is_full() || self.get_winner().is_some()
    }
    pub fn get(&self, x: usize, y: usize) -> Option<Player> {
        self.board[x+y*7]
    }
    pub fn test(&self, x: usize, y: usize, color: Player) -> bool {
        // false if empty
        let item = self.board[x+y*7];
        item.is_some() && item.unwrap() == color
    }
    pub fn column_get_free_space(&self, x: usize) -> usize {
        for y in 0..6 {
            if self.get(x, y).is_some() {
                return y;
            }
        }
        6
    }
    pub fn put(&mut self, x: usize) {
        let turn = self.turn;
        self.column_put(x, turn);
    }
    // Private
    pub fn column_put(&mut self, x: usize, color: Player) {
        let space = self.column_get_free_space(x);
        if space == 0 {
            panic!("Cannot put to full column.");
        }
        self.set(x, space-1, color);
    }
    fn set(&mut self, x: usize, y: usize, color: Player) {
        self.board[x+y*7] = Some(color);
    }
}

impl Copy for Board {}
impl Clone for Board { fn clone(&self) -> Board { *self } }
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("1 2 3 4 5 6 7\n");
        for y in 0..6 {
            for x in 0..7 {
                result.push(match self.get(x, y) {
                    Some(Player::White) => 'W',
                    Some(Player::Black) => 'B',
                    None                => '-',
                });
                result.push(' ');
            };
            result.push('\n');
        };
        write!(f, "{}", result)
    }
}

pub fn create_empty_board() -> Board {
    Board {board: [None; 42], turn: Player::White}
}
