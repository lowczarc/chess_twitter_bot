use std::process::{Command, Stdio};

use chess::{Board, Color, File, Game, MoveGen, Rank, Square};

fn get_all_moves(game: Game) -> String {
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

    for _ in 0..5 {
        game.make_move(MoveGen::new_legal(&game.current_position()).next().unwrap());
        print_board(game.current_position());
        println!("\n");
    }
    let moves = get_all_moves(game);
    println!("{}", moves);
    engine.kill().expect("Stockfish failed to shutdown");
}
