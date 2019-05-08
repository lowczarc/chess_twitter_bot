mod board_image;
mod chess_engine;

use std::{
    fs::File,
    io::Read,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};

use chess::{BoardStatus, Game};
use rust_twitter_bot_lib::{tweet_structure::QueryOption, TwitterBot};
use toml;

use board_image::*;
use chess_engine::*;

fn config_twitter_bot() -> TwitterBot {
    let mut file_content = String::new();

    let mut config_file = File::open("Config.toml").expect("Config.toml needed");
    config_file
        .read_to_string(&mut file_content)
        .expect("Invalid Config.toml");

    toml::from_str(&file_content).unwrap()
}

fn main() {
    let mut game = Game::new();
    let mut engine = Command::new("engine/stockfish")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Stockfish failed to launch");
    let assets = construct_assets();
    let twitter_bot = config_twitter_bot();

    loop {
        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        let stockfish_move = stockfish_move(&mut engine, get_all_moves(&game))
            .expect("Stockfish exited unexpectedly");
        game.make_move(stockfish_move);
        print_board(game.current_position());
        create_image(&assets, game.current_position(), "chess_board.png");

        twitter_bot
            .tweet(
                "Stockfish vs Stockfish #Stockfish #Chess #TwitterBot #Rust",
                Some(
                    twitter_bot
                        .upload_file(Path::new("chess_board.png"))
                        .unwrap(),
                ),
            )
            .unwrap();
    }

    engine.kill().expect("Stockfish failed to shutdown");
}
