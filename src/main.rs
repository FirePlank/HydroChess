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

    for square in 0..64 {
        unsafe {
            Bitboard(PAWN_ATTACKS[Side::BLACK][square]).show();
        }
    }
}
