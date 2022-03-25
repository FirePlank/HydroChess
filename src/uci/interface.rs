use std::io;
use std::io::Write;

// use crate::r#move::movegen::*;
use crate::board::position::*;
use crate::search::*;
// use crate::r#move::encode::*;
use crate::uci::*;

const NAME: &str = "HydroChess";
const AUTHOR: &str = "FirePlank";

// main UCI loop
pub fn main_loop() {
    let mut cmd = String::new();
    let mut position = Position::empty();
    let mut searcher = Searcher::new();
    loop {
        // get UCI input inline
        io::stdout().flush().unwrap_or_else(|error| {
            println!("info string failed to keep things inline when taking input: {}", error);
        });
        io::stdin().read_line(&mut cmd).unwrap_or_else(|error| {
            println!("info string failed to take UCI input: {}", error);
            return 0;
        });

        // handle all the UCI commands
        match cmd.trim().split_whitespace().next().unwrap_or_else(|| {
            println!("info string no command given");
            return ".";
        }) {
            "uci" => {
                println!("id name {}", NAME);
                println!("id author {}", AUTHOR);
                println!("uciok");
            },
            "position" => {
                position = parse_position(&cmd);
            },
            "ucinewgame" => {
                position = Position::new();
            },
            "go" => {
                position.parse_go(&mut searcher, &cmd);
            },
            "isready" => println!("readyok"),
            "stop" => {
                // TODO: stop search
            }
            "quit" => break,
            "." => (),
            _ => println!("info string unknown UCI command: {}", cmd)
        }

        // reset command string
        cmd = String::new();
    }
}