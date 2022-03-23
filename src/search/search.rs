use crate::board::position::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;

use rand::Rng;

pub struct Searcher {
    ply: u8,
    nodes: u64,

    // communication
    stop: bool,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            ply: 0,
            nodes: 0,

            stop: false,
        }
    }

    pub fn stop(&mut self) {
        self.stop = true;
    }

    pub fn negamax(&mut self, position: &mut Position, depth: u8) -> u32 {
        let mut move_list = MoveList::new();
        position.generate_pseudo_moves(&mut move_list);
        let mut legal_moves = Vec::new();
        for move_count in 0..move_list.count {
            let move_ = move_list.moves[move_count as usize];
            let legal = position.make(move_);
            if !legal {
                position.unmake(move_);
                continue;
            }
            position.unmake(move_);
            legal_moves.push(move_);
        }
        let num = rand::thread_rng().gen_range(0..legal_moves.len());
        return legal_moves[num as usize];
    }
}