mod board_image;
mod chess_engine;
mod twitter_move;

use std::{
    fs::File,
    io::{stdin, Read, Write},
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
        .write_all("setoption name Skill Level value 1".as_bytes())
        .expect("Failed to write to stdin");

    let assets = construct_assets();
    let twitter_bot = config_twitter_bot();

    loop {
        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        let stockfish_move = stockfish_move(&mut engine, get_all_moves(&game))
            .expect("Stockfish exited unexpectedly");
        println!("Stockfish move: {}", stockfish_move);
        game.make_move(stockfish_move);
        print_board(game.current_position());
        create_image(&assets, game.current_position(), "chess_board.png");

        if game.current_position().status() == BoardStatus::Checkmate {
            break;
        }
        get_twitter_move(&game, &twitter_bot);
        let next_move;
        loop {
            let mut player_move_str = String::new();
            stdin()
                .read_line(&mut player_move_str)
                .expect("stdin read failed");
            let player_move = str_to_chess_move(&player_move_str);
            if player_move.is_some() && game.current_position().legal(player_move.unwrap()) {
                next_move = player_move.unwrap();
                break;
            }
            println!("Invalid move");
        }
        game.make_move(next_move);
        print_board(game.current_position());
        create_image(&assets, game.current_position(), "chess_board.png");
    }
    println!("Checkmate");

    engine.kill().expect("Stockfish failed to shutdown");
}
