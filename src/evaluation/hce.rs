// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;
use crate::movegen::*;
use crate::search::*;

// const material_score: [i32; 12] = [
//     100,      // white pawn score
//     300,      // white knight scrore
//     350,      // white bishop score
//     500,      // white rook score
//    1000,      // white queen score
//   10000,      // white king score
//    -100,      // black pawn score
//    -300,      // black knight scrore
//    -350,      // black bishop score
//    -500,      // black rook score
//   -1000,      // black queen score
//  -10000,      // black king score
// ];

// evaluation function
pub fn evaluate(position: Position) -> i16 {
    let mut score = 0;

    // add material score
    score += position.material_scores[0] - position.material_scores[1];

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

// Recalculates incremental counters in `board`. This function should be called only during board initialization, as it's too slow in regular search.
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