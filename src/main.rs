mod bitboard;
use bitboard::*;

use std::time::Instant;

mod attacks;
use attacks::*;

use std::io;
use std::io::Write;
use std::io::stdin;

fn main() {
    init_leapers_attacks();

    // init occupancy bitboard
    let mut block = Bitboard(0);
    block.set(Square::D7 as usize);
    block.set(Square::D2 as usize);
    block.set(Square::C4 as usize);
    block.set(Square::G4 as usize);

    fly_rook_attacks(Square::D4 as i32, block).show();
}
