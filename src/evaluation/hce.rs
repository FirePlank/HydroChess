// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;
use crate::movegen::*;
use crate::search::*;

// evaluation function
pub fn evaluate(position: &Position) -> i16 {
    let mut score = 0;

    let eg = position.phase() <= 7;

    if eg {
        // add material score
        score += position.material_scores[0][1] - position.material_scores[1][1]; 
        // add piece square table score
        score += position.pst_scores[0][1] - position.pst_scores[1][1];
    } else {
        // add material score
        score += position.material_scores[0][0] - position.material_scores[1][0]; 
        // add piece square table score
        score += position.pst_scores[0][0] - position.pst_scores[1][0];
    }

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    }

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

pub fn force_king_corner(position: &Position) -> i16 {
    let mut eval = 0.0;

    // favor positions where the opponent king has been forced into the edge of the board
    // this makes the bot be able to checkmate easier in the endgame
    return (eval * 10.0 * (1.0-(position.phase()as f32/24.0))) as i16;
}

// calculates material and PST scores in `Position`
pub fn calculate_all(position: &mut Position) {
    for color_index in 0..2 {
        let mut score = 0;
        let mut score_eg = 0;
        let mut pst_score = 0;
        let mut pst_eg_score = 0;
        for piece_index in 0..6 {
            let index = if color_index == 0 { piece_index } else { piece_index + 6 };
            let mut bitboard = position.bitboards[index];
            while bitboard.0 != 0 {
                let square = bitboard.ls1b();

                score += PIECE_VALUE[piece_index];
                score_eg += PIECE_VALUE_EG[piece_index];
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
        position.material_scores[color_index][0] = score;
        position.material_scores[color_index][1] = score_eg;
    }
}

// calculates material score in `Position`
pub fn calculate_material(position: &mut Position) {
    for color_index in 0..2 {
        let mut score = 0;
        let mut score_eg = 0;
        for piece_index in 0..6 {
            let index = if color_index == 0 { piece_index } else { piece_index + 6 };
            let mut bitboard = position.bitboards[index];
            while bitboard.0 != 0 {
                let square = bitboard.ls1b();

                score += PIECE_VALUE[piece_index];
                score_eg += PIECE_VALUE_EG[piece_index];

                bitboard.pop(square as usize);
            }
        }
        position.material_scores[color_index][0] = score;
        position.material_scores[color_index][1] = score_eg;
    }
}