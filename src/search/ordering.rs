use crate::board::position::Position;
use crate::r#move::*;
use crate::evaluation::*;
use crate::search::*;

use std::cmp::Reverse;

// most valuable victim & less valuable attacker
/*                   
    (Victims) Pawn  Knight  Bishop  Rook  Queen   King
  (Attackers)
        Pawn   105    205    305    405    505    605
      Knight   104    204    304    404    504    604
      Bishop   103    203    303    403    503    603
        Rook   102    202    302    402    502    602
       Queen   101    201    301    401    501    601
        King   100    200    300    400    500    600
*/
// MVV LVA [attacker][victim]
#[rustfmt::skip]
pub const MVV_LVA: [[u16;6];6] = [
    [105, 205, 305, 405, 505, 605,],
	[104, 204, 304, 404, 504, 604,],
	[103, 203, 303, 403, 503, 603,],
	[102, 202, 302, 402, 502, 602,],
	[101, 201, 301, 401, 501, 601,],
	[100, 200, 300, 400, 500, 600,],
];

impl Searcher {
    pub fn sort_moves(&mut self, position: &Position, move_list: MoveList) -> [(u32, u32); 356] {
        // sort moves
        let mut move_scores: [(u32, u32);356] = [(0, 0);356];
    
        // score all the moves within a move list
        for count in 0..move_list.count as usize {
            // score move
            move_scores[count] = (self.score_move(&position, move_list.moves[count]), move_list.moves[count]);
        }
        move_scores.sort_by_key(|w| Reverse(w.0));
        return move_scores;
    }

    pub fn score_move(&mut self, position: &Position, move_: u32) -> u32 {
        // if move scoring is allowed
        if self.score_pv {
            // make sure we are dealing with PV move
            if self.pv_table[0][self.ply as usize] == move_ {
                // disable score PV flag
                self.score_pv = false;
                // give PV move the highest score so we search it first
                return 16000;
            }
        }

        let mut score: u32 = 0;
        let promoted = promoted(move_);
        let target = target(move_);
        // score capture move
        if capture(move_) != 0 {
            // prioritize captures
            score += 8000;
            if enpassant(move_) != 0 {
                // we know the target and source piece are pawns so no need to continue
                return score;
            } if promoted != 0 {
                // promotions always first
                return score+2000+PIECE_VALUE[(promoted%6) as usize] as u32;
            }

            // let captured = position.get_piece(target);
            // let attackers = position.get_attackers(target as usize, position.side);
            // let defenders = position.get_attackers(target as usize, position.side^1);
            // score move by MVV LVA lookup [source piece][target piece]
            return score + MVV_LVA[(get_piece(move_)%6) as usize][position.get_square_piece(target as usize)%6] as u32;
            // return score + get_see(get_piece(move_) % 6, captured as u8, attackers, defenders) as u32;
        }
        // score quiet move
        else {
            if self.killers[0][self.ply as usize] == move_ {
                // score 1st killer move
                score += 4000;
            } else if self.killers[1][self.ply as usize] == move_ {
                // score 2nd killer move
                score += 2500;
            } else {
                // score history move
                score += self.history[get_piece(move_) as usize][target as usize] as u32;
            }

            // reward for castling
            if castling(move_) != 0 {
                score += 400;
            }
        }
        
        if promoted != 0 {
            // promotions always first
            score += 9500 + PIECE_VALUE[(promoted%6) as usize] as u32;
        }

        return score;
    }
}