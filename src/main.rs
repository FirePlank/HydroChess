mod bitboard;
use bitboard::*;

use std::io;
use std::io::Write;
use std::io::stdin;

fn main() {
    let mut bitboard = Bitboard(0);
    bitboard.print();
}
