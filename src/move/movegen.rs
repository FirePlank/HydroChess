use crate::board::bitboard::*;
use crate::board::attacks::*;
use crate::board::position::*;

pub struct Move {
    m: u32,
    score: i16,
}

impl Position {
    pub fn gen_moves(&self, side: usize)  {
        // define source & target squares
        let mut source_square;
        let mut target_square;

        // define current piece's bitboard copy & it's attacks
        let mut bitboard: Bitboard;
        let mut attacks: u32;

        // loop over all the bitboards
        for piece in 0..12 {
            // init piece bitboard copy
            bitboard = self.bitboards[piece as usize];
            // generate white pawns and white king castling moves
            if side == Side::WHITE {
                if piece == Piece::WhitePawn as usize {
                    // loop over white pawns within white pawn bitboard
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;
                        // pop LS1B in bitboard
                        // bitboard.pop(source_square);
                        
                        // get pawn's target square
                        target_square = source_square - 8;
                        
                        // generate quiet pawn moves
                        if !(target_square < Square::A8 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square) == 0) {
                            // pawn promotion
                            if source_square >= Square::A7 as usize && source_square <= Square::H7 as usize {
                                // add move into a move list
                                println!("pawn promotion: {}{}q", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}r", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}b", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}n", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                            } else {
                                // one square ahead pawn move
                                println!("pawn move: {}{}", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);

                                // two squares ahead pawn move
                                if (source_square >= Square::A2 as usize && source_square <= Square::H2 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square - 8) == 0) {
                                    target_square = source_square - 16;
                                    println!("pawn move: {}{}", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                }
                            }
                        }
                        // pop ls1b from piece bitboard copy
                        bitboard.pop(source_square);
                    }
                }
            // generate black moves
            } else {
                if piece == Piece::BlackPawn as usize {
                    // loop over black pawns within black pawn bitboard
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;
                        // pop LS1B in bitboard
                        // bitboard.pop(source_square);
                        
                        // get pawn's target square
                        target_square = source_square + 8;
                        
                        // generate quiet pawn moves
                        if !(target_square > Square::H1 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square) == 0) {
                            // pawn promotion
                            if source_square >= Square::A2 as usize && source_square <= Square::H2 as usize {
                                // add move into a move list
                                println!("pawn promotion: {}{}q", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}r", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}b", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                                println!("pawn promotion: {}{}n", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);
                            } else {
                                // one square ahead pawn move
                                println!("pawn move: {}{}", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square]);

                                // two squares ahead pawn move
                                if (source_square >= Square::A7 as usize && source_square <= Square::H7 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square + 8) == 0) {
                                    println!("double pawn move: {}{}", SQUARE_COORDS[source_square], SQUARE_COORDS[target_square+8]);
                                }
                            }
                        }
                        // pop ls1b from piece bitboard copy
                        bitboard.pop(source_square);
                    }
                }
            }
        }
    }
}