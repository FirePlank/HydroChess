use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::thread;

// use crate::r#move::movegen::*;
use crate::board::position::*;
use crate::search::*;
// use crate::r#move::encode::*;
use crate::uci::*;

const NAME: &str = "HydroChess";
const AUTHOR: &str = "FirePlank";
pub const SUPPORTED_VARIANTS: [&str; 6] = ["standard", "suicide", "giveaway", "antichess", "3check", "three-check"];

// main UCI loop
pub fn main_loop() {
    let mut cmd = String::new();
    let mut position = Position::new();

    println!("{} by {}", NAME, AUTHOR);

    loop {
        // get UCI input inline
        io::stdout().flush().unwrap_or_else(|error| {
            println!("info string failed to keep things inline when taking input: {}", error);
        });
        io::stdin().read_line(&mut cmd).unwrap_or_else(|error| {
            println!("info string failed to take UCI input: {}", error);
            return 0;
        });

        // append cmd to debug.txt on newline
        // let mut file = OpenOptions::new()
        //     .append(true)
        //     .open("debug.txt")
        //     .unwrap();

        // file.write_all(cmd.as_bytes()).expect("write failed");

        // handle all the UCI commands
        match cmd.trim().to_lowercase().split_whitespace().next().unwrap_or_else(|| {
            println!("info string no command given");
            return ".";
        }) {
            "uci" => {
                println!("id name {}", NAME);
                println!("id author {}", AUTHOR);
                println!("\noption name Threads type spin default 1 min 1 max 65535");
                println!("option name Automatic Threads type check default true");
                println!("option name Hash type spin default 32 min 1 max 65535");
                println!("option name Clear Hash type button");
                println!("option name UCI_Variant type combo default standard var {}", SUPPORTED_VARIANTS.join(" var "));
                println!("uciok");
            },
            "setoption" => parse_option(&cmd),
            "position" => {
                position = parse_position(&cmd);
            },
            "ucinewgame" => {
                position = Position::new();
                unsafe { 
                    TT.reset(); 
                    // REPETITION.iter_mut().for_each(|x| *x = 0);
                    // REP_INDEX = 0;
                }
            },
            "go" => {
                let mut pos: Position = position.clone();
                thread::spawn( move || {
                        pos.parse_go(&cmd);
                });
            },
            "isready" => println!("readyok"),
            "stop" => unsafe { STOP = true },
            "quit" => break,
            "." => (),
            _ => println!("info string unknown UCI command: {}", cmd)
        }

        // reset command string
        cmd = String::new();
    }
}
