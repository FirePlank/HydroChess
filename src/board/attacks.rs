use super::bitboard::*;
use super::magic::*;
use super::position::*;

// make A file be 0s
const NOT_A_FILE: Bitboard = Bitboard(18374403900871474942);
// make H file be 0s
const NOT_H_FILE: Bitboard = Bitboard(9187201950435737471);
// make H and G files be 0s
const NOT_HG_FILE: Bitboard = Bitboard(4557430888798830399);
// make A and B files be 0s
const NOT_AB_FILE: Bitboard = Bitboard(18229723555195321596);

// pawn attack table [side][square]
pub static mut PAWN_ATTACKS: [[u64; 64]; 2] = [[0; 64], [0; 64]];
// knight attack table [square]
pub static mut KNIGHT_ATTACKS: [u64; 64] = [0; 64];
// king attack table [square]
pub static mut KING_ATTACKS: [u64; 64] = [0; 64];

// bishop attack masks
pub static mut BISHOP_MASKS: [u64; 64] = [0; 64];
// bishop attack masks
pub static mut ROOK_MASKS: [u64; 64] = [0; 64];

// bishop attack table [square][occupancies]
pub static mut BISHOP_ATTACKS: [[u64; 512]; 64] = [[0; 512]; 64];
// rook attack table [square][occupancies]
pub static mut ROOK_ATTACKS: [[u64; 4096]; 64] = [[0; 4096]; 64];

// generate pawn attacks
pub fn mask_pawn_attacks(square: usize, side: usize) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard
    let mut bitboard = Bitboard(0); // piece bitboard

    bitboard.set(square); // set piece on board

    if side == Side::WHITE {
        // generate white pawn attacks
        // dont place attack if it will be outside the board
        if (bitboard.0 >> 7) & NOT_A_FILE.0 != 0 {
            attacks.0 |= bitboard.0 >> 7;
        }
        if (bitboard.0 >> 9) & NOT_H_FILE.0 != 0 {
            attacks.0 |= bitboard.0 >> 9;
        }
    } else {
        // generate black pawn attacks
        // dont place attack if it will be outside the board
        if (bitboard.0 << 7) & NOT_H_FILE.0 != 0 {
            attacks.0 |= bitboard.0 << 7;
        }
        if (bitboard.0 << 9) & NOT_A_FILE.0 != 0 {
            attacks.0 |= bitboard.0 << 9;
        }
    }
    // return attack map
    return attacks;
}

// generate knight attacks
pub fn mask_knight_attacks(square: usize) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard
    let mut bitboard = Bitboard(0); // piece bitboard

    bitboard.set(square); // set piece on board

    // generate knight attacks (OFFSETS: 17, 15, 10, 6)
    if bitboard.0 << 17 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 17;
    }
    if bitboard.0 << 15 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 15;
    }
    if bitboard.0 << 10 & NOT_AB_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 10;
    }
    if bitboard.0 << 6 & NOT_HG_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 6;
    }
    if bitboard.0 >> 17 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 17;
    }
    if bitboard.0 >> 15 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 15;
    }
    if bitboard.0 >> 10 & NOT_HG_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 10;
    }
    if bitboard.0 >> 6 & NOT_AB_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 6;
    }

    // return attack map
    return attacks;
}

// generate king moves
pub fn mask_king_attack(square: usize) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard
    let mut bitboard = Bitboard(0); // piece bitboard

    bitboard.set(square); // set piece on board

    // generate king attacks
    if bitboard.0 >> 8 != 0 {
        attacks.0 |= bitboard.0 >> 8;
    }
    if bitboard.0 >> 9 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 9;
    }
    if bitboard.0 >> 7 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 7;
    }
    if bitboard.0 >> 1 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 >> 1;
    }

    if bitboard.0 << 8 != 0 {
        attacks.0 |= bitboard.0 << 8;
    }
    if bitboard.0 << 9 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 9;
    }
    if bitboard.0 << 7 & NOT_H_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 7;
    }
    if bitboard.0 << 1 & NOT_A_FILE.0 != 0 {
        attacks.0 |= bitboard.0 << 1;
    }

    // return attack map
    return attacks;
}

// mask bishop attacks
pub fn mask_bishop_attacks(square: i32) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard

    // init target rank & files
    let tr = square / 8;
    let tf = square % 8;
    // init ranks & files
    let mut r = tr + 1;
    let mut f = tf + 1;

    // mask relevant bishop occupancy bits
    while r <= 6 && f <= 6 {
        attacks.0 |= 1 << (r * 8 + f);
        r += 1;
        f += 1;
    }
    r = tr - 1;
    f = tf + 1;
    while r >= 1 && f <= 6 {
        attacks.0 |= 1 << (r * 8 + f);
        r -= 1;
        f += 1;
    }
    r = tr + 1;
    f = tf - 1;
    while r <= 6 && f >= 1 {
        attacks.0 |= 1 << (r * 8 + f);
        r += 1;
        f -= 1;
    }
    r = tr - 1;
    f = tf - 1;
    while r >= 1 && f >= 1 {
        attacks.0 |= 1 << (r * 8 + f);
        r -= 1;
        f -= 1;
    }

    // return attack map
    return attacks;
}

// mask rook attacks
pub fn mask_rook_attacks(square: i32) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard

    // init target rank & files
    let tr = square / 8;
    let tf = square % 8;
    // init ranks & files
    let mut r = tr + 1;
    let mut f = tf + 1;

    // mask relevant rook occupancy bits
    while r <= 6 {
        attacks.0 |= 1 << (r * 8 + tf);
        r += 1
    }
    r = tr - 1;
    while r >= 1 {
        attacks.0 |= 1 << (r * 8 + tf);
        r -= 1
    }
    while f <= 6 {
        attacks.0 |= 1 << (tr * 8 + f);
        f += 1
    }
    f = tf - 1;
    while f >= 1 {
        attacks.0 |= 1 << (tr * 8 + f);
        f -= 1
    }

    // return attack map
    return attacks;
}

// generate bishop attacks on the fly
pub fn fly_bishop_attacks(square: i32, block: Bitboard) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard

    // init target rank & files
    let tr = square / 8;
    let tf = square % 8;
    // init ranks & files
    let mut r = tr + 1;
    let mut f = tf + 1;

    // generate bishop attacks
    while r <= 7 && f <= 7 {
        attacks.0 |= 1 << (r * 8 + f);
        // check if blocked
        if 1 << (r * 8 + f) & block.0 != 0 {
            break;
        }
        r += 1;
        f += 1;
    }
    r = tr - 1;
    f = tf + 1;
    while r >= 0 && f <= 7 {
        attacks.0 |= 1 << (r * 8 + f);
        // check if blocked
        if 1 << (r * 8 + f) & block.0 != 0 {
            break;
        }
        r -= 1;
        f += 1;
    }
    r = tr + 1;
    f = tf - 1;
    while r <= 7 && f >= 0 {
        attacks.0 |= 1 << (r * 8 + f);
        // check if blocked
        if 1 << (r * 8 + f) & block.0 != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }
    r = tr - 1;
    f = tf - 1;
    while r >= 0 && f >= 0 {
        attacks.0 |= 1 << (r * 8 + f);
        // check if blocked
        if 1 << (r * 8 + f) & block.0 != 0 {
            break;
        }
        r -= 1;
        f -= 1;
    }

    // return attack map
    return attacks;
}

// generate rook attacks on the fly
pub fn fly_rook_attacks(square: i32, block: Bitboard) -> Bitboard {
    let mut attacks = Bitboard(0); // result attacks bitboard

    // init target rank & files
    let tr = square / 8;
    let tf = square % 8;
    // init ranks & files
    let mut r = tr + 1;
    let mut f = tf + 1;

    // generate rook attacks
    while r <= 7 {
        attacks.0 |= 1 << (r * 8 + tf);
        // check if blocked
        if ((1 << (r * 8 + tf)) & block.0) != 0 {
            break;
        }
        r += 1
    }
    r = tr - 1;
    while r >= 0 {
        attacks.0 |= 1 << (r * 8 + tf);
        // check if blocked
        if ((1 << (r * 8 + tf)) & block.0) != 0 {
            break;
        }
        r -= 1
    }
    while f <= 7 {
        attacks.0 |= 1 << (tr * 8 + f);
        // check if blocked
        if ((1 << (tr * 8 + f)) & block.0) != 0 {
            break;
        }
        f += 1;
    }
    f = tf - 1;
    while f >= 0 {
        attacks.0 |= 1 << (tr * 8 + f);
        // check if blocked
        if ((1 << (tr * 8 + f)) & block.0) != 0 {
            break;
        }
        f -= 1;
    }

    // return attack map
    return attacks;
}

// get bishop attacks
pub fn get_bishop_attacks(square: usize, mut occupancy: Bitboard) -> u64 {
    unsafe {
        // get bishop attacks assuming current board occupancy
        occupancy.0 &= BISHOP_MASKS[square];
        occupancy.0 = occupancy.0.wrapping_mul(MAGIC_BISHOP[square]);
        occupancy.0 >>= 64 - BISHOP_BITS[square];

        // return bishop attacks

        return BISHOP_ATTACKS[square][occupancy.0 as usize];
    }
}
// get rook attacks
pub fn get_rook_attacks(square: usize, mut occupancy: Bitboard) -> u64 {
    unsafe {
        // get rook attacks assuming current board occupancy
        occupancy.0 &= ROOK_MASKS[square];
        occupancy.0 = occupancy.0.wrapping_mul(MAGIC_ROOK[square]);
        occupancy.0 >>= 64 - ROOK_BITS[square];

        // return rook attacks
        return ROOK_ATTACKS[square][occupancy.0 as usize];
    }
}

// get queen attacks
pub fn get_queen_attacks(square: usize, occupancy: Bitboard) -> u64 {
    unsafe {
        // init bishop occupancies
        let mut bishop_occupancies = occupancy;
        // init rook occupancies
        let mut rook_occupancies = occupancy;

        // get bishop attacks assuming current board occupancy
        bishop_occupancies.0 &= BISHOP_MASKS[square];
        bishop_occupancies.0 = bishop_occupancies.0.wrapping_mul(MAGIC_BISHOP[square]);
        bishop_occupancies.0 >>= 64 - BISHOP_BITS[square];
        // get bishop attacks
        let mut result = BISHOP_ATTACKS[square][bishop_occupancies.0 as usize];

        // get rook attacks assuming current board occupancy
        rook_occupancies.0 &= ROOK_MASKS[square];
        rook_occupancies.0 = rook_occupancies.0.wrapping_mul(MAGIC_ROOK[square]);
        rook_occupancies.0 >>= 64 - ROOK_BITS[square];
        // get rook attacks
        result |= ROOK_ATTACKS[square][rook_occupancies.0 as usize];

        // return queen attacks
        return result;
    }
}

// init slider piece's attack tables
pub fn init_sliders_attacks(bishop: bool) {
    // loop over 64 board squares
    for square in 0..64 {
        unsafe {
            // init bishop & rook masks
            BISHOP_MASKS[square] = mask_bishop_attacks(square as i32).0;
            ROOK_MASKS[square] = mask_rook_attacks(square as i32).0;

            // init current mask
            let attack_mask: Bitboard = if bishop {
                Bitboard(BISHOP_MASKS[square])
            } else {
                Bitboard(ROOK_MASKS[square])
            };

            // init relevant occupancy bit count
            let relevant_bits = attack_mask.count();
            // init occupancy indicies
            let occupancy_indicies = 1 << relevant_bits;

            // loop over occupancy indicies
            for index in 0..occupancy_indicies {
                if bishop {
                    // bishop
                    // init current occupancy variation
                    let occupancy = set_occupancy(index, relevant_bits, attack_mask);
                    // init magic index
                    let magic_index = (occupancy.0.wrapping_mul(MAGIC_BISHOP[square]))
                        >> (64 - BISHOP_BITS[square]);
                    // init bishop attacks
                    BISHOP_ATTACKS[square][magic_index as usize] =
                        fly_bishop_attacks(square as i32, occupancy).0;
                } else {
                    // rook
                    // init current occupancy variation
                    let occupancy = set_occupancy(index, relevant_bits, attack_mask);
                    // init magic index
                    let magic_index =
                        (occupancy.0.wrapping_mul(MAGIC_ROOK[square])) >> (64 - ROOK_BITS[square]);
                    // init rook attacks
                    ROOK_ATTACKS[square][magic_index as usize] =
                        fly_rook_attacks(square as i32, occupancy).0;
                }
            }
        }
    }
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

pub fn init_all() {
    // init leaper pieces attacks
    init_leapers_attacks();

    // init slider pieces attacks
    init_sliders_attacks(true); // bishop
    init_sliders_attacks(false); // rook
}
