extern crate rand;
use rand::Rng;
use std::io;

mod Board;
use Board::Gamestate;
mod Solve;

fn main() {
    let mut state = init();

    let mut input = String::new();
    println!("1) Let the AI play to 2048\n2) Play to 2048 yourself");
    io::stdin().read_line(&mut input);
    match input.trim() {
        "1" => {
            println!("Preparing AI...");
            let score = Solve::ai_solve(state);
            println!("Game over! You ended with a score of {}!", score);
        },
        "2" => {
            println!("Preparing board...");
            let score = Solve::pl_solve(&mut state);
            println!("Game over! You ended with a score of {}!", score);
        },
        _ => {
            println!("Option not recognized.\nProgram ending...")
        }
    }
}


// initialize variables
fn init() -> Gamestate {
    let mut state = Gamestate{
        score: 0,
        board: [[0,0,0,0]; 4],
    };

    // insert [2:90%,4:10%] into empty game state
    let row = rand::thread_rng().gen_range(0,4);
    let col = rand::thread_rng().gen_range(0,4);
    let mut v = rand::thread_rng().gen_range(0,10);
    match v {
        0..=8 => v = 2,
        9 => v = 4,
        _ => println!("Error: out of bounds.")
    }
    state.board[row][col] = v;

    state
}