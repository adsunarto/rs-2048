extern crate rand;
use rand::Rng;

// #[derive(Clone, Copy)]
impl Copy for Gamestate {} impl Clone for Gamestate { fn clone(&self) -> Self { *self } }

pub struct Gamestate {
  pub score: i16,
  pub board: [[i16; 4]; 4],
}

use std::process::Command;
fn clear_console() {
      // Using "clear" on macOS
      Command::new("clear")
          .status()
          .expect("Failed to clear console");
}

impl Gamestate {
  // constructor
  pub fn new() -> Self {
    Gamestate {
      score: 0,
      board: [[0,0,0,0]; 4],
    }
  }

  // print board visually
  pub fn print(&self) {
    clear_console();
    
    let separator = "+----+----+----+----+";
    
    println!("{}", separator);
    for row in &self.board {
        for &num in row {
            print!("|{:^4}", num);
        }
        println!("|");
        println!("{}", separator);
    }
}


  // return value of tile with greatest value
  pub fn get_max(&self) -> u16 {
    let mut max_tile = 0;
    for (x,row) in self.board.iter().enumerate() {
      for (y,_col) in row.iter().enumerate() {
        if self.board[x][y] > max_tile {
          max_tile = self.board[x][y]
        }
      }
    }
    max_tile as u16
  }

  // return bool determining whether board can be updated or not
  pub fn can_move(&self) -> bool {
    true
  }
}


impl Gamestate {
  // UP
  pub fn swipe_up(state: &mut Gamestate) -> Gamestate {
    Self::transpose(state);
    Self::swipe_left(state);
    Self::transpose(state);
    *state
  }

  // DOWN
  pub fn swipe_down(state: &mut Gamestate) -> Gamestate {
    Self::flip_ud(state);
    Self::swipe_up(state);
    Self::flip_ud(state);
    *state
  }

  // LEFT
  pub fn swipe_left(state: &mut Gamestate) -> Gamestate {
    Self::merge_left(state);
    Self::slide_left(state);
    Self::spawn_tile(state);
    *state
  }

  fn merge_left(state: &mut Gamestate) -> Gamestate {
    for row in 0..4 {
      let mut k: i16 = -1; // k: col of last merged element
      for rcol in 1..4 {
        for lcol in (k+1..rcol as i16).rev() {
          if state.board[row as usize][rcol as usize] == 0 {
            // do nothing
          } else if state.board[row as usize][lcol as usize] != 0 && state.board[row as usize][lcol as usize] != state.board[row as usize][rcol as usize] {
            k = lcol;
          } else if state.board[row as usize][lcol as usize] == state.board[row as usize][rcol as usize] && state.board[row as usize][lcol as usize] != 0 {
            state.score += state.board[row as usize][lcol as usize] * 2;
            state.board[row as usize][lcol as usize] = state.board[row as usize][lcol as usize]*2;
            state.board[row as usize][rcol as usize] = 0;
            k = lcol;
          }
        }
      }
    }
    *state
  }

  fn slide_left(state: &mut Gamestate) -> Gamestate {
    for row in 0..4 {
      for col in 1..4 {
        if col == 1 {
          if state.board[row][col-1] == 0 {
            state.board[row][col-1] = state.board[row][col];
            state.board[row][col] = 0;
          }
        } else if col == 2 {
          if state.board[row][col-1] == 0 && state.board[row][col-2] == 0 {
            state.board[row][col-2] = state.board[row][col];
            state.board[row][col] = 0;
          } else if state.board[row][col-1] == 0 {
            state.board[row][col-1] = state.board[row][col];
            state.board[row][col] = 0;
          }
        } else if col == 3 {
          if state.board[row][col-1] == 0 && state.board[row][col-2] == 0 && state.board[row][col-3] == 0 {
            state.board[row][col-3] = state.board[row][col];
            state.board[row][col] = 0;
          } else if state.board[row][col-1] == 0 && state.board[row][col-2] == 0 {
            state.board[row][col-2] = state.board[row][col];
            state.board[row][col] = 0;
          } else if state.board[row][col-1] == 0 {
            state.board[row][col-1] = state.board[row][col];
            state.board[row][col] = 0;
          }
        }
      }
    }
    *state
  }

  // RIGHT
  pub fn swipe_right(state: &mut Gamestate) -> Gamestate {
    Self::flip_lr(state);
    Self::swipe_left(state);
    Self::flip_lr(state);
    *state
  }
}

impl Gamestate {
  // flip game board horizontally
  fn flip_lr(state: &mut Gamestate) -> Gamestate {
    let mut copy = [[0,0,0,0]; 4];
    for row in 0..4 {
      for col in 0..4 {
        copy[row][4-1-col] = state.board[row][col];
      }
    }
    state.board = copy;
    *state
  }

  // flip game board vertically
  fn flip_ud(state: &mut Gamestate) -> Gamestate {
    let mut copy = [[0,0,0,0]; 4];
    for col in 0..4 {
      for row in 0..4 {
        copy[4-1-row][col] = state.board[row][col];
      }
    }
    state.board = copy;
    *state
  }

  // transpose game board
  fn transpose(state: &mut Gamestate) -> Gamestate {
    let mut copy = [[0,0,0,0]; 4];
    for row in 0..4 {
      for col in 0..4 {
        copy[row][col] = state.board[col][row]
      }
    }
    state.board = copy;
    *state
  }

  fn spawn_tile(state: &mut Gamestate) -> Gamestate {
    let mut empty = vec![];
    for row in 0..4 {
      for col in 0..4{
        if state.board[row][col] == 0 {
          empty.push((row,col));
        }
      }
    }
    let mut v = rand::thread_rng().gen_range(0,10);
    match v {
        0..=8 => v = 2,
        9 => v = 4,
        _ => println!("Error: out of bounds.")
    }
    let xy = rand::thread_rng().gen_range(0,empty.len());
    state.board[empty[xy].0][empty[xy].1] = v;
    *state
  }
}