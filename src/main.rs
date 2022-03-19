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

    // mask_bishop_attacks(Square::D4 as usize).show();
    for square in 0..64 {
        mask_rook_attacks(square).show();
    }
}
