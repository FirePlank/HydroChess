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
    occupancy.set(Square::F2 as usize);

    // print bishop attacks
    Bitboard(get_bishop_attacks(Square::D4 as usize, &mut occupancy)).show();
    // occupancy.show();


    // for rank in 0..8 {
    //     for file in 0..8 {
    //         let square = rank * 8 + file;

    //         print!("{}, ", mask_rook_attacks(square).count());
    //     }
    //     println!();
    // }

    // find first one bit in u64 trailing zero
    // let mut i = 0;
    // while i < 64 {
    //     if block.0 & (1 << i) != 0 {
    //         println!("{}", i);
    //         break;
    //     }
    //     i += 1;
    // }
    //Bitboard(lowest).show();
    //block.show();
    // fly_rook_attacks(Square::D4 as i32, block).show();
}
