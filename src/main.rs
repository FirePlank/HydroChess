mod bitboard;
use bitboard::*;
mod attacks;
use attacks::*;
mod magic;
use magic::*;

// use std::time::Instant;
// use std::io;
// use std::io::Write;
// use std::io::stdin;

fn main() {
    init_all();

    // define test bitboard
    let mut occupancy = Bitboard(0);
    occupancy.set(Square::C5 as usize);
    occupancy.set(Square::F2 as usize);
    occupancy.set(Square::G7 as usize);
    occupancy.set(Square::B2 as usize);

    Bitboard(get_rook_attacks(Square::E5 as usize, &mut occupancy)).show();
}
