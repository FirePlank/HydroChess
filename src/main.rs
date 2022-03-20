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
    let mut pos = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    pos.show(true);
}
