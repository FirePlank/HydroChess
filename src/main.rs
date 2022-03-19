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
    occupancy.set(Square::D2 as usize);
    occupancy.show();

    // print bishop attacks
    Bitboard(get_rook_attacks(Square::D4 as usize, &mut occupancy)).show();

}
