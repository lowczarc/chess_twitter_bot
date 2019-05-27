mod board_image;
mod chess_engine;
mod twitter_move;

use std::{
    fs::File,
    io::{Read, Write},
    process::{Command, Stdio},
};

use chess::{BoardStatus, Game};
use rust_twitter_bot_lib::TwitterBot;
use toml;

use board_image::*;
use chess_engine::*;
use twitter_move::*;

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
    let sstdin = engine
        .stdin
        .as_mut()
        .expect("Failed to open stockfish stdin");
    sstdin
        .write_all("setoption name Skill Level value 1\n".as_bytes())
        .expect("Failed to write to stdin");

    let twitter_bot = config_twitter_bot();

    let mut last_tweet: Option<i64> = None;

    loop {
        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        create_image(game.current_position(), "chess_board.png");
        last_tweet = Some(get_twitter_move(&mut game, &twitter_bot, last_tweet));
        print_board(game.current_position());

        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        let stockfish_move = stockfish_move(&mut engine, get_all_moves(&game))
            .expect("Stockfish exited unexpectedly");
        println!("Stockfish move: {}", stockfish_move);
        game.make_move(stockfish_move);
        print_board(game.current_position());
    }
    println!("Checkmate");

    engine.kill().expect("Stockfish failed to shutdown");
}
