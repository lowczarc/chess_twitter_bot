use std::{collections::HashMap, path::Path};

use chess::{Action, Game};
use rust_twitter_bot_lib::TwitterBot;

pub fn get_twitter_move(game: &Game, twitter_bot: &TwitterBot, reply_to: Option<i64>) -> i64 {
    let actions = game.actions();
    let mut params: HashMap<&str, &str> = HashMap::new();

    let image = twitter_bot
        .upload_file(Path::new("chess_board.png"))
        .unwrap()
        .to_string();
    let mut reply;

    if let Some(reply_id) = reply_to {
        reply = reply_id.to_string();
        params.insert("in_reply_to_status_id", &reply);
    }
    params.insert("media_ids", &image);

    let tweet = twitter_bot
        .tweet(
            &format!(
                "{}#TwitterVsStockfish\n{}Vote for the next move in the comments with the hashtag #MyPlanToBeatStockfish ⬇️",
                if reply_to.is_some() { "@ZezezBot " } else { "" },
                if actions.len() > 1 {
                    format!(
                        "Twitter last move: {}\nStockfish move: {}\n",
                        if let Action::MakeMove(makemove) = actions.get(actions.len() - 2).unwrap()
                        {
                            makemove.to_string()
                        } else {
                            String::new()
                        },
                        if let Action::MakeMove(makemove) = actions.get(actions.len() - 1).unwrap()
                        {
                            makemove.to_string()
                        } else {
                            String::new()
                        }
                    )
                } else {
                    String::new()
                },
            ),
            Some(params),
        )
        .unwrap();

    tweet.id()
}
