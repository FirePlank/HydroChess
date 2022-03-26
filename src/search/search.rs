use crate::board::position::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;
use crate::evaluation::*;

use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_PLY: usize = 127;
pub const MATE_VALUE: i16 = i16::MAX-150;
pub const MATE_SCORE: i16 = i16::MAX-300;
pub const INFINITY: i16 = i16::MAX;
// stop search if time is up
pub static mut STOP: bool = false;

pub struct Searcher {
    pub ply: u8,
    pub nodes: u64,
    pub time: SystemTime,

    pub killers: [[u32;MAX_PLY];2],
    pub history: [[u32;64];12],

    pub pv_table: [[u32;MAX_PLY];MAX_PLY], // PV table [ply][ply]
    pub pv_length: [u8;MAX_PLY],           // PV lenght [ply]
    pub follow_pv: bool,
    pub score_pv: bool,

    pub full_depth_moves: u8,
    pub reduction_limit: u8,

    // uci options
    pub inc: i32,
    pub movetime: i32,
    pub movestogo: i32,
    pub playtime: i32,
    pub timeset: bool,
    pub stoptime: u128,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            ply: 0,
            nodes: 0,
            time: SystemTime::now(),
            killers: [[0;MAX_PLY];2],
            history: [[0;64];12],
            pv_table: [[0;MAX_PLY];MAX_PLY],
            pv_length: [0;MAX_PLY],
            follow_pv: false,
            score_pv: false,
            full_depth_moves: 3,
            reduction_limit: 2,
            inc: 0,
            movetime: -1,
            movestogo: 30,
            playtime: -1,
            timeset: false,
            stoptime: 0,
        }
    }

    pub fn communicate(&mut self) {
        if self.timeset && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() > self.stoptime {
            unsafe { STOP = true; }
        }
    }

    pub fn search_position(&mut self, position: &mut Position, depth: u8) {
        // reset search variables
        unsafe { STOP = false; }
        // self.nodes = 0;
        // self.follow_pv = false;
        // self.score_pv = false;
        // // rust compiler automatically changes these to memset
        // self.killers.iter_mut().for_each(|x| *x = [0;MAX_PLY]);
        // self.history.iter_mut().for_each(|x| *x = [0;64]);
        // self.pv_table.iter_mut().for_each(|x| *x = [0;MAX_PLY]);
        // self.pv_length.iter_mut().for_each(|x| *x = 0);
        // define initial apha beta bounds
        // let mut alpha = -INFINITY;
        // let mut beta = INFINITY;
        // iterative deepening
        for current_depth in 1..depth+1 {
            // return 0 if time is up
            if unsafe { STOP } { break; }
            // enable follow PV flag
            self.follow_pv = true;
            // find best move within a given position
            let score = self.negamax(position, -INFINITY, INFINITY, current_depth, true);

            // if score <= alpha || score >= beta {
            //     alpha = -INFINITY;
            //     beta = INFINITY;
            //     continue;
            // }

            // set up the window for the next iteration
            // alpha = score - 50;
            // beta = score + 50;
            
            print!("info score cp {} depth {} nodes {} time {} pv ", score, current_depth, self.nodes, SystemTime::now().duration_since(self.time).expect("Time went backwards").as_millis());
            // loop over the moves within a PV lone
            for count in 0..self.pv_length[0] {
                // print PV move
                Move(self.pv_table[0][count as usize]).show();
                print!(" ");
            }
            println!();
            // check if we have forced mate so we can stop search
            if self.pv_length[0] < current_depth-1 {
                break;
            }
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
        // every 2047 nodes, check if time is up
        if self.nodes & 2047 == 0 {
            self.communicate();
        }
        
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
            // check if time is up
            if unsafe { STOP } { return 0; }
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

    pub fn negamax(&mut self, position: &mut Position, mut alpha: i16, mut beta: i16, mut depth: u8, mut null_move: bool) -> i16 {
        // every 2047 nodes
        if self.nodes & 2047 == 0 {
            self.communicate();
        }
        // init PV lenght
        self.pv_length[self.ply as usize] = self.ply;

        let pv_node = beta.wrapping_sub(alpha) > 1;
        let mut score: i16;
        
        // recursion escape condition
        if depth == 0 {
            return self.quiescence(position, alpha, beta);
        }

        // too deep, return eval
        if self.ply == MAX_PLY as u8 {
            return evaluate(position);
        }

        // mate distance pruning
        if alpha < -MATE_VALUE {
            alpha = -MATE_VALUE;
        } if beta > MATE_VALUE-1 {
            beta = MATE_VALUE-1;
        } if alpha >= beta {
            return alpha;
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
        else if !in_check && !pv_node {
            // static evaluation
            let eval = evaluate(position);

            // evaluation pruning
            if depth < 3 && (beta-1).abs() as i32 > -49000 + 100 {
                let eval_margin = PIECE_VALUE[Piece::WhitePawn as usize] * depth as i16;
                if eval - eval_margin >= beta {
                    return eval - eval_margin;
                }
            }

            // null move pruning
            if null_move {
                if self.ply != 0 && depth > 2 && eval >= beta {
                    // make a null move
                    position.side ^= 1;
                    // preserve enpassant
                    let enpassant = position.enpassant;
                    position.enpassant = Square::NoSquare;
                    
                    // search moves with reduced depth to find beta cutoffs
                    let score = -self.negamax(position, -beta, -beta+1, depth-1-self.reduction_limit, false);
                    // take back null move
                    position.side ^= 1;
                    position.enpassant = enpassant;
                    // return 0 if time is up
                    if unsafe { STOP } { return 0; }
                    // fail-hard beta cutoff
                    if score >= beta {
                        // node (move) fails high
                        return beta;
                    }
                }

                // razoring
                score = eval + PIECE_VALUE[Piece::WhitePawn as usize];
                let new_score;

                if score < beta {
                    if depth == 1 {
                        new_score = self.quiescence(position, alpha, beta);
                        if new_score < beta {
                            return if new_score > score { new_score } else { score };
                        }
                    }
                }
            }
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

        // number of moves searched in a move list
        let mut moves_searched = 0;

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

            // full depth search
            if moves_searched == 0 {
                // do normal alpha-beta search
                score = -self.negamax(position, -beta, -alpha, depth - 1, true);
            } else {
                // condition to consider LMR (late move reduction)
                if moves_searched >= self.full_depth_moves && depth >= self.reduction_limit && 
                                    !in_check && capture(move_) == 0 && promoted(move_) == 0 {
                    // search current move with reduced depth
                    score = -self.negamax(position, -alpha-1, -alpha, depth-2, true);
                } else {
                    score = alpha+1;
                }
                // if found a better move during LMR re-search at normal depth
                if score > alpha {
                    score = -self.negamax(position, -alpha-1, -alpha, depth-1, true);
                    // if LMR fails re-search at full depth
                    if score > alpha && score < beta {
                        score = -self.negamax(position, -beta, -alpha, depth-1, true);
                    }
                }
            }

            // take back move
            position.unmake(move_);
            // decrement ply
            self.ply -= 1;
            // return 0 if time is up
            if unsafe { STOP } { return 0; }
            // increment the number of moves searched so far
            moves_searched += 1;

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
                return -MATE_SCORE+self.ply as i16;
            } else {
                // stalemate
                return 0;
            }
        }
        // node (move) fails low
        return alpha;
    }
}