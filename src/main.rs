mod chess_engine;

use std::process::{Command, Stdio};

use chess::{BoardStatus, Game};
use chess_engine::*;

fn main() {
    let mut game = Game::new();
    let mut engine = Command::new("engine/stockfish")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Stockfish failed to launch");

    loop {
        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        let stockfish_move = stockfish_move(&mut engine, get_all_moves(&game))
            .expect("Stockfish exited unexpectedly");
        game.make_move(stockfish_move);
        print_board(game.current_position());
        println!("\n");
    }

    engine.kill().expect("Stockfish failed to shutdown");
}
