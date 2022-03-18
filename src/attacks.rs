use super::bitboard::*;

pub struct Side;
#[allow(dead_code)]
impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

// make A file be 0s
const NOT_A_FILE: Bitboard = Bitboard(18374403900871474942);
// make H file be 0s
const NOT_H_FILE: Bitboard = Bitboard(9187201950435737471);
// make H and G files be 0s
const NOT_HG_FILE: Bitboard = Bitboard(4557430888798830399);
// make A and B files be 0s
const NOT_AB_FILE: Bitboard = Bitboard(18229723555195321596);

pub static mut PAWN_ATTACKS: [[u64;64];2] = [[0;64],[0;64]];

pub fn mask_pawn_attacks(square: usize, side: usize) -> Bitboard {
    let mut attacks = Bitboard(0);     // result attacks bitboard
    let mut bitboard = Bitboard(0);    // piece bitboard
    // set piece on board
    set_bit(&mut bitboard, square);             
    // bitboard.show();

    if side == Side::WHITE {
        if (bitboard.0 >> 7) & NOT_A_FILE.0 != 0 {
            attacks.0 |= bitboard.0 >> 7;
        } if (bitboard.0 >> 9) & NOT_H_FILE.0 != 0{
            attacks.0 |= bitboard.0 >> 9;
        }
    } else {
        if (bitboard.0 << 7) & NOT_H_FILE.0 != 0 {
            attacks.0 |= bitboard.0 << 7;
        } if (bitboard.0 << 9) & NOT_A_FILE.0 != 0{
            attacks.0 |= bitboard.0 << 9;
        }
    }
    return attacks;
}

pub fn init_leapers_attacks() {
    // loop over 64 board squares
    for square in 0..64 {
        unsafe {
            // init pawn attacks
            PAWN_ATTACKS[Side::WHITE][square] = mask_pawn_attacks(square, Side::WHITE).0;
            PAWN_ATTACKS[Side::BLACK][square] = mask_pawn_attacks(square, Side::BLACK).0;
        }
    }
}