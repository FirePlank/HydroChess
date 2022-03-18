use super::bitboard::*;

pub struct Side;
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

// pawn attack table [side][square]
pub static mut PAWN_ATTACKS: [[u64;64];2] = [[0;64],[0;64]];
// knight attack table [square]
pub static mut KNIGHT_ATTACKS: [u64;64] = [0;64];
// king attack table [square]
pub static mut KING_ATTACKS: [u64;64] = [0;64];

// generate pawn attacks
pub fn mask_pawn_attacks(square: usize, side: usize) -> Bitboard {
    let mut attacks = Bitboard(0);     // result attacks bitboard
    let mut bitboard = Bitboard(0);    // piece bitboard

    set_bit(&mut bitboard, square); // set piece on board

    if side == Side::WHITE {
        // generate white pawn attacks
        // dont place attack if it will be outside the board
        if (bitboard.0 >> 7) & NOT_A_FILE.0 != 0 {
            attacks.0 |= bitboard.0 >> 7;
        } if (bitboard.0 >> 9) & NOT_H_FILE.0 != 0{
            attacks.0 |= bitboard.0 >> 9;
        }
    } else {
        // generate black pawn attacks
        // dont place attack if it will be outside the board
        if (bitboard.0 << 7) & NOT_H_FILE.0 != 0 {
            attacks.0 |= bitboard.0 << 7;
        } if (bitboard.0 << 9) & NOT_A_FILE.0 != 0{
            attacks.0 |= bitboard.0 << 9;
        }
    }
    // return attack map
    return attacks;
}

// generate knight attacks
pub fn mask_knight_attacks(square: usize) -> Bitboard {
    let mut attacks = Bitboard(0);     // result attacks bitboard
    let mut bitboard = Bitboard(0);    // piece bitboard

    set_bit(&mut bitboard, square); // set piece on board

    // generate knight attacks (OFFSETS: 17, 15, 10, 6)
    if bitboard.0 << 17 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 17;
    } if bitboard.0 << 15 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 15;
    } if bitboard.0 << 10 & NOT_AB_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 10;
    } if bitboard.0 << 6 & NOT_HG_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 6;

    } if bitboard.0 >> 17 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 17;
    } if bitboard.0 >> 15 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 15;
    } if bitboard.0 >> 10 & NOT_HG_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 10;
    } if bitboard.0 >> 6 & NOT_AB_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 6;
    }

    // return attack map
    return attacks;
}

// generate king moves
pub fn mask_king_attack(square: usize) -> Bitboard {
    let mut attacks = Bitboard(0);     // result attacks bitboard
    let mut bitboard = Bitboard(0);    // piece bitboard

    set_bit(&mut bitboard, square); // set piece on board

    // generate king attacks
    if bitboard.0 >> 8 != 0 {
        attacks.0 |= bitboard.0 >> 8;
    } if bitboard.0 >> 9 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 9;
    } if bitboard.0 >> 7 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 7;
    } if bitboard.0 >> 1 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 1;
    }

    if bitboard.0 << 8 != 0 {
        attacks.0 |= bitboard.0 << 8;
    } if bitboard.0 << 9 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 9;
    } if bitboard.0 << 7 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 7;
    } if bitboard.0 << 1 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 1;
    }

    // return attack map
    return attacks;
}

pub fn init_leapers_attacks() {
    // loop over 64 board squares
    for square in 0..64 {
        unsafe {
            // init pawn attacks
            PAWN_ATTACKS[Side::WHITE][square] = mask_pawn_attacks(square, Side::WHITE).0;
            PAWN_ATTACKS[Side::BLACK][square] = mask_pawn_attacks(square, Side::BLACK).0;

            // init knight attacks
            KNIGHT_ATTACKS[square] = mask_knight_attacks(square).0;

            // init king attacks
            KING_ATTACKS[square] = mask_king_attack(square).0;
        }
    }
}