use crate::search::*;
use std::mem;

pub const EXACT: u8 = 0;
pub const LOWER_BOUND: u8 = 1;
pub const UPPER_BOUND: u8 = 2;

pub struct BestMove {
    pub value: u32,
}

#[derive(Clone, Default, Debug)]
pub struct TTData {
    pub hash: u64,       // unique chess position identifier
    pub score: i16,      // score (alpha/beta/PV)
    pub best_move: u32,  // the best move to play
    pub depth: u8,       // current search depth
    pub age: u16,        // the age of the entry so we can determine when to delete it
    pub flag: u8,        // the type of node (fail-high, fail-low, exact, invalid)
}

pub struct TranspositionTable {
    pub table: Vec<TTData>,
    pub size: usize,
    pub age: u16,
}

impl TranspositionTable {
    pub fn new(size: usize, quiet: bool) -> TranspositionTable {
        let bucket_size = size * 1048576 / mem::size_of::<TTData>();
        let mut tt = TranspositionTable {
            table: Vec::with_capacity(bucket_size),
            size: bucket_size,
            age: 0,
        };
        if !quiet {
            println!("info string allocating {} KB and {} items for TT", bucket_size*mem::size_of::<TTData>()/1024, bucket_size);
        }

        if size != 0 {
            tt.table.resize(tt.size, Default::default());
        }

        return tt;
    }

    pub fn reset(&mut self) {
        self.table.clear();
        self.table.resize(self.size, Default::default());
    }

    pub fn probe(&self, alpha: i16, beta: i16, best_move: &mut BestMove, depth: u8, ply: u8, hash: u64) -> i16 {
        let entry = &self.table[hash as usize % self.size];
        if entry.hash == hash {
            if entry.depth >= depth {
                // init score
                let mut score = entry.score;

                // adjust mating scores
                if score < -MATE_SCORE {
                    score += ply as i16;
                } else if score > MATE_SCORE {
                    score -= ply as i16;
                }
                // match hash flag
                if entry.flag == EXACT {
                    return score;
                } else if entry.flag == LOWER_BOUND && score <= alpha {
                    return alpha;
                } else if entry.flag == UPPER_BOUND && score >= beta {
                    return beta;
                }
            }

            // store best move
            best_move.value = entry.best_move;
        }
        // if hash entry doesn't exist
        return NO_ENTRY;
    }

    pub fn write(&mut self, hash: u64, mut score: i16, best_move: u32, depth: u8, ply: u8, flag: u8) {
        let entry = &self.table[hash as usize % self.size];
        let replace;
        if entry.hash == 0 {
            replace = true;
        } else if entry.hash == hash {
            replace = (entry.depth >= 3 && depth >= entry.depth - 3) || entry.flag == EXACT;
        } else {
            replace = entry.age != self.age || depth >= entry.depth;
        }
        
        if replace {
            if score < -MATE_SCORE {
                score -= ply as i16;
            } else if score > MATE_SCORE {
                score += ply as i16;
            }
            self.table[hash as usize % self.size] = TTData {
                hash,
                score,
                best_move,
                depth,
                flag,
                age: self.age,
            };
        }
    }

    pub fn age(&mut self) {
        self.age = self.age.wrapping_add(1);
    }

}