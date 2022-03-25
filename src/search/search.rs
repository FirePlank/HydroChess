use crate::board::position::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;
use crate::evaluation::*;

use std::time::Instant;

pub const MAX_PLY: usize = 127;

pub struct Searcher {
    pub ply: u8,
    pub nodes: u64,
    pub time: Instant,

    pub killers: [[u32;MAX_PLY];2],
    pub history: [[u32;64];12],

    pub pv_table: [[u32;MAX_PLY];MAX_PLY], // PV table [ply][ply]
    pub pv_length: [u8;MAX_PLY],           // PV lenght [ply]
    pub follow_pv: bool,
    pub score_pv: bool,

    // communication
    pub stop: bool,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            ply: 0,
            nodes: 0,
            time: Instant::now(),
            killers: [[0;MAX_PLY];2],
            history: [[0;64];12],
            pv_table: [[0;MAX_PLY];MAX_PLY],
            pv_length: [0;MAX_PLY],
            follow_pv: false,
            score_pv: false,
            stop: false,
        }
    }

    pub fn stop(&mut self) {
        self.stop = true;
    }

    pub fn search_position(&mut self, position: &mut Position, depth: u8) {
        // reset search variables
        self.nodes = 0;
        self.follow_pv = false;
        self.score_pv = false;
        // rust compiler automatically changes these to memset
        self.killers.iter_mut().for_each(|x| *x = [0;MAX_PLY]);
        self.history.iter_mut().for_each(|x| *x = [0;64]);
        self.pv_table.iter_mut().for_each(|x| *x = [0;MAX_PLY]);
        self.pv_length.iter_mut().for_each(|x| *x = 0);

        // iterative deepening
        for current_depth in 1..depth+1 {
            // enable follow PV flag
            self.follow_pv = true;
            // find best move within a given position
            let score = self.negamax(position, i16::MIN+1, i16::MAX-1, current_depth);

            print!("info score cp {} depth {} nodes {} time {} pv ", score, current_depth, self.nodes, self.time.elapsed().as_millis());
            // loop over the moves within a PV lone
            for count in 0..self.pv_length[0] {
                // print PV move
                Move(self.pv_table[0][count as usize]).show();
                print!(" ");
            }
            println!();
        }

        // bestmove
        print!("bestmove ");
        Move(self.pv_table[0][0]).show();
        println!();
    }

    pub fn enable_pv_scoring(&mut self, move_list: &MoveList) {
        // disable following PV
        self.follow_pv = false;
        
        // loop over the moves within a move list
        for count in 0..move_list.count {
            // make sure we hit PV move
            if self.pv_table[0][self.ply as usize] == move_list.moves[count as usize] {
                // enable move scoring
                self.score_pv = true;
                // enable following PV
                self.follow_pv = true;
                // break loop
                break;
            }
        }
    }

    pub fn quiescence(&mut self, position: &mut Position, mut alpha: i16, beta: i16) -> i16 {
        // increment nodes
        self.nodes += 1;
        // evaluate position
        let eval = evaluate(position);
        
        // fail-hard beta cutoff
        if eval >= beta {
            // node (move) fails high
            return beta;
        } else if eval > alpha {
            // PV node (move)
            alpha = eval;
        }

        // create move list
        let mut move_list = MoveList::new();
        // generate captures
        position.generate_pseudo_captures(&mut move_list);
        let counted = move_list.count;
        // sort moves
        let sorted = self.sort_moves(&position, move_list);

        // loop over captures within move list
        for count in 0..counted {
            // get capture move                              
            let move_ = sorted[count as usize].1;

            if !position.make(move_) {
                position.unmake(move_);
                continue;
            }

            // increment ply
            self.ply += 1;
            // score current move
            let score = -self.quiescence(position, -beta, -alpha);
            // take back move
            position.unmake(move_);
            // decrement ply
            self.ply -= 1;

            // fail-hard beta cutoff
            if score >= beta {
                // node (move) fails high
                return beta;
            } else if score > alpha {
                // PV node (move)
                alpha = score;
            }
        }
        // node (move) fails low
        return alpha;
    }

    pub fn negamax(&mut self, position: &mut Position, mut alpha: i16, beta: i16, mut depth: u8) -> i16 {
        // init PV lenght
        self.pv_length[self.ply as usize] = self.ply;
        
        // recursion escape condition
        if depth == 0 {
            return self.quiescence(position, alpha, beta);
        }

        // too deep, return eval
        if self.ply == MAX_PLY as u8 {
            return evaluate(position);
        }

        // increment nodes counter
        self.nodes += 1;

        // is king in check
        let in_check = position.is_attacked(if position.side == 0 { position.bitboards[Piece::WhiteKing as usize].ls1b() as usize } else {
            position.bitboards[Piece::BlackKing as usize].ls1b() as usize
        }, position.side^1);

        // increase search depth if the king has been exposed to a check
        if in_check {
            depth += 1;
        }
        // legal moves
        let mut legal_moves = 0;
        // create move list
        let mut move_list = MoveList::new();
        // generate moves
        position.generate_pseudo_moves(&mut move_list);
        let counted = move_list.count;

        // if we are now following PV line
        if self.follow_pv {
            // enable PV move scoring
            self.enable_pv_scoring(&move_list);
        }
        // sort moves
        let sorted = self.sort_moves(&position, move_list);

        // loop over moves within move list
        for count in 0..counted {
            // get move                               
            let move_ = sorted[count as usize].1;

            if !position.make(move_) {
                position.unmake(move_);
                continue;
            }

            // increment ply and legal moves
            self.ply += 1;
            legal_moves += 1;
            // score current move
            let score = -self.negamax(position, -beta, -alpha, depth - 1);
            // take back move
            position.unmake(move_);
            // decrement ply
            self.ply -= 1;

            // fail-hard beta cutoff
            if score >= beta {
                // on quiet moves
                if capture(move_) == 0 {
                    // store killer moves
                    self.killers[1][self.ply as usize] = self.killers[0][self.ply as usize];
                    self.killers[0][self.ply as usize] = move_;
                }
                // node (move) fails high
                return beta;
            } else if score > alpha {
                // on quiet move
                if capture(move_) == 0 {  
                    // store history moves
                    self.history[get_piece(move_) as usize][target(move_) as usize] += depth as u32;
                }
                // PV node (move)
                alpha = score;

                // write PV move
                self.pv_table[self.ply as usize][self.ply as usize] = move_;

                // loop over the next ply
                for next_ply in self.ply+1..self.pv_length[self.ply as usize+1] {
                    // copy move from deeper ply into a current ply's line
                    self.pv_table[self.ply as usize][next_ply as usize] = self.pv_table[self.ply as usize+1][next_ply as usize];
                }

                // adjust PV lenght
                self.pv_length[self.ply as usize] = self.pv_length[self.ply as usize+1];
            }
        }
        // check if checkmate or stalemate
        if legal_moves == 0 {
            if in_check {
                // checkmate
                return i16::MIN+500+self.ply as i16;
            } else {
                // stalemate
                return 0;
            }
        }
        // node (move) fails low
        return alpha;
    }
}