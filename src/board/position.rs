use std::collections::HashMap;

use super::attacks::*;
use super::bitboard::*;
use super::magic::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    NO_SQUARE
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
pub const ASCII_PIECES: &[u8] = "PNBRQKpnbrqk".as_bytes();
// unicode pieces
pub const UNICODE_PIECES: [&str;12] = ["♟", "♞", "♝", "♜", "♛", "♚", "♙", "♘", "♗", "♖", "♕", "♔"];
// convert ASCII character pieces to encoded constants
lazy_static! {
    pub static ref CHAR_PIECES: HashMap<&'static str, Piece> = HashMap::from([
        ("P", Piece::WhitePawn),
        ("N", Piece::WhiteKnight),
        ("B", Piece::WhiteBishop),
        ("R", Piece::WhiteRook),
        ("Q", Piece::WhiteQueen),
        ("K", Piece::WhiteKing),
        ("p", Piece::BlackPawn),
        ("n", Piece::BlackKnight),
        ("b", Piece::BlackBishop),
        ("r", Piece::BlackRook),
        ("q", Piece::BlackQueen),
        ("k", Piece::BlackKing),
    ]);
}

// castling rights
pub enum Castling {
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8
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
    BlackKing
}
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
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
}
impl Position {
    pub fn new() -> Position {
        Position {
            bitboards: [Bitboard(0); 12],
            occupancies: [Bitboard(0); 3],
            side: Side::WHITE,
            enpassant: Square::NO_SQUARE,
            castle: Castling::BK as u8,
        }
    }
}

pub static mut POSITION: Position = Position {
    bitboards: [Bitboard(0); 12],
    occupancies: [Bitboard(0); 3],
    side: Side::WHITE,
    enpassant: Square::NO_SQUARE,
    castle: Castling::BK as u8,
};