use crate::board::bitboard::*;
use crate::board::attacks::*;
use crate::board::position::*;
use crate::r#move::encode::*;
pub struct Move(pub u32);


pub struct MoveList {
    pub moves: [u32; 256],
    pub count: i32,
}

/*
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
*/

// promoted pieces in string format, easily indexable with Piece enum as usize
pub const PROMOTED_PIECES: [& str; 11] = [" ", "n", "b", "r", "q", " ", " ", "n", "b", "r", "q"];

impl Move {
    pub fn show_move(&self) {
        let source = source(self.0);
        let target = target(self.0);
        let promoted = promoted(self.0);
        println!("{}{}{}", SQUARE_COORDS[source as usize], SQUARE_COORDS[target as usize], PROMOTED_PIECES[promoted as usize]);
    }
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [0; 256],
            count: 0,
        }
    }
    pub fn show(&self) {
        if self.count == 0 {
            println!("The move list is empty.");
            return;
        }
        println!("\n    move    piece    capture    double    enpassant    castling\n");
        // loop over moves within a move list
        for move_count in 0..self.count {
            // init move
            let move_ = self.moves[move_count as usize];
            let source = source(move_);
            let target = target(move_);
            let piece = piece(move_);
            let promoted = promoted(move_);
            let capture = capture(move_);
            let double = double(move_);
            let enpassant = enpassant(move_);
            let castling = castling(move_);

            // print moves
            println!("    {}{}{}   {}          {}         {}         {}            {}", SQUARE_COORDS[source as usize], SQUARE_COORDS[target as usize], PROMOTED_PIECES[promoted as usize], ASCII_PIECES[piece as usize], capture, double, enpassant, castling);
        }
        // print total number of moves
        println!("\n    Total number of moves: {}", self.count);
    }
    pub fn add(&mut self, move_: u32) {
        // store move
        self.moves[self.count as usize] = move_;
        // increment move count
        self.count += 1;
    }
}

impl Position {
    pub fn generate_moves(&self, move_list: &mut MoveList) {
        // define source & target squares
        let mut source_square;
        let mut target_square;

        // define current piece's bitboard copy & it's attacks
        let mut bitboard: Bitboard;
        let mut attacks;

        // loop over all the bitboards
        unsafe {
            for piece in 0..12 {
                // init piece bitboard copy
                bitboard = self.bitboards[piece as usize];
                // generate white pawns & white king castling moves
                if self.side == Side::WHITE {
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
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteQueen as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteKnight as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteBishop as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteRook as u8, 0, 0, 0, 0));
                                } else {
                                    // one square ahead pawn move
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));

                                    // two squares ahead pawn move
                                    if (source_square >= Square::A2 as usize && source_square <= Square::H2 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square - 8) == 0) {
                                        target_square = source_square - 16;
                                        move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 1, 0, 0));
                                    }
                                }
                            }

                            // init pawn attacks bitboard
                            attacks = Bitboard(PAWN_ATTACKS[self.side as usize][source_square] & self.occupancies[Side::BLACK as usize].0);
                            // generate pawn captures
                            while attacks.0 != 0 {
                                // get least significant 1st bit index
                                target_square = attacks.ls1b() as usize;

                                if source_square >= Square::A7 as usize && source_square <= Square::H7 as usize {
                                    // pawn promotion capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteQueen as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteKnight as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteBishop as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteRook as u8, 1, 0, 0, 0));
                                } else {
                                    // pawn capture normal
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                                }
                                // pop LS1B in bitboard
                                attacks.pop(target_square);
                            }

                            // generate enpassant captures
                            if self.enpassant != Square::NoSquare {
                                // lookup pawn attacks and bitwise AND with enpassant square (bit)
                                let enpassant_attacks = PAWN_ATTACKS[self.side as usize][source_square] & (1 << self.enpassant as usize);
                                // make sure enpassant capture is available
                                if enpassant_attacks != 0 {
                                    // get least significant 1st bit index
                                    target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                    // enpassant capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 1, 0));
                                }
                            }

                            // pop ls1b from piece bitboard copy
                            bitboard.pop(source_square);
                        }
                    }
                    // castling moves
                    if piece == Piece::WhiteKing as usize {
                        // king self.side castling is available
                        if (self.castle & Castling::WK as u8) != 0 {
                            // make sure squares between king and rook are empty
                            if (self.occupancies[Side::BOTH as usize].get(Square::F1 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::G1 as usize) == 0) {
                                // make sure king and the spaces in between are not attacked
                                if !self.is_attacked(Square::E1 as usize, Side::BLACK as usize) && !self.is_attacked(Square::F1 as usize, Side::BLACK as usize) {
                                    // add move into a move list
                                    move_list.add(encode_move(Square::E1 as u8, Square::G1 as u8, piece as u8, 0, 0, 0, 0, 1));
                                }
                            }
                        }

                        // queen self.side castling is available
                        if (self.castle & Castling::WQ as u8) != 0 {
                            // make sure squares between king and rook are empty
                            if (self.occupancies[Side::BOTH as usize].get(Square::D1 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::C1 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::B1 as usize) == 0) {
                                // make sure king and the spaces in between are not attacked
                                if !self.is_attacked(Square::E1 as usize, Side::BLACK as usize) && !self.is_attacked(Square::D1 as usize, Side::BLACK as usize) {
                                    // add move into a move list
                                    move_list.add(encode_move(Square::E1 as u8, Square::C1 as u8, piece as u8, 0, 0, 0, 0, 1));
                                }
                            }
                        }
                    }
                

                // generate black pawns & black king castling moves
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
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackQueen as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackKnight as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackBishop as u8, 0, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackRook as u8, 0, 0, 0, 0));
                                } else {
                                    // one square ahead pawn move
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));

                                    // two squares ahead pawn move
                                    if (source_square >= Square::A7 as usize && source_square <= Square::H7 as usize) && (self.occupancies[Side::BOTH as usize].get(target_square + 8) == 0) {
                                        move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 1, 0, 0));
                                    }
                                }
                            }

                            // init pawn attacks bitboard
                            attacks = Bitboard(PAWN_ATTACKS[self.side as usize][source_square] & self.occupancies[Side::WHITE as usize].0);
                            // generate pawn captures
                            while attacks.0 != 0 {
                                // get least significant 1st bit index
                                target_square = attacks.ls1b() as usize;

                                // pawn promotion
                                if source_square >= Square::A2 as usize && source_square <= Square::H2 as usize {
                                    // add move into a move list
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackQueen as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackKnight as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackBishop as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackRook as u8, 1, 0, 0, 0));
                                } else {
                                    // pawn capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                                }
                                // pop LS1B in bitboard
                                attacks.pop(target_square);
                            }

                            // generate enpassant captures
                            if self.enpassant != Square::NoSquare {
                                // lookup pawn attacks and bitwise AND with enpassant square (bit)
                                let enpassant_attacks = PAWN_ATTACKS[self.side as usize][source_square] & (1 << self.enpassant as usize);
                                // make sure enpassant capture is available
                                if enpassant_attacks != 0 {
                                    // get least significant 1st bit index
                                    target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                    // enpassant capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 1, 0));
                                }
                            }

                            // pop ls1b from piece bitboard copy
                            bitboard.pop(source_square);
                        }
                    }

                    // generate castling moves
                    if piece == Piece::BlackKing as usize {
                        // make sure king is not in check
                        if !self.is_attacked(Square::E8 as usize, Side::WHITE as usize) {
                            // king self.side castling is available
                            if (self.castle & Castling::BK as u8) != 0 {
                                // make sure squares between king and rook are empty
                                if (self.occupancies[Side::BOTH as usize].get(Square::F8 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::G8 as usize) == 0) {
                                    // make sure king and the spaces in between are not attacked
                                    if !self.is_attacked(Square::E8 as usize, Side::WHITE as usize) && !self.is_attacked(Square::F8 as usize, Side::WHITE as usize) {
                                        // add move into a move list
                                        move_list.add(encode_move(Square::E8 as u8, Square::G8 as u8, piece as u8, 0, 0, 0, 0, 1));
                                    }
                                }
                            }

                            // queen self.side castling is available
                            if (self.castle & Castling::BQ as u8) != 0 {
                                // make sure squares between king and rook are empty
                                if (self.occupancies[Side::BOTH as usize].get(Square::D8 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::C8 as usize) == 0) && (self.occupancies[Side::BOTH as usize].get(Square::B8 as usize) == 0) {
                                    // make sure king and the spaces in between are not attacked
                                    if !self.is_attacked(Square::E8 as usize, Side::WHITE as usize) && !self.is_attacked(Square::D8 as usize, Side::WHITE as usize) {
                                        // add move into a move list
                                        move_list.add(encode_move(Square::E8 as u8, Square::C8 as u8, piece as u8, 0, 0, 0, 0, 1));
                                    }
                                }
                            }
                        }
                    }
                }

                // generate knight moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteKnight as usize;
                } else { piece_to_check = Piece::BlackKnight as usize; }
                
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init knight attacks bitboard
                        attacks = Bitboard(KNIGHT_ATTACKS[source_square] & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate knight captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) == 0} else { self.occupancies[Side::WHITE as usize].get(target_square) == 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));
                            } else {
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));     
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate bishop moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteBishop as usize;
                } else { piece_to_check = Piece::BlackBishop as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init bishop attacks bitboard
                        attacks = Bitboard(get_bishop_attacks(source_square, self.occupancies[Side::BOTH as usize]) & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate bishop captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) == 0} else { self.occupancies[Side::WHITE as usize].get(target_square) == 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));
                            } else {
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));                        
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate rook moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteRook as usize;
                } else { piece_to_check = Piece::BlackRook as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init rook attacks bitboard
                        let occ;
                        if self.side == Side::WHITE { occ = self.occupancies[Side::WHITE as usize]; } else { occ = self.occupancies[Side::BLACK as usize]; }
                        attacks = Bitboard(get_rook_attacks(source_square, self.occupancies[Side::BOTH as usize]) & !occ.0);
                        // generate rook captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) == 0} else { self.occupancies[Side::WHITE as usize].get(target_square) == 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));
                            } else {
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));         
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate queen moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteQueen as usize;
                } else { piece_to_check = Piece::BlackQueen as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init queen attacks bitboard
                        attacks = Bitboard(get_queen_attacks(source_square, self.occupancies[Side::BOTH as usize]) & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate queen captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) == 0} else { self.occupancies[Side::WHITE as usize].get(target_square) == 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));
                            } else {
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate king moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteKing as usize;
                } else { piece_to_check = Piece::BlackKing as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        source_square = bitboard.ls1b() as usize;

                        // init piece attacks in order to get set of target squares
                        attacks = Bitboard(KING_ATTACKS[source_square] & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });

                        // generate king captures
                        while attacks.0 != 0 {
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) == 0} else { self.occupancies[Side::WHITE as usize].get(target_square) == 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 0, 0, 0, 0));
                            } else {
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));                               
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }
            }
        }
    }

    pub fn generate_captures(&self, move_list: &mut MoveList) {
        // define source & target squares
        let mut source_square;
        let mut target_square;

        // define current piece's bitboard copy & it's attacks
        let mut bitboard: Bitboard;
        let mut attacks;

        // loop over all the bitboards
        unsafe {
            for piece in 0..12 {
                // init piece bitboard copy
                bitboard = self.bitboards[piece as usize];
                // generate white pawns attacks
                if self.side == Side::WHITE {
                    if piece == Piece::WhitePawn as usize {
                        // loop over white pawns within white pawn bitboard
                        while bitboard.0 != 0 {
                            // get least significant 1st bit index
                            source_square = bitboard.ls1b() as usize;
                            // pop LS1B in bitboard
                            // bitboard.pop(source_square);
                            
                            // get pawn's target square
                            target_square = source_square - 8;
                            
                            // init pawn attacks bitboard
                            attacks = Bitboard(PAWN_ATTACKS[self.side as usize][source_square] & self.occupancies[Side::BLACK as usize].0);
                            // generate pawn captures
                            while attacks.0 != 0 {
                                // get least significant 1st bit index
                                target_square = attacks.ls1b() as usize;

                                if source_square >= Square::A7 as usize && source_square <= Square::H7 as usize {
                                    // pawn promotion capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteQueen as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteKnight as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteBishop as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::WhiteRook as u8, 1, 0, 0, 0));
                                } else {
                                    // pawn capture normal
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                                }
                                // pop LS1B in bitboard
                                attacks.pop(target_square);
                            }

                            // generate enpassant captures
                            if self.enpassant != Square::NoSquare {
                                // lookup pawn attacks and bitwise AND with enpassant square (bit)
                                let enpassant_attacks = PAWN_ATTACKS[self.side as usize][source_square] & (1 << self.enpassant as usize);
                                // make sure enpassant capture is available
                                if enpassant_attacks != 0 {
                                    // get least significant 1st bit index
                                    target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                    // enpassant capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 1, 0));
                                }
                            }

                            // pop ls1b from piece bitboard copy
                            bitboard.pop(source_square);
                        }
                    }
                // generate black pawns attacks
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
                            
                            // init pawn attacks bitboard
                            attacks = Bitboard(PAWN_ATTACKS[self.side as usize][source_square] & self.occupancies[Side::WHITE as usize].0);
                            // generate pawn captures
                            while attacks.0 != 0 {
                                // get least significant 1st bit index
                                target_square = attacks.ls1b() as usize;

                                // pawn promotion
                                if source_square >= Square::A2 as usize && source_square <= Square::H2 as usize {
                                    // add move into a move list
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackQueen as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackKnight as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackBishop as u8, 1, 0, 0, 0));
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, Piece::BlackRook as u8, 1, 0, 0, 0));
                                } else {
                                    // pawn capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                                }
                                // pop LS1B in bitboard
                                attacks.pop(target_square);
                            }

                            // generate enpassant captures
                            if self.enpassant != Square::NoSquare {
                                // lookup pawn attacks and bitwise AND with enpassant square (bit)
                                let enpassant_attacks = PAWN_ATTACKS[self.side as usize][source_square] & (1 << self.enpassant as usize);
                                // make sure enpassant capture is available
                                if enpassant_attacks != 0 {
                                    // get least significant 1st bit index
                                    target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                    // enpassant capture
                                    move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 1, 0));
                                }
                            }

                            // pop ls1b from piece bitboard copy
                            bitboard.pop(source_square);
                        }
                    }
                }

                // generate knight moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteKnight as usize;
                } else { piece_to_check = Piece::BlackKnight as usize; }
                
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init knight attacks bitboard
                        attacks = Bitboard(KNIGHT_ATTACKS[source_square] & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate knight captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) != 0} else { self.occupancies[Side::WHITE as usize].get(target_square) != 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0)); 
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate bishop moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteBishop as usize;
                } else { piece_to_check = Piece::BlackBishop as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init bishop attacks bitboard
                        attacks = Bitboard(get_bishop_attacks(source_square, self.occupancies[Side::BOTH as usize]) & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate bishop captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) != 0} else { self.occupancies[Side::WHITE as usize].get(target_square) != 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));   
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate rook moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteRook as usize;
                } else { piece_to_check = Piece::BlackRook as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init rook attacks bitboard
                        let occ;
                        if self.side == Side::WHITE { occ = self.occupancies[Side::WHITE as usize]; } else { occ = self.occupancies[Side::BLACK as usize]; }
                        attacks = Bitboard(get_rook_attacks(source_square, self.occupancies[Side::BOTH as usize]) & !occ.0);
                        // generate rook captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) != 0} else { self.occupancies[Side::WHITE as usize].get(target_square) != 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate queen moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteQueen as usize;
                } else { piece_to_check = Piece::BlackQueen as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        // get least significant 1st bit index
                        source_square = bitboard.ls1b() as usize;

                        // init queen attacks bitboard
                        attacks = Bitboard(get_queen_attacks(source_square, self.occupancies[Side::BOTH as usize]) & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });
                        // generate queen captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) != 0} else { self.occupancies[Side::WHITE as usize].get(target_square) != 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }

                // generate king moves
                let piece_to_check;
                if self.side == Side::WHITE as usize { piece_to_check = Piece::WhiteKing as usize;
                } else { piece_to_check = Piece::BlackKing as usize; }
                if piece == piece_to_check {
                    while bitboard.0 != 0 {
                        source_square = bitboard.ls1b() as usize;

                        // init piece attacks in order to get set of target squares
                        attacks = Bitboard(KING_ATTACKS[source_square] & if self.side == Side::WHITE { !self.occupancies[Side::WHITE as usize].0 } else { !self.occupancies[Side::BLACK as usize].0 });

                        // generate king captures
                        while attacks.0 != 0 {
                            target_square = attacks.ls1b() as usize;

                            if if self.side == Side::WHITE { self.occupancies[Side::BLACK as usize].get(target_square) != 0} else { self.occupancies[Side::WHITE as usize].get(target_square) != 0}{
                                // add move into a move list
                                move_list.add(encode_move(source_square as u8, target_square as u8, piece as u8, 0, 1, 0, 0, 0));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }
                        // pop LS1B in bitboard
                        bitboard.pop(source_square);
                    }
                }
            }
        }
    
    }
}