// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;
use crate::movegen::*;
use crate::search::*;

// evaluation function
pub fn evaluate(position: Position) -> i16 {
    let mut score = 0;

    // add material score
    score += position.material_scores[0] - position.material_scores[1];
    // add piece square table score
    // TODO: make it check if endgame or not so we know which pst table to use
    score += position.pst_scores[0][0] - position.pst_scores[1][0];

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

// calculates material and PST scores in `Position`
pub fn calculate_all(position: &mut Position) {
    for color_index in 0..2 {
        let mut score = 0;
        let mut pst_score = 0;
        let mut pst_eg_score = 0;
        for piece_index in 0..6 {
            let index = if color_index == 0 { piece_index } else { piece_index + 6 };
            let mut bitboard = position.bitboards[index];
            while bitboard.0 != 0 {
                let square = bitboard.ls1b();

                score += unsafe { PIECE_VALUE[piece_index] };
                if color_index == 0 {
                    pst_score += PSQT[piece_index][square as usize];
                    pst_eg_score += PSQT_EG[piece_index][square as usize];
                } else {
                    pst_score += PSQT[piece_index][(square^56) as usize];
                    pst_eg_score += PSQT_EG[piece_index][(square^56) as usize];
                }

                bitboard.pop(square as usize);
            }
        }
        position.pst_scores[color_index][0] = pst_score;
        position.pst_scores[color_index][1] = pst_eg_score;
        position.material_scores[color_index] = score as i16;
    }
}

// calculates material score in `Position`
pub fn calculate_material(position: &mut Position) {
    for color_index in 0..2 {
        let mut score = 0;
        for piece_index in 0..6 {
            let index = if color_index == 0 { piece_index } else { piece_index + 6 };
            let mut bitboard = position.bitboards[index];
            while bitboard.0 != 0 {
                let piece = index;
                let square = bitboard.ls1b();

                score += unsafe { PIECE_VALUE[piece_index] };

                bitboard.pop(square as usize);
            }
        }
        position.material_scores[color_index] = score as i16;
    }
}