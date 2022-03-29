// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;
use crate::movegen::*;
use crate::search::*;

pub static mut MASKS: Masks = Masks::new();

pub struct Masks {
    pub file_masks: [u64; 64],
    pub rank_masks: [u64; 64],
    pub isolated_masks: [u64; 64],
    pub white_passed_masks: [u64; 64],
    pub black_passed_masks: [u64; 64],
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
    let phase = position.phase() <= 7;
    if phase {
        // add material score
        score += position.material_scores[0][1] - position.material_scores[1][1]; 
        // add piece square table score
        score += position.pst_scores[0][1] - position.pst_scores[1][1];
        // add double pawn score
        score += (position.bitboards[0].0 & position.bitboards[0].0 << 8).count_ones() as i16 * DOUBLED_PAWN_ENDING - 
        (position.bitboards[Piece::BlackPawn as usize].0 & position.bitboards[Piece::BlackPawn as usize].0 << 8).count_ones() as i16 * DOUBLED_PAWN_ENDING;
        // add mobility score
        score += position.mobility[2] * BISHOP - position.mobility[8] * BISHOP;
        score += position.mobility[3] * ROOK_EG - position.mobility[9] * ROOK_EG;
        score += position.mobility[4] * QUEEN_EG - position.mobility[10] * QUEEN_EG;
        // add score to get king closer to the other for mate
        score += force_king_corner(&position);
    } else {
        // add material score
        score += position.material_scores[0][0] - position.material_scores[1][0]; 
        // add piece square table score
        score += position.pst_scores[0][0] - position.pst_scores[1][0];
        // add double pawn score
        score += (position.bitboards[0].0 & position.bitboards[0].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING - 
        (position.bitboards[Piece::BlackPawn as usize].0 & position.bitboards[Piece::BlackPawn as usize].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING;

        score += position.mobility[2] * BISHOP - position.mobility[8] * BISHOP;
        score += position.mobility[3] * ROOK - position.mobility[9] * ROOK;
        score += position.mobility[4] * QUEEN - position.mobility[10] * QUEEN;
    }
    score += calculate_all(&position, phase);

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    }

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

// pub fn calculate_mobility(position: &Position, phase: bool) -> i16 {
//     let mut score = 0;

//     // add bishop mobility
//     score += get_bishop_attacks(square, occupancy);

//     // return score
//     return score;
// }

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

    // favour positions where the opponent king has been forced into the edge of the board
    // this makes the bot be able to checkmate easier in the endgame
    let opponent_square = if position.side == 0 { position.bitboards[Piece::BlackKing as usize].ls1b() } else {
        position.bitboards[Piece::WhiteKing as usize].ls1b()
    };
    let opponent_rank = GET_RANK[opponent_square as usize];
    let opponent_file = opponent_square % 8;

    eval += (3 - opponent_file).max(opponent_file - 4) as f32 + (3 - opponent_rank).max(opponent_rank - 4) as f32;

    // Incentivize moving king closer to opponent king
    let king_square = if position.side == 0 { position.bitboards[Piece::WhiteKing as usize].ls1b() } else {
        position.bitboards[Piece::BlackKing as usize].ls1b()
    };
    let king_rank = GET_RANK[king_square as usize];
    let king_file = king_square % 8;

    eval += 14.0 - ((king_file - opponent_file).abs() as f32 + (king_rank - opponent_rank).abs() as f32);

    return if position.side == 0 { (eval * 3.0 * (1.25-(position.phase()as f32/24.0))) as i16 } else { -(eval * 3.0 * (1.25-(position.phase()as f32/24.0))) as i16 };
}

// calculates PST and piece av
pub fn init_calculation(position: &mut Position) {
    let both = Bitboard(position.occupancies[0].0 | position.occupancies[1].0);
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

                match piece_index {
                    // mobility
                    2 => {
                        if color_index == 0 {
                            position.mobility[2] = get_bishop_attacks(square as usize, both).count_ones() as i16 - 7;
                        } else {
                            position.mobility[8] = get_bishop_attacks(square as usize, both).count_ones() as i16 - 7;
                        }
                    }
                    3 => {
                        if color_index == 0 {
                            position.mobility[3] = get_rook_attacks(square as usize, both).count_ones() as i16 - 7;
                        } else {
                            position.mobility[9] = get_rook_attacks(square as usize, both).count_ones() as i16 - 7;
                        }
                    }
                    4 => {
                        if color_index == 0 {
                            position.mobility[4] = get_queen_attacks(square as usize, both).count_ones() as i16 - 7;
                        } else {
                            position.mobility[10] = get_queen_attacks(square as usize, both).count_ones() as i16 - 7;
                        }
                    },
                    _ => ()
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

// calculates all the different evaluation scores for the given position
pub fn calculate_all(position: &Position, phase: bool) -> i16 {
    let mut score = 0;
    if phase {
        for piece_index in 0..Piece::BlackPawn as usize {
            let mut bitboard = position.bitboards[piece_index];
            while bitboard.0 != 0 {
                let square = bitboard.ls1b();
                
                unsafe {
                    if piece_index == 0 {
                        // isolated pawns and passed pawns
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.isolated_masks[square as usize]) == 0 {
                            score += ISOLATED_PAWN_ENDING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.isolated_masks[square as usize]) == 0 {
                                score -= ISOLATED_PAWN_ENDING;
                            }
                        }
    
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                            score += PASSED_PAWN_ENDING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                                score -= PASSED_PAWN_ENDING;
                            }
                        }
                    }
                }
                bitboard.pop(square as usize);
            }
        }
        return score;
    }
    for piece_index in 0..Piece::BlackPawn as usize {
        let mut bitboard = position.bitboards[piece_index];
        while bitboard.0 != 0 {
            let square = bitboard.ls1b();
            
            unsafe {
                match piece_index {
                    0 => {
                        // isolated pawns and passed pawns
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.isolated_masks[square as usize]) == 0 {
                            score += ISOLATED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.isolated_masks[square as usize]) == 0 {
                                score -= ISOLATED_PAWN_OPENING;
                            }
                        }

                        if position.side == 0 && (position.bitboards[0].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                            score += PASSED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                                score -= PASSED_PAWN_OPENING;
                            }
                        }
                    },
                    3 | 9 => {
                        // open files
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                score += OPEN_FILE;
                            } else {
                                score += SEMI_OPEN_FILE;
                            }
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                                    score -= OPEN_FILE;
                                } else {
                                    score -= SEMI_OPEN_FILE;
                                }
                            }
                        }
                    },
                    5 | 11 => {
                        // open file penalties and king safety bonus
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                score -= OPEN_FILE_PENALTY;
                            } else {
                                score -= SEMI_OPEN_FILE_PENALTY;
                            }
                            score += (KING_ATTACKS[square as usize] & position.occupancies[0].0).count_ones() as i16 * KING_SHIELD;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                                    score += OPEN_FILE_PENALTY;
                                } else {
                                    score += SEMI_OPEN_FILE_PENALTY;
                                }
                            }
                            score -= (KING_ATTACKS[square as usize] & position.occupancies[0].0).count_ones() as i16 * KING_SHIELD;
                        }
                    }
                    _ => ()
                }
            }
            bitboard.pop(square as usize);
        }
    }
    return score;
}