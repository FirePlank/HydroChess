#[macro_use]
extern crate lazy_static;

mod board {
    pub mod bitboard;
    pub mod attacks;
    pub mod magic;
    pub mod position;
}
use crate::board::bitboard::*;
use crate::board::attacks::*;
use crate::board::magic::*;
use crate::board::position::*;

// use std::time::Instant;
// use std::io;
// use std::io::Write;
// use std::io::stdin;

fn main() {
    init_all();

    let mut pos = Position::new();
    pos.enpassant = Square::A2;
    pos.bitboards[Piece::WhitePawn as usize].set(Square::A2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::B2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::C2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::D2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::E2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::F2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::G2 as usize);
    pos.bitboards[Piece::WhitePawn as usize].set(Square::H2 as usize);
    pos.bitboards[Piece::WhiteKnight as usize].set(Square::B1 as usize);
    pos.show(true);
}
