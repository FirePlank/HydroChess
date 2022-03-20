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
    
    let mut occupancy = Bitboard(0);
    occupancy.set(Square::B4 as usize);
    Bitboard(get_queen_attacks(Square::D4 as usize, occupancy)).show();
}
