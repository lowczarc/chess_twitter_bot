use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, Command, Stdio},
};

use chess::{Board, ChessMove, Color, File, Game, MoveGen, Piece, Rank, Square};

const move_time: u32 = 500;

fn str_to_chess_move(chess_move: &str) -> ChessMove {
    let src = Square::from_string(chess_move[..2].to_owned()).expect("not a valid square");
    let dest = Square::from_string(chess_move[2..4].to_owned()).expect("not a valid square");
    let promotion: Option<Piece> = match chess_move.chars().nth(4) {
        Some('q') => Some(Piece::Queen),
        Some('b') => Some(Piece::Bishop),
        Some('r') => Some(Piece::Rook),
        Some('k') => Some(Piece::Knight),
        _ => None,
    };

    ChessMove::new(src, dest, promotion)
}

fn stockfish_move(stockfish: &mut Child, all_moves: String) -> Result<ChessMove, ()> {
    let mut stdin = stockfish
        .stdin
        .as_mut()
        .expect("Failed to open stockfish stdin");

    let mut stdout = BufReader::new(stockfish.stdout.as_mut().unwrap());
    let command_sent = &format!(
        "position startpos moves {}\ngo movetime {}\n",
        all_moves, move_time
    );

    let size = stdin
        .write(command_sent.as_bytes())
        .expect("Failed to write to stdin");

    for line in stdout.lines() {
        if let Ok(command) = line {
            let words: Vec<&str> = command.split(" ").collect();
            if words[0] == "bestmove" {
                if words[1] == "(none)" {
                    return Err(());
                }
                return Ok(str_to_chess_move(words[1]));
            }
        }
    }
    Err(())
}

fn get_all_moves(game: &Game) -> String {
    return game
        .actions()
        .iter()
        .map(|elem| {
            if let chess::Action::MakeMove(mve) = elem {
                return mve.to_string();
            }
            "".to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");
}

fn print_board(board: Board) {
    for i in 0..8 {
        for j in 0..8 {
            let square = Square::make_square(Rank::from_index(7 - i), File::from_index(j));
            if let (Some(piece), Some(color)) = (board.piece_on(square), board.color_on(square)) {
                if color == Color::White {
                    print!("{}", piece.to_string().to_uppercase());
                } else {
                    print!("{}", piece);
                }
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut game = Game::new();
    let mut engine = Command::new(concat!(env!("CARGO_MANIFEST_DIR"), "/engine/stockfish"))
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Stockfish failed to launch");

    loop {
        if game.current_position().status() == chess::BoardStatus::Checkmate {
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
