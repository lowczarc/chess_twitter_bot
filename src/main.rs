use chess::{Board, BoardStatus, ChessMove, Color, File, Piece, Rank, Square};

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
    let mut board = Board::default();
    let mut moves: [ChessMove; 256] = [ChessMove::default(); 256];

    for _ in 0..200 {
        let size = board.enumerate_moves(&mut moves);
        board = board.make_move_new(moves[0]);
        print_board(board);
        println!("\n");
    }
}
