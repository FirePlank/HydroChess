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
    
    let pos = Position::from_fen("8/8/8/3K4/8/8/8/8 w - -");
    pos.show_attacked(Side::WHITE as usize);
}
