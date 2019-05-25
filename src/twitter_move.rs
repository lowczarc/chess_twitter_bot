use std::{collections::HashMap, path::Path, thread::sleep, time::Duration};

use chess::{Action, ChessMove, Game, MoveGen};
use rust_twitter_bot_lib::{tweet_structure::Tweet, TwitterBot};

use crate::chess_engine::str_to_chess_move;

const TIME_BETWEEN_CHECK: u64 = 15;
const MINIMUM_COMMENTS_NUM: usize = 5;

pub fn get_vote_from_comment(tweet_content: &str) -> Option<ChessMove> {
    let words: Vec<&str> = tweet_content
        .split(" ")
        .filter(|elem| {
            elem.chars().nth(0).is_some()
                && elem.chars().nth(0) != Some('@')
                && elem.chars().nth(0) != Some('#')
        })
        .collect();

    if words.len() == 1 {
        return str_to_chess_move(words[0]);
    }
    None
}

pub fn get_twitter_move(game: &mut Game, twitter_bot: &TwitterBot, reply_to: Option<i64>) -> i64 {
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

    let mut since_id = tweet.id().to_string();
    let mut responses: HashMap<String, ChessMove> = HashMap::new();

    while responses.len() < MINIMUM_COMMENTS_NUM {
        sleep(Duration::from_secs(TIME_BETWEEN_CHECK));

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("result_type", "recent");
        params.insert("since_id", &since_id);

        let new_responses: Vec<Tweet> = twitter_bot
            .get_tweets_query(&format!("to:ZezezBot #MyPlanToBeatStockfish"), Some(params))
            .unwrap()
            .into_iter()
            .filter(|elem| elem.reply_to() == Some(tweet.id()))
            .collect();

        if new_responses.get(0).is_some() {
            since_id = new_responses.get(0).unwrap().id().to_string();
        }

        for response in new_responses.iter() {
            if let Some(chess_move) = get_vote_from_comment(response.content()) {
                responses.insert(response.user().name().to_string(), chess_move);
            }
        }
    }

    let mut possibles_moves: HashMap<ChessMove, i32> = HashMap::new();

    for legal_move in MoveGen::new_legal(&game.current_position()) {
        possibles_moves.insert(legal_move, 0);
    }

    for response in responses.values() {
        if let Some(&legal_move) = possibles_moves.get(response) {
            possibles_moves.insert(*response, legal_move + 1);
        }
    }

    let mut higher_move: (ChessMove, i32) = (ChessMove::default(), -1);

    for (chess_move, count) in possibles_moves.iter() {
        if higher_move.1 < *count {
            higher_move = (*chess_move, *count);
        }
    }

    println!(
        "Chosen move: {} with {} votes",
        higher_move.0, higher_move.1
    );
    game.make_move(higher_move.0);

    tweet.id()
}
