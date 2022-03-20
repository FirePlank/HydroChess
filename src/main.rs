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

    let pos = Position::new();
    let mut board = pos.bitboards[Piece::WhitePawn as usize];
    board.set(Square::E2 as usize);
    board.show();

    // print piece
    println!("Piece: {}", ASCII_PIECES[Piece::WhitePawn as usize] as char);
    println!("Unicode: {}", UNICODE_PIECES[Piece::WhitePawn as usize]);

    let piece;
    match CHAR_PIECES.get("K") {
        Some(a) => piece = a,
        None => piece = &Piece::BlackBishop
    };

    println!("{:?}", piece);
}
