use crate::board::position::*;
use crate::board::zobrist::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;
use crate::evaluation::*;
use crate::cache::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

pub const MAX_PLY: usize = 127;
pub const MATE_VALUE: i16 = i16::MAX-150;
pub const MATE_SCORE: i16 = i16::MAX-300;
pub const INFINITY: i16 = i16::MAX;
pub const NO_ENTRY: i16 = INFINITY-500;

pub static mut OPTIONS: SearchOptions = SearchOptions::default();
// stop search if time is up
pub static mut STOP: bool = false;

pub struct SearchOptions {
    pub threads_allowed: bool,
    pub hash_size: u16
}
impl SearchOptions {
    pub const fn default() -> SearchOptions {
        SearchOptions {
            threads_allowed: true,
            hash_size: 32,
        }
    }
}

pub static mut TT: TranspositionTable = TranspositionTable {
    table: vec![],
    size: 0,
    age: 0,
};

#[derive(Clone)]
pub struct Searcher {
    pub ply: u8,
    pub nodes: u64,
    pub time: u128,

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
    pub const fn new() -> Searcher {
        Searcher {
            ply: 0,
            nodes: 0,
            time: 0,
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

    // pub fn is_repetition(&self, position: &mut Position) -> bool {
    //     unsafe {
    //         // loop over repetition indices range
    //         for index in 0..REP_INDEX {
    //             // if repetition is found
    //             if REPETITION[index] == position.hash {
    //                 return true;
    //             }
    //         }
    //         // if no repetition found
    //         return false;
    //     }
    // }

    pub fn communicate(&mut self) {
        if self.timeset && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() > self.stoptime {
            unsafe { STOP = true; }
        }
    }

    pub fn search_position(&'static mut self, position: &'static mut Position, depth: u8) {
        // reset search variables
        unsafe { STOP = false; }

        for current_depth in 1..depth+1 {
            // return 0 if time is up
            if unsafe { STOP } { break; }
            // enable follow PV flag
            self.follow_pv = true;

            // SMP search
            let mut score = -INFINITY;
            // multi-threaded search if allowed
                if unsafe { OPTIONS.threads_allowed } && current_depth > 6 {
                let mut move_list = MoveList::new();
                position.generate_pseudo_moves(&mut move_list);
                let mut threads = Vec::with_capacity(move_list.count as usize);
                let mut handles = Vec::with_capacity(move_list.count as usize);
                for _ in 0..(move_list.count as f32/1.5) as usize {
                    let mut pos = position.clone();
                    let mut searcher = self.clone();
                    let handle = thread::spawn(move || {
                        let scorer: i16 = searcher.negamax(&mut pos, -INFINITY, INFINITY, current_depth-1, false);
                        return (scorer, searcher.pv_length[0], searcher.pv_table[0], searcher.nodes);
                    });
                    handles.push(handle);
                }
                while !handles.is_empty() {
                    threads.push(handles.pop().expect("error while popping").join().unwrap());
                }
                for item in threads.iter() {
                    if item.0 > score {
                        self.nodes = item.3;
                        score = item.0;
                        self.pv_length[0] = item.1;
                        self.pv_table[0] = item.2;
                    }
                }
            } else {
                // find best move within a given position
                score = self.negamax(position, -INFINITY, INFINITY, current_depth, true);
            }

            // if score <= alpha || score >= beta {
            //     alpha = -INFINITY;
            //     beta = INFINITY;
            //     continue;
            // }

            // set up the window for the next iteration
            // alpha = score - 50;
            // beta = score + 50;
            let mate;

            if score > -MATE_VALUE && score < -MATE_SCORE {
                print!("info score mate {} depth {} nodes {} time {} pv ", -(self.pv_length[0] as i16)/2-1, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()-self.time);
                mate = true;
            } else if score > MATE_SCORE && score < MATE_VALUE {
                print!("info score mate {} depth {} nodes {} time {} pv ", self.pv_length[0]/2+1, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()-self.time);
                mate = true;
            } else {
                print!("info score cp {} depth {} nodes {} time {} pv ", score, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()-self.time);
                mate = false;
            }
            // loop over the moves within a PV lone
            for count in 0..self.pv_length[0] {
                // print PV move
                Move(self.pv_table[0][count as usize]).show();
                print!(" ");
            }
            println!();
            
            // if forced mate exists there is no need to search further
            if mate {
                break;
            }
        }

        // bestmove
        print!("bestmove ");
        Move(self.pv_table[0][0]).show();
        println!();

        // age TT
        unsafe {
            TT.age();
        }
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

        // too deep, return eval
        if self.ply >= MAX_PLY as u8 {
            return eval;
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
            if unsafe { STOP } { return NO_ENTRY; }
            // fail-hard beta cutoff
            if score > alpha {
                // PV node (move)
                alpha = score;
                if score >= beta {
                    // node (move) fails high
                    return beta;
                }
            }
        }
        // node (move) fails low
        return alpha;
    }

    pub fn negamax(&mut self, position: &mut Position, mut alpha: i16, mut beta: i16, mut depth: u8, null_move: bool) -> i16 {
        let pv_node = beta.wrapping_sub(alpha) > 1;
        let mut best_move = BestMove { value: 0 };
        let mut hash_flag = LOWER_BOUND;

        let mut score: i16;
        let is_root = self.ply == 0;

        // increment nodes counter
        self.nodes += 1;

        // too deep, return eval
        if self.ply >= MAX_PLY as u8 {
            return evaluate(position);
        }

        // fifty-move rule
        if position.is_fifty() {
            return 0;
        }

        // init PV lenght
        self.pv_length[self.ply as usize] = self.ply;
        
        if !is_root {
            // repetition
            if position.is_threefold() {
                return 0;
            }

            // mate distance pruning
            if alpha < -MATE_VALUE {
                alpha = -MATE_VALUE;
            } if beta > MATE_VALUE-1 {
                beta = MATE_VALUE-1;
            } if alpha >= beta {
                return alpha;
            }
        }

        // self.pv_table[self.ply as usize][self.ply as usize] = 0;

        // recursion escape condition
        if depth == 0 {
            return self.quiescence(position, alpha, beta);
        }

        // is king in check
        let in_check = position.is_attacked(if position.side == 0 { position.bitboards[Piece::WhiteKing as usize].ls1b() as usize } else {
            position.bitboards[Piece::BlackKing as usize].ls1b() as usize
        }, position.side^1);

        // increase search depth if the king has been exposed to a check
        if in_check {
            depth += 1;
        }

        // read hash entry
        if !is_root && !pv_node {
            score = unsafe { TT.probe(alpha, beta, &mut best_move, depth, self.ply, position.hash) };
            if score != NO_ENTRY {
                // return score
                return score;
            }
        }
        
        // every 2047 nodes
        if self.nodes & 2047 == 0 {
            self.communicate();
        }

        // static evaluation
        let eval = evaluate(position);

        if !in_check && !pv_node {
            // evaluation pruning
            if depth < 3 && (beta-1).abs() as i32 > -49000 + 100 {
                let eval_margin = PIECE_VALUE[Piece::WhitePawn as usize] * depth as i16;
                if eval - eval_margin >= beta {
                    return eval - eval_margin;
                }
            }

            // // razoring
            // if depth < 2 && eval + 339 < alpha {
            //     return self.quiescence(position, alpha, beta);
            // }

            // // futility pruning
            // let reversed_fp_margin = 64*depth as i16;
            // if depth < 9 && eval - reversed_fp_margin >= beta {
            //     return eval - reversed_fp_margin;
            // }

            // null move pruning
            if null_move {
                if self.ply != 0 && depth > 2 && eval >= beta {
                    // increment ply
                    self.ply += 1;
                    // make null move
                    position.make_null_move();
                    // search moves with reduced depth to find beta cutoffs
                    let score = -self.negamax(position, -beta, -beta+1, depth-1-self.reduction_limit, false);
                    // take back null move
                    position.unmake_null_move();
                    // decrement ply
                    self.ply -= 1;

                    // return 0 if time is up
                    if unsafe { STOP } { return NO_ENTRY; }
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
        // nice name btw lol
        let fp_margin = eval + 97 * depth as i16;

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

        let mut best_score = -INFINITY;
        let mut skip_quiet = false;

        // loop over moves within move list
        for count in 0..counted {
            // get move                               
            let move_ = sorted[count as usize].1;

            let is_quiet = capture(move_) == 0;
            if is_quiet && skip_quiet {
                continue;
            }

            let is_killer = self.killers[0][self.ply as usize] == move_ || self.killers[1][self.ply as usize] == move_;

            if !is_root && best_score > -INFINITY {
                if depth < 8 && is_quiet && !is_killer && fp_margin <= alpha && alpha.abs() < INFINITY - 100 {
                    skip_quiet = true;
                    continue;
                }
            }

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
            // decrement repetition index
            // unsafe { REP_INDEX = REP_INDEX.wrapping_sub(1); }
            // return 0 if time is up
            if unsafe { STOP } { return 0; }
            // increment the number of moves searched so far
            moves_searched += 1;


            if score > best_score {
                best_score = score;
            }

            // fail-hard beta cutoff
            if score > alpha {
                hash_flag = EXACT;
                best_move.value = move_;
                best_score = score;
                // PV node (move)
                alpha = score;

                // on quiet move
                if is_quiet {  
                    // store history moves
                    self.history[get_piece(move_) as usize][target(move_) as usize] += depth as u32;
                }

                // write PV move
                self.pv_table[self.ply as usize][self.ply as usize] = move_;
                // loop over the next ply
                for next_ply in self.ply+1..self.pv_length[self.ply as usize+1] {
                    // copy move from deeper ply into a current ply's line
                    self.pv_table[self.ply as usize][next_ply as usize] = self.pv_table[self.ply as usize+1][next_ply as usize];
                }
                // adjust PV lenght
                self.pv_length[self.ply as usize] = self.pv_length[self.ply as usize+1];

                if score >= beta {
                    // store hash entry with the score equal to beta
                    unsafe { TT.write(position.hash, beta, best_move.value, depth, self.ply, UPPER_BOUND); }

                    // on quiet moves
                    if is_quiet {
                        // store killer moves
                        self.killers[1][self.ply as usize] = self.killers[0][self.ply as usize];
                        self.killers[0][self.ply as usize] = move_;
                    }
                    // node (move) fails high
                    return beta;
                }
            }
        }
        // check if checkmate or stalemate
        if legal_moves == 0 {
            if in_check {
                // checkmate
                return -MATE_VALUE+self.ply as i16;
            } else {
                // stalemate
                return 0;
            }
        }
        // store hash entry with the score equal to alpha
        unsafe { TT.write(position.hash, alpha, best_move.value, depth, self.ply, hash_flag); }
        // node (move) fails low
        return alpha;
    }
}