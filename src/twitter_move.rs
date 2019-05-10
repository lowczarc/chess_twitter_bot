use std::{collections::HashMap, path::Path, thread::sleep, time::Duration};

use chess::{Action, Game};
use rust_twitter_bot_lib::{tweet_structure::Tweet, TwitterBot};

const TIME_BETWEEN_CHECK: u64 = 15;
const MINIMUM_COMMENTS_NUM: usize = 1;

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

    let mut since_id = tweet.id().to_string();
    let mut responses: Vec<Tweet> = Vec::new();

    while responses.len() < MINIMUM_COMMENTS_NUM {
        sleep(Duration::from_secs(TIME_BETWEEN_CHECK));

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("result_type", "recent");
        params.insert("since_id", &since_id);

        let mut new_responses: Vec<Tweet> = twitter_bot
            .get_tweets_query(&format!("to:ZezezBot #MyPlanToBeatStockfish"), Some(params))
            .unwrap()
            .into_iter()
            .filter(|elem| elem.reply_to() == Some(tweet.id()))
            .collect();
        if new_responses.get(0).is_some() {
            since_id = new_responses.get(0).unwrap().id().to_string();
        }
        responses.append(&mut new_responses);
    }

    println!("{:?}", responses);

    tweet.id()
}
