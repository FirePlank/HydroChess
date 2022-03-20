use std::collections::HashMap;

use super::attacks::*;
use super::bitboard::*;
use super::magic::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    NoSquare
}
// square string list
#[allow(dead_code)]
pub const SQUARE_COORDS: [&str;64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

// ASCII pieces
pub const ASCII_PIECES: [&str;12] = [
    "P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k",
];
// unicode pieces
pub const UNICODE_PIECES: [&str;12] = ["♟", "♞", "♝", "♜", "♛", "♚", "♙", "♘", "♗", "♖", "♕", "♔"];
// convert ASCII character pieces to encoded constants
lazy_static! {
    pub static ref ASCII_TO_SQUARE:  HashMap<&'static str, Square> = HashMap::from([
        ("a8", Square::A8),("b8", Square::B8),("c8", Square::C8),("d8", Square::D8),("e8", Square::E8),("f8", Square::F8),("g8", Square::G8),("h8", Square::H8),
        ("a7", Square::A7),("b7", Square::B7),("c7", Square::C7),("d7", Square::D7),("e7", Square::E7),("f7", Square::F7),("g7", Square::G7),("h7", Square::H7),
        ("a6", Square::A6),("b6", Square::B6),("c6", Square::C6),("d6", Square::D6),("e6", Square::E6),("f6", Square::F6),("g6", Square::G6),("h6", Square::H6),
        ("a5", Square::A5),("b5", Square::B5),("c5", Square::C5),("d5", Square::D5),("e5", Square::E5),("f5", Square::F5),("g5", Square::G5),("h5", Square::H5),
        ("a4", Square::A4),("b4", Square::B4),("c4", Square::C4),("d4", Square::D4),("e4", Square::E4),("f4", Square::F4),("g4", Square::G4),("h4", Square::H4),
        ("a3", Square::A3),("b3", Square::B3),("c3", Square::C3),("d3", Square::D3),("e3", Square::E3),("f3", Square::F3),("g3", Square::G3),("h3", Square::H3),
        ("a2", Square::A2),("b2", Square::B2),("c2", Square::C2),("d2", Square::D2),("e2", Square::E2),("f2", Square::F2),("g2", Square::G2),("h2", Square::H2),
        ("a1", Square::A1),("b1", Square::B1),("c1", Square::C1),("d1", Square::D1),("e1", Square::E1),("f1", Square::F1),("g1", Square::G1),("h1", Square::H1),
    ]); 
}

// castling rights
pub enum Castling {
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8,
}

// pieces
#[derive(Debug)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

pub struct Side;
impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
    pub const BOTH: usize = 2;
}

pub struct Position {
    pub bitboards: [Bitboard; 12],
    pub occupancies: [Bitboard; 3],
    pub side: usize,
    pub enpassant: Square,
    pub castle: u8,
    pub halfmove: u32,
    pub fullmove: u32,
}
impl Position {
    pub fn new() -> Position {
        Position {
            bitboards: [Bitboard(0); 12],
            occupancies: [Bitboard(0); 3],
            side: Side::WHITE,
            enpassant: Square::NoSquare,
            castle: 15, // <--- all castles allowed
            halfmove: 0,
            fullmove: 0
        }
    }
    pub fn empty() -> Position {
        Position {
            bitboards: [Bitboard(0); 12],
            occupancies: [Bitboard(0); 3],
            side: Side::WHITE,
            enpassant: Square::NoSquare,
            castle: 0,
            halfmove: 0,
            fullmove: 0
        }
    }
    pub fn show(&self, unicode: bool) {
        let pieces;
        if unicode {
            pieces = UNICODE_PIECES;
        } else {
            pieces = ASCII_PIECES;
        }
        // loop over board ranks
        for rank in 0..8 {
            for file in 0..8 {
                // init square
                let square = rank * 8 + file;

                if file == 0 {
                    print!("{}  ", 8 - rank);
                }

                // define piece
                let mut piece: i8 = -1;
                // loop over all the piece bitboards
                for bb_piece in 0..12 {
                    if self.bitboards[bb_piece].get(square) != 0{
                        piece = bb_piece as i8;
                        break;
                    }
                }

                if piece == -1 {
                    print!(". ");
                } else {
                    print!("{} ", pieces[piece as usize]);
                }
            }
            // print new line every rank
            println!();
        }
        // print board files
        println!("   a b c d e f g h\n");
        // print side to move
        println!("Side to move: {}", if self.side == Side::WHITE { "White" } else { "Black" });
        // print enpassant
        println!("Enpassant: {}", if self.enpassant == Square::NoSquare { "None" } else { SQUARE_COORDS[self.enpassant as usize] });
        // print castling rights
        println!("Castling: {}", if self.castle == 0 { "None".to_string() } else {
            let mut castling = String::new();
            if self.castle & Castling::WK as u8 != 0 {
                castling.push_str("K");
            } else { castling.push_str("-"); }
            if self.castle & Castling::WQ as u8 != 0 {
                castling.push_str("Q");
            } else { castling.push_str("-"); }
            if self.castle & Castling::BK as u8 != 0 {
                castling.push_str("k");
            } else { castling.push_str("-"); }
            if self.castle & Castling::BQ as u8 != 0 {
                castling.push_str("q");
            } else { castling.push_str("-"); }
            castling
        });
    }

    pub fn from_fen(fen: &str) -> Position {
        let mut position = Position::empty();

        let mut square = 0;
        let mut rank = 0;
        let mut file = 0;
        let mut index = 0;
        for x in fen.split_whitespace() {
            if index == 0 {
                for i in x.chars() {
                    match i {
                        '1' => {
                            square += 1;file += 1;
                            square = rank * 8 + file;
                        },
                        '2' => {
                            square += 2;file += 2;
                            square = rank * 8 + file;
                        },
                        '3' => {
                            square += 3;file += 3;
                            square = rank * 8 + file;
                        },
                        '4' => {
                            square += 4;file += 4;
                            square = rank * 8 + file;
                        },
                        '5' => {
                            square += 5;file += 5;
                            square = rank * 8 + file;
                        },
                        '6' => {
                            square += 6;file += 6;
                            square = rank * 8 + file;
                        },
                        '7' => {
                            square += 7;file += 7;
                            square = rank * 8 + file;
                        },
                        '8' => {
                            square += 8;file += 8;
                            square = rank * 8 + file;
                        },
                        'P' => {
                            position.bitboards[Piece::WhitePawn as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'N' => {
                            position.bitboards[Piece::WhiteKnight as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'B' => {
                            position.bitboards[Piece::WhiteBishop as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'R' => {
                            position.bitboards[Piece::WhiteRook as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'Q' => {
                            position.bitboards[Piece::WhiteQueen as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'K' => {
                            position.bitboards[Piece::WhiteKing as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'p' => {
                            position.bitboards[Piece::BlackPawn as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'n' => {
                            position.bitboards[Piece::BlackKnight as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'b' => {
                            position.bitboards[Piece::BlackBishop as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'r' => {
                            position.bitboards[Piece::BlackRook as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'q' => {
                            position.bitboards[Piece::BlackQueen as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        'k' => {
                            position.bitboards[Piece::BlackKing as usize].set(square);
                            file += 1;square = rank * 8 + file;
                        },
                        '/' =>  {
                            rank += 1;file = 0;
                        },
                        _ => (),
                    }
            }
            } else if index == 1 {
                if x == "w" {
                    position.side = Side::WHITE;
                } else {
                    position.side = Side::BLACK;
                }
            } else if index == 2 {
                for i in x.chars() {
                    match i {
                        'K' => {
                            position.castle |= Castling::WK as u8;
                        },
                        'Q' => {
                            position.castle |= Castling::WQ as u8;
                        },
                        'k' => {
                            position.castle |= Castling::BK as u8;
                        },
                        'q' => {
                            position.castle |= Castling::BQ as u8;
                        },
                        _ => (),
                    }
                }
            } else if index == 3 {
                if x != "-" {
                    position.enpassant = *ASCII_TO_SQUARE.get(x).unwrap();
                }
            } else if index == 4 {
                position.halfmove = x.parse::<u32>().unwrap();
            } else if index == 5 {
                position.fullmove = x.parse::<u32>().unwrap();
            }
            index += 1;
        }
        return position; 
    }
}

// pub static mut POSITION: Position = Position {
//     bitboards: [Bitboard(0); 12],
//     occupancies: [Bitboard(0); 3],
//     side: Side::WHITE,
//     enpassant: Square::NoSquare,
//     castle: 15,
// };