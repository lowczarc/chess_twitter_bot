use std::path::Path;

use chess::{Action, Game};
use rust_twitter_bot_lib::TwitterBot;

pub fn get_twitter_move(game: &Game, twitter_bot: &TwitterBot) -> () {
    let actions = game.actions();
    let tweet = twitter_bot
        .tweet(
            &format!(
                "#TwitterVsStockfish\n{}Stockfish move: {}\nVote for the next move in the comments ⬇️",
                if actions.len() > 1 {
                    format!(
                        "Twitter last move : {}\n",
                        if let Action::MakeMove(makemove) = actions.get(actions.len() - 2).unwrap()
                        {
                            makemove.to_string()
                        } else {
                            String::new()
                        }
                    )
                } else {
                    String::new()
                },
                if let Action::MakeMove(makemove) = actions.get(actions.len() - 1).unwrap() {
                    makemove.to_string()
                } else {
                    String::new()
                }
            ),
            Some(
                twitter_bot
                    .upload_file(Path::new("chess_board.png"))
                    .unwrap(),
            ),
        )
        .unwrap();

    println!("{}", tweet.id());
}
