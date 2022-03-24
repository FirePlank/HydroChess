use crate::board::position::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;
use crate::evaluation::*;

use std::time::Instant;

pub struct Searcher {
    ply: u8,
    nodes: u64,
    time: Instant,
    best_move: u32,
    // communication
    stop: bool,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            ply: 0,
            nodes: 0,
            time: Instant::now(),
            best_move: 0,
            stop: false,
        }
    }

    pub fn stop(&mut self) {
        self.stop = true;
    }

    pub fn search_position(&mut self, position: &mut Position, depth: u8) {
        // find best move within a given position
        let score = self.negamax(position, i16::MIN+1, i16::MAX-1, depth);

        // bestmove
        print!("bestmove ");
        Move(self.best_move).show();
        println!("\ninfo score cp {} depth {} nodes {} time {}", score, depth, self.nodes, self.time.elapsed().as_millis());
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
        // loop over captures within move list
        for count in 0..move_list.count {
            // get capture move                              
            let move_ = move_list.moves[count as usize];

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

    pub fn negamax(&mut self, position: &mut Position, mut alpha: i16, beta: i16, depth: u8) -> i16 {
        // recursion escape condition
        if depth == 0 {
            return self.quiescence(position, alpha, beta);
        }
        // increment nodes counter
        self.nodes += 1;

        // is king in check
        let in_check = position.is_attacked(if position.side == 0 { position.bitboards[Piece::WhiteKing as usize].ls1b() as usize } else {
            position.bitboards[Piece::BlackKing as usize].ls1b() as usize
        }, position.side^1);

        // legal moves
        let mut legal_moves = 0;
        // create move list
        let mut move_list = MoveList::new();
        // generate moves
        position.generate_pseudo_moves(&mut move_list);
        // loop over moves within move list
        for count in 0..move_list.count {
            // get move                               
            let move_ = move_list.moves[count as usize];

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
                // node (move) fails high
                return beta;
            } else if score > alpha {
                // PV node (move)
                alpha = score;
                // if root move
                if self.ply == 0 {
                    // set best move
                    self.best_move = move_;
                }
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