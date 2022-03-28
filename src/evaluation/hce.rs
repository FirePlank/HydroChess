// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;
use crate::movegen::*;
use crate::search::*;

pub static mut MASKS: Masks = Masks::new();

#[derive(Debug, Clone)]
pub struct Eval {
    pub material_scores: [[i16; 2]; 2],
    pub pst_scores: [[i16; 2]; 2],
    pub isolated_pawns: [[i16; 2]; 2],
    pub double_pawns: [[i16; 2]; 2],
    pub passed_pawns: [[i16; 2]; 2],
}

pub struct Masks {
    pub file_masks: [u64; 64],
    pub rank_masks: [u64; 64],
    pub isolated_masks: [u64; 64],
    pub white_passed_masks: [u64; 64],
    pub black_passed_masks: [u64; 64],
}

impl Eval {
    pub const fn new() -> Eval {
        Eval {
            material_scores: [[14560, 14740];2],
            pst_scores: [[348, -92]; 2],
            isolated_pawns: [[0, 0],[0, 0]],
            double_pawns: [[0, 0],[0, 0]],
            passed_pawns: [[0, 0],[0, 0]],
        }
    }
    pub const fn empty() -> Eval {
        Eval {
            material_scores: [[0, 0];2],
            pst_scores: [[0, 0]; 2],
            isolated_pawns: [[0, 0]; 2],
            double_pawns: [[0, 0]; 2],
            passed_pawns: [[0, 0]; 2],
        }
    }
}

impl Masks {
    pub const fn new() -> Masks {
        Masks {
            file_masks: [0; 64],
            rank_masks: [0; 64],
            isolated_masks: [0; 64],
            white_passed_masks: [0; 64],
            black_passed_masks: [0; 64],
        }
    }
}

// evaluation function
pub fn evaluate(position: &Position) -> i16 {
    let mut score = 0;

    let eg = position.phase() <= 7;

    if eg {
        // add material score
        score += position.eval.material_scores[0][1] - position.eval.material_scores[1][1]; 
        // add piece square table score
        score += position.eval.pst_scores[0][1] - position.eval.pst_scores[1][1];
        // add isolated pawn score
        score -= position.eval.isolated_pawns[0][1] + position.eval.isolated_pawns[1][1];
        // add passed pawn score
        score += position.eval.passed_pawns[0][1] + position.eval.passed_pawns[1][1];
    } else {
        // add material score
        score += position.eval.material_scores[0][0] - position.eval.material_scores[1][0]; 
        // add piece square table score
        score += position.eval.pst_scores[0][0] - position.eval.pst_scores[1][0];
        // add isolated pawn score
        score -= position.eval.isolated_pawns[0][0] + position.eval.isolated_pawns[1][0];
        // add passed pawn score
        score += position.eval.passed_pawns[0][0] - position.eval.passed_pawns[1][0];
    }

    // add double pawn score
    score += (position.bitboards[0].0 & position.bitboards[0].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING - 
    (position.bitboards[Piece::BlackPawn as usize].0 & position.bitboards[Piece::BlackPawn as usize].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING;

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    }

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

// set file or rank mask for a given square
pub fn set_file_rank_mask(file_num: i8, rank_num: i8) -> Bitboard {
    // file or rank mask
    let mut mask = Bitboard(0);

    // loop over ranks
    for rank in 0..8 {
        // loop over files
        for file in 0..8 {
            // init square
            let square = rank * 8 + file;
            if file_num != -1 {
                // on file match
                if file == file_num {
                    mask.set(square as usize);
                }
            } else if rank_num != -1 {
                // on rank match
                if rank == rank_num {
                    mask.set(square as usize);
                }
            }
        }
    }

    // return mask
    return mask;
}

// init evaluation masks
pub fn init_evaluation_masks() -> Masks {
    let mut eval = Masks::new();

    // init file masks
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            // init file mask for a current square
            eval.file_masks[square as usize] |= set_file_rank_mask(file, -1).0;
        }
    }
    // init rank masks
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            // init rank mask for a current square
            eval.rank_masks[square as usize] |= set_file_rank_mask(-1, rank).0;
        }
    }
    // init isolated masks
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            // init isolated mask for a current square
            eval.isolated_masks[square as usize] |= set_file_rank_mask(file-1, -1).0;
            eval.isolated_masks[square as usize] |= set_file_rank_mask(file+1, -1).0;
        }
    }

    // init white passed pawn masks
    for rank in 1..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            // init white passed pawn mask for a current square
            eval.white_passed_masks[square as usize] |= set_file_rank_mask(file-1, -1).0;
            eval.white_passed_masks[square as usize] |= set_file_rank_mask(file, -1).0;
            eval.white_passed_masks[square as usize] |= set_file_rank_mask(file+1, -1).0;
            
            // loop over redundant ranks
            for i in 0..(8 - rank) {
                // reset redundant bits
                eval.white_passed_masks[square as usize] &= !eval.rank_masks[((7-i)*8 + file) as usize];
            }
        }
    }

    // init black passed pawn masks
    for rank in 0..7 {
        for file in 0..8 {
            let square = rank * 8 + file;
            // init black passed pawn mask for a current square
            eval.black_passed_masks[square as usize] |= set_file_rank_mask(file-1, -1).0;
            eval.black_passed_masks[square as usize] |= set_file_rank_mask(file, -1).0;
            eval.black_passed_masks[square as usize] |= set_file_rank_mask(file+1, -1).0;
            
            // loop over redundant ranks
            for i in 0..rank+1 {
                // reset redundant bits
                eval.black_passed_masks[square as usize] &= !eval.rank_masks[(i * 8 + file) as usize];
            }
        }
    }

    return eval;
}

pub fn force_king_corner(position: &Position) -> i16 {
    let mut eval = 0.0;

    // favor positions where the opponent king has been forced into the edge of the board
    // this makes the bot be able to checkmate easier in the endgame
    return (eval * 10.0 * (1.0-(position.phase()as f32/24.0))) as i16;
}

// calculates all the different evaluation scores for the given position
pub fn calculate_all(position: &mut Position) {
    let phase = position.phase() <= 7;
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
                
                unsafe {
                    // isolated pawns and passed pawns
                    if piece_index == 0 {
                        if color_index == 0 && (position.bitboards[0].0 & MASKS.isolated_masks[square as usize]) == 0 {
                            position.eval.isolated_pawns[0][1] += ISOLATED_PAWN_ENDING;
                            position.eval.isolated_pawns[0][0] += ISOLATED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.isolated_masks[square as usize]) == 0 {
                                position.eval.isolated_pawns[1][1] += ISOLATED_PAWN_ENDING;
                                position.eval.isolated_pawns[1][0] += ISOLATED_PAWN_OPENING;
                            }
                        }

                        if color_index == 0 && (position.bitboards[0].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                            position.eval.passed_pawns[0][1] += PASSED_PAWN_ENDING;
                            position.eval.passed_pawns[0][0] += PASSED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                                position.eval.passed_pawns[1][1] += PASSED_PAWN_ENDING;
                                position.eval.passed_pawns[1][0] += PASSED_PAWN_OPENING;
                            }
                        }
                    }
                }

                bitboard.pop(square as usize);
            }
        }
        position.eval.pst_scores[color_index][0] = pst_score;
        position.eval.pst_scores[color_index][1] = pst_eg_score;
        position.eval.material_scores[color_index][0] = score;
        position.eval.material_scores[color_index][1] = score_eg;
    }
}