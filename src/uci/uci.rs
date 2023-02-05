use crate::variants::antichess;
use crate::{r#move::movegen::*, board::position};
use crate::board::position::*;
use crate::cache::*;
use crate::r#move::encode::*;
use crate::search::*;
use crate::interface::SUPPORTED_VARIANTS;

use std::time::{UNIX_EPOCH, SystemTime};

impl Position {
    // parse user/GUI move string input (eg. "e2e4")
    pub fn parse_uci(&mut self, move_string: &str) -> u32 {
        // make iterator of lowercased str
        let move_parse = move_string.to_lowercase();
        let mut move_parse = move_parse.chars();
        // create move list
        let mut move_list = MoveList::new();
        if unsafe { OPTIONS.variant == Variant::Suicide } {
            move_list = antichess::generate_moves(self);
        } else { 
            // generate pseudo-legal moves
            self.generate_pseudo_moves(&mut move_list);
        }

        // parse source square
        let source_square = *ASCII_TO_SQUARE.get((move_parse.next().unwrap_or('.').to_string()+&move_parse.next().unwrap_or('.').to_string()).as_str()).unwrap_or_else( || {
            println!("info string Invalid source square given");
            return &Square::NoSquare;
        });
        if source_square == Square::NoSquare {
            return 0;
        }
        // parse target square
        let target_square = *ASCII_TO_SQUARE.get((move_parse.next().unwrap_or('.').to_string()+&move_parse.next().unwrap_or('.').to_string()).as_str()).unwrap_or_else( || {
            println!("info string Invalid target square given");
            return &Square::NoSquare;
        });
        if target_square == Square::NoSquare {
            return 0;
        }
        let promotion_string = move_parse.next().unwrap_or_else(|| {
            return '.';
        });

        // loop over the moves within a move list
        for move_count in 0..move_list.count {
            let move_ = move_list.moves[move_count as usize];
            // make sure source & target squares are available within the generated move
            if source_square as u8 == source(move_) && target_square as u8 == target(move_) {
                // init promoted piece
                let promoted_piece = promoted(move_);
                if promoted_piece != 0 {
                    // check if promotion piece is correct
                    if promotion_string == '.' { 
                        println!("info string No promotion piece given");
                        return 0;
                    }
                    // parse promotion piece and check move string for each promotion type

                    // promoted to queen
                    if ((promoted_piece == Piece::WhiteQueen as u8 || promoted_piece == Piece::BlackQueen as u8) && promotion_string == 'q') ||
                    // promoted to rook
                    ((promoted_piece == Piece::WhiteRook as u8 || promoted_piece == Piece::BlackRook as u8) && promotion_string == 'r') ||
                    // promoted to bishop
                    ((promoted_piece == Piece::WhiteBishop as u8 || promoted_piece == Piece::BlackBishop as u8) && promotion_string == 'b') ||
                    // promoted to knight
                    ((promoted_piece == Piece::WhiteKnight as u8 || promoted_piece == Piece::BlackKnight as u8) && promotion_string == 'n') ||
                    // promoted to king (variants only)
                    ((promoted_piece == Piece::WhiteKing as u8 || promoted_piece == Piece::BlackKing as u8) && promotion_string == 'k') {
                        // return move
                        return move_;
                    }

                    // continue loop to try to find promotion piece that matches move string
                    continue;
                }
                // return move if valid
                return move_;
            }
        }
        return 0;
    }

    // parse UCI "go" command
    pub fn parse_go(&mut self, cmd: &str) {
        // init error closures
        let error = || {
            println!("info string Invalid uci command given");
            return ".";
        };
        // let error_param = || {
        //     println!("info string Invalid parameter value given");
        //     return ".";
        // }
        let silent = || {
            return ".";
        };
        // init depth
        let mut depth: u8 = 0;
        // split command by whitespace
        let trimmed = cmd.trim().to_lowercase();
        let mut split_cmd = trimmed.split_whitespace();
        split_cmd.next().unwrap_or_else(error);

        let mut searcher: Searcher = Searcher::new();;
        unsafe {
        searcher.time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        loop {
            let next = split_cmd.next().unwrap_or_else(silent);
            if next == "." { break; }
            match next {
                "depth" => {
                    depth = split_cmd.next().unwrap_or_else(silent).parse::<u8>().unwrap_or_else(|error| {
                        println!("info string Invalid parameter value given: {}", error);
                        return 0;
                    });
                }, 
                "wtime" => {
                    if self.side == 0 {
                        searcher.playtime = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                            println!("info string Invalid parameter value given: {}", error);
                            return -1;
                        });
                        if searcher.playtime != -1 { searcher.playtime += 7; }
                    }
                },
                "btime" => {
                    if self.side == 1 {
                        searcher.playtime = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                            println!("info string Invalid parameter value given: {}", error);
                            return -1;
                        });
                        if searcher.playtime != -1 { searcher.playtime += 7; }
                    }
                },
                "winc" => {
                    if self.side == 0 {
                        searcher.inc = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                            println!("info string Invalid parameter value given: {}", error);
                            return 0;
                        });
                        if searcher.inc != 0 { searcher.inc += 6; }
                    }
                },
                "binc" => {
                    if self.side == 1 {
                        searcher.inc = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                            println!("info string Invalid parameter value given: {}", error);
                            return 0;
                        });
                        if searcher.inc != 0 { searcher.inc += 6; }
                    }
                },
                "movestogo" => {
                    searcher.movestogo = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                        println!("info string Invalid parameter value given: {}", error);
                        return 0;
                    });
                    if searcher.movestogo != 0 { searcher.movestogo += 10; }
                },
                "movetime" => {
                    searcher.movetime = split_cmd.next().unwrap_or_else(silent).parse::<i32>().unwrap_or_else(|error| {
                        println!("info string Invalid parameter value given: {}", error);
                        return -1;
                    });
                    if searcher.movetime != -1 { searcher.movetime += 9; }
                },
                _ => ()
            }
        }

        if searcher.movetime != -1 {
            searcher.playtime = searcher.movetime;
            searcher.movestogo = 1;
        }

        if searcher.playtime != -1 {
            searcher.timeset = true;
            searcher.playtime /= searcher.movestogo;

            // calculate the stop time based on the available moves and the phase of the game
            // let mut move_list = MoveList::new();
            // self.generate_pseudo_moves(&mut move_list);
            searcher.stoptime = searcher.time + searcher.playtime as u128 + searcher.inc as u128;
        }
        
        if depth == 0 { depth = MAX_PLY as u8; }
        println!("info string time: {} start: {} stop: {} depth: {} timeset: {}", searcher.playtime, searcher.time, searcher.stoptime, depth, searcher.timeset);

        static mut search: Searcher = Searcher::new();
        search = searcher.clone();
        static mut pos: Position = Position::empty();
        pos = self.clone();
        search.search_position(&mut pos, depth);
        }
    }
}

pub fn parse_position(cmd: &str) -> Position {
    // init error closures
    let error = || {
        println!("info string Invalid uci command given");
        return ".";
    };
    let silent = || {
        return ".";
    };
    // init position
    let mut position = Position::empty();
    
    // if 3check is enabled, split and remove + from command and parse it
    let mut cmd = cmd.to_string();
    let mut checks: [usize; 2];
    if unsafe { OPTIONS.variant == Variant::ThreeCheck } {
        // get amount of + signs in command
        let count = cmd.matches('+').count();
        // if there are more than 1 + signs
        if count == 1 {
            // checks are right before and after the first + sign
            let index = cmd.find('+').unwrap_or_else(|| {
                println!("info string Invalid uci command given");
                return 0;
            });
            checks = [cmd[index-1..index].parse::<usize>().unwrap_or_else(|error| {
                println!("info string Invalid parameter value given: {}", error);
                return 0;
            }), cmd[index+1..index+2].parse::<usize>().unwrap_or_else(|error| {
                println!("info string Invalid parameter value given: {}", error);
                return 0;
            })];

            for i in 0..2 {
                checks[i] = 3 - checks[i];
            }

            cmd = cmd.replace(&cmd[index-1..index+3], "");
        } else { 
            // get index of first + in command
            let index = cmd.find('+').unwrap_or_else(|| {
                println!("info string Invalid uci command given");
                return 0;
            });
            // get next 3 characters after +
            let check = &cmd[index+1..index+4];
            // parse checks, numbers are right before and after the + sign
            checks = [check[2..3].parse::<usize>().unwrap_or_else(|error| {
                println!("info string Invalid parameter value given: {}", error);
                return 0;
            }), check[0..1].parse::<usize>().unwrap_or_else(|error| {
                println!("info string Invalid parameter value given: {}", error);
                return 0;
            })];

            // remove that part from command starting from + and ending after 3rd character
            cmd = cmd.replace(&format!("+{}", check), "");
        }
    } else {
        checks = [0, 0];
    }

    // println!("info string cmd: {}", cmd);
    // println!("info string checks: {} {}", checks[0], checks[1]);

    // split command by whitespace
    let mut split_cmd = cmd.trim().split_whitespace();
    split_cmd.next().unwrap_or_else(error);

    let next = split_cmd.next().unwrap_or_else(silent);
    // parse UCI "startpos" command
    if next == "startpos" {
        // set position to startpos
        position = Position::new();
    // parse UCI "fen" command
    } else if next == "fen" {
        // make sure "fen" command is available within command string
        let fen = split_cmd.next().unwrap_or_else(silent).to_string()+" "+
        &(split_cmd.next().unwrap_or_else(silent)).to_string()+" "+&(split_cmd.next().unwrap_or_else(silent)).to_string()+" "+
        &(split_cmd.next().unwrap_or_else(silent)).to_string()+" "+&(split_cmd.next().unwrap_or_else(silent)).to_string()+" "+
        &(split_cmd.next().unwrap_or_else(silent)).to_string();

        // check if any of the fen assignments failed
        if fen.ends_with(".") {
            println!("info string Invalid fen given");
            return Position::empty();
        }
        // init board from fen
        position = Position::from_fen(&fen);
        position.checks = checks;

    } else if next != "." { println!("info string Invalid uci command given"); }

    // parse moves after position
    if position.occupancies[0].0 != 0 {
        let check = split_cmd.next().unwrap_or_else(silent);
        if check == "moves" {
            for move_ in split_cmd {
                let move_ = position.parse_uci(&move_);
                if move_ != 0 {
                    position.make(move_);

                } else {
                    println!("info string Invalid move given");
                    return Position::empty();
                }
            }
            return position;
        } else {
            return position;
        }
    }
    return Position::empty();
}

pub fn parse_option(cmd: &str) {
    // init error closures
    let error = || {
        println!("info string Invalid option given");
        return ".";
    };
    let silent = || {
        return ".";
    };

    // split command by whitespace
    let trimmed = cmd.trim().to_lowercase();
    let mut split_cmd = trimmed.split_whitespace();
    split_cmd.next().unwrap_or_else(error);
    // check if they have set a name
    if split_cmd.next().unwrap_or_else(silent) != "name" { return; }

    let name = split_cmd.next().unwrap_or_else(error);
    if name == "automatic" {
        if split_cmd.next().unwrap_or_else(error) == "threads" {
            if split_cmd.next().unwrap_or_else(error) == "value" {
                let response = split_cmd.next().unwrap_or_else(error);
                if response == "true" {
                    unsafe { OPTIONS.threads_automatic = true; }
                } else if response == "false" {
                    unsafe { OPTIONS.threads_automatic = false; }
                } else {
                    println!("info string Invalid value given, please give either true or false");
                }
            }
        }
    } else if name == "threads" {
        if split_cmd.next().unwrap_or_else(error) == "value" {
            let value = split_cmd.next().unwrap_or_else(silent).parse::<u16>().unwrap_or_else(|error| {
                println!("info string Invalid value for option given: {}", error);
                return 0;
            });
            if value < 1 {
                println!("info string Invalid value given, please give a value greater than 0");
            } else {
                unsafe { 
                    OPTIONS.threads = value;
                }
            }
        }
    } else if name == "hash" {
        if split_cmd.next().unwrap_or_else(error) == "value" {
            let value = split_cmd.next().unwrap_or_else(silent).parse::<u16>().unwrap_or_else(|error| {
                println!("info string Invalid value for option given: {}", error);
                return 0;
            });
            if value < 1 {
                println!("info string Invalid value given, please give a value greater than 0");
            } else {
                unsafe { 
                    OPTIONS.hash_size = value; 
                    TT = TranspositionTable::new(value as usize, false);
                }
            }
        }
    } else if name == "clear" {
        if split_cmd.next().unwrap_or_else(error) == "hash" {
            unsafe { TT.reset(); }
        }
    } else if name == "uci_variant" {
        if split_cmd.next().unwrap_or_else(error) == "value" {
            // check if variant in SUPPORTED_VARIANTS list
            let variant = split_cmd.next().unwrap_or_else(error);
            if SUPPORTED_VARIANTS.contains(&variant) {
                match variant {
                    "antichess" | "suicide" | "giveaway" => unsafe { OPTIONS.variant = Variant::Suicide },
                    "3check" | "three-check" => unsafe { OPTIONS.variant = Variant::ThreeCheck }
                    _ => (),
                }
            } else {
                println!("info string Unknown variant given");
            }
        }
    } else {
        println!("info string Unknown option given");
    }
}