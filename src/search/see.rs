use crate::evaluation::*;

pub fn get_lsb(x: u64) -> u64 {
    return x & x.wrapping_neg();
}

pub fn evaluate(target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    if attackers == 0 {
        return 0;
    }

    return evaluate_internal(get_lsb(attackers as u64).trailing_zeros() as u8, get_see_piece_index(target_piece), attackers, defenders);
}

pub fn evaluate_internal(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    if attackers == 0 {
        return 0;
    }
    
    let score = evaluate_internal(if defenders == 0 { 0 } else { get_lsb(defenders as u64).trailing_zeros() } as u8, attacking_piece, defenders, attackers & !(1 << attacking_piece));
    return 0.max(get_piece_value(target_piece) - score);
}

pub static mut SEE_TABLE: [[[i16; 256];256];6] = [[[0;256];256];6];

pub fn init_see() {
    for a in 0..6 {
        for b in 0..256 {
            for c in 0..256 {
                unsafe {
                    SEE_TABLE[a][b][c] = evaluate(a as u8, b as u8, c as u8);
                }
            }
        }
    }
}

pub fn get_see(attacking_piece: u8, target_piece: u8, attackers: u8, defenders: u8) -> i16 {
    let attacking_piece_index = get_see_piece_index(attacking_piece);
    let target_piece_index = get_see_piece_index(target_piece);
    let updated_attackers = attackers & !(1 << attacking_piece_index);

    let see_result = unsafe { SEE_TABLE[attacking_piece as usize][defenders as usize][updated_attackers as usize] };
    get_piece_value(target_piece_index) - see_result
}

fn get_see_piece_index(piece: u8) -> u8 {
    match piece {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 4,
        4 => 6,
        5 => 7,
        _ => panic!("Invalid value: piece={}", piece),
    }
}

fn get_piece_value(piece_index: u8) -> i16 {
    match piece_index {
        0 => PIECE_VALUE[0] as i16,           // Pawn
        1 | 2 | 3 => PIECE_VALUE[2] as i16, // 3x Knight/bishop
        4 | 5 => PIECE_VALUE[3] as i16,       // 2x Rook
        6 => PIECE_VALUE[4] as i16,          // Queen
        7 => PIECE_VALUE[5] as i16,           // King
        _ => panic!("Invalid value: piece_index={}", piece_index),
    }
}
