use crate::board::position::*;

use rand::{Rng,SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::Bitboard;

// random piece keys [piece][square]
pub static mut ZOBRIST_KEYS: [[u64;64];12] = [[0;64];12];
// random enpassant keys
pub static mut ZOBRIST_EP_KEYS: [u64;64] = [0;64];
// random castling keys [color][castling]
pub static mut ZOBRIST_CASTLING_KEYS: [u64;16] = [0;16];
// random side to move key
pub static mut ZOBRIST_TURN: u64 = 0;

// init random hash keys
pub fn init_zobrist() {
    let mut rng = ChaCha8Rng::seed_from_u64(69);
    unsafe {
        // loop over piece codes
        for piece in Piece::WhitePawn as usize..Piece::BlackKing as usize + 1 {
            // loop over squares
            for square in 0..64 {
                // generate random hash key
                ZOBRIST_KEYS[piece][square] = rng.gen::<u64>();
            }
        }
        // loop over board squares
        for square in 0..8 {
            // generate random hash key
            ZOBRIST_EP_KEYS[square] = rng.gen::<u64>();
        }
        // loop over castling keys
        for index in 0..16 {
            // generate random hash key
            ZOBRIST_CASTLING_KEYS[index] = rng.gen::<u64>();
        }
        // init random side key
        ZOBRIST_TURN = rng.gen::<u64>();
    }
}

impl Position {
    // generate unique pos ID
    pub fn generate_hash_key(&self) -> u64 {
        // final hash key
        let mut final_key: u64 = 0;
        // temp piece bitboard copy
        let mut bitboard: Bitboard;
        // loop over piece bitboards 
        for piece in Piece::WhitePawn as usize..Piece::BlackKing as usize + 1 {
            // loop over squares
            bitboard = self.bitboards[piece];
            // loop over the pieces within a bitboard
            while bitboard.0 != 0 {
                // get the square
                let square = bitboard.ls1b();
                // add the piece to the hash key
                final_key ^= unsafe { ZOBRIST_KEYS[piece][square as usize] };
                // pop LS1B
                bitboard.pop(square as usize);
            }
            
        }
        // add enpassant square to hash key
        if self.enpassant != Square::NoSquare {
            final_key ^= unsafe { ZOBRIST_EP_KEYS[self.enpassant as usize] };
        }
        // add castling rights to hash key
        final_key ^= unsafe { ZOBRIST_CASTLING_KEYS[self.castle as usize] };
        // add side to move to hash key
        if self.side == 1 {
            // black to move
            final_key ^= unsafe { ZOBRIST_TURN };
        }
        // return final key
        return final_key;
    }
}