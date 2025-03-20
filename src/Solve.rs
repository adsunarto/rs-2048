use std::io;
use std::convert::TryInto;

// impl Copy for Gamestate {} { fn clone(&self) -> Self { *self } }

// mod Board;
use crate::Board::Gamestate;


// implement AI
pub fn ai_solve(state: Gamestate) -> u16 {
//   let mut score = 0;
  let mut tmp_state;
  let mut queue = vec![state];

  while queue.len() > 0 {
    tmp_state = queue[0];
    if tmp_state.get_max() == 2048 {
        tmp_state.print();
        return tmp_state.score.try_into().unwrap();
    }
    else {
        // queue.append(Gamestate::swipe_up(tmp_state));
        // queue.append(Gamestate::swipe_down(tmp_state));
        // queue.append(Gamestate::swipe_left(tmp_state));
        // queue.append(Gamestate::swipe_right(tmp_state));
    }
    queue.remove(0);
  }
  0
//   state.print();
//   tmp_state.score
}


// game via user input
pub fn pl_solve(state: &mut Gamestate) -> i16 {
  let mut max_tile = 0;
  state.print();

  while max_tile != 2048 && state.can_move() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    match input.trim() {
        "w" => {
            *state = Gamestate::swipe_up(state);
            state.print();
        },
        "a" => {
            *state = Gamestate::swipe_left(state);
            state.print();
        },
        "s" => {
            *state = Gamestate::swipe_down(state);
            state.print();
        },
        "d" => {
            *state = Gamestate::swipe_right(state);
            state.print();
        },
        _ => println!("Invalid command. Try again.")
    }
    max_tile = state.get_max();
  }
  state.score
}