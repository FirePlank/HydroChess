use crate::r#move::movegen::*;
use crate::board::position::{self, *};
use crate::r#move::encode::*;
use crate::search::*;

impl Position {
    // parse user/GUI move string input (eg. "e2e4")
    pub fn parse_uci(&self, move_string: &str) -> u32 {
        // make iterator of lowercased str
        let move_parse = move_string.to_lowercase();
        let mut move_parse = move_parse.chars();
        // create move list
        let mut move_list = MoveList::new();
        // generate pseudo-legal moves
        self.generate_pseudo_moves(&mut move_list);

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
                    ((promoted_piece == Piece::WhiteKnight as u8 || promoted_piece == Piece::BlackKnight as u8) && promotion_string == 'n') {
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
        let mut split_cmd = cmd.trim().split_whitespace();
        split_cmd.next().unwrap_or_else(error);

        let next = split_cmd.next().unwrap_or_else(silent);
        // parse "depth" parameter
        if next == "depth" {
            depth = split_cmd.next().unwrap_or_else(silent).parse::<u8>().unwrap_or_else(|error| {
                println!("info string Invalid parameter value given: {}", error);
                return 0;
            }).max(1);
        } else if next == "wtime" {
            // placeholder
        }
        if depth == 0 { depth = 6; }
        println!("info depth {}", if depth == 0 { 99 } else { depth });
        let mut searcher = Searcher::new();
        let move_ = searcher.negamax(self, depth);
        let mut move_str = String::new();
        move_str.push_str(SQUARE_COORDS[source(move_) as usize]);
        move_str.push_str(SQUARE_COORDS[target(move_) as usize]);
        if promoted(move_) != 0 {
            move_str.push_str(PROMOTED_PIECES[promoted(move_) as usize]);
        }
        println!("bestmove {}", move_str);
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
    // split command by whitespace
    let mut split_cmd = cmd.trim().split_whitespace();
    split_cmd.next().unwrap_or_else(error);

    let next = split_cmd.next().unwrap_or_else(silent);
    // parse UCI "startpos" command
    if next == "startpos" {
        // return chess board with start position
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