use std::process::Command;
use std::fs;

const REPOSITORY: &str = "https://github.com/official-stockfish/Stockfish.git";
const STOCKFISH_DIR: &str = "target/stockfish";
const SRC_STOCKFISH: &str = "target/stockfish/src";
const STOCKFISH: &str = "target/stockfish/src/stockfish";
const MOVE_TO: &str = "engine/stockfish";

fn main() {
    Command::new("git")
        .args(&["clone", REPOSITORY, STOCKFISH_DIR])
        .output()
        .expect("Can't clone Stockfish");

    Command::new("make")
        .args(&["build", "ARCH=x86-64", "-C", SRC_STOCKFISH])
        .output()
        .expect("Can't make Stockfish");

    fs::create_dir("engine").unwrap_or(());
    fs::copy(STOCKFISH, MOVE_TO).expect("Can't copy Stockfish executable");
}
