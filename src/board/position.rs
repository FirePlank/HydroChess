use std::collections::HashMap;

use super::attacks::*;
use crate::r#move::*;
use crate::evaluation::*;
use crate::board::*;
use crate::search::OPTIONS;


#[allow(dead_code)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    A8,B8,C8,D8,E8,F8,G8,H8,
    A7,B7,C7,D7,E7,F7,G7,H7,
    A6,B6,C6,D6,E6,F6,G6,H6,
    A5,B5,C5,D5,E5,F5,G5,H5,
    A4,B4,C4,D4,E4,F4,G4,H4,
    A3,B3,C3,D3,E3,F3,G3,H3,
    A2,B2,C2,D2,E2,F2,G2,H2,
    A1,B1,C1,D1,E1,F1,G1,H1,
    NoSquare,
}
// square string list
#[allow(dead_code)]
#[rustfmt::skip]
pub const SQUARE_COORDS: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

// ASCII pieces
pub const ASCII_PIECES: [&str; 12] = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"];
// unicode pieces
pub const UNICODE_PIECES: [&str; 12] = ["♟", "♞", "♝", "♜", "♛", "♚", "♙", "♘", "♗", "♖", "♕", "♔"];
// convert ASCII character pieces to encoded constants
lazy_static! {
    #[rustfmt::skip]
    pub static ref ASCII_TO_SQUARE: HashMap<&'static str, Square> = HashMap::from([
        ("a8", Square::A8),("b8", Square::B8),("c8", Square::C8),("d8", Square::D8),("e8", Square::E8),("f8", Square::F8),("g8", Square::G8),("h8", Square::H8),
        ("a7", Square::A7),("b7", Square::B7),("c7", Square::C7),("d7", Square::D7),("e7", Square::E7),("f7", Square::F7),("g7", Square::G7),("h7", Square::H7),
        ("a6", Square::A6),("b6", Square::B6),("c6", Square::C6),("d6", Square::D6),("e6", Square::E6),("f6", Square::F6),("g6", Square::G6),("h6", Square::H6),
        ("a5", Square::A5),("b5", Square::B5),("c5", Square::C5),("d5", Square::D5),("e5", Square::E5),("f5", Square::F5),("g5", Square::G5),("h5", Square::H5),
        ("a4", Square::A4),("b4", Square::B4),("c4", Square::C4),("d4", Square::D4),("e4", Square::E4),("f4", Square::F4),("g4", Square::G4),("h4", Square::H4),
        ("a3", Square::A3),("b3", Square::B3),("c3", Square::C3),("d3", Square::D3),("e3", Square::E3),("f3", Square::F3),("g3", Square::G3),("h3", Square::H3),
        ("a2", Square::A2),("b2", Square::B2),("c2", Square::C2),("d2", Square::D2),("e2", Square::E2),("f2", Square::F2),("g2", Square::G2),("h2", Square::H2),
        ("a1", Square::A1),("b1", Square::B1),("c1", Square::C1),("d1", Square::D1),("e1", Square::E1),("f1", Square::F1),("g1", Square::G1),("h1", Square::H1),
    ]);
}

// castling rights
pub enum Castling {
    WK = 1,
    WQ = 2,
    BK = 4,
    BQ = 8,
}
// castling rights update constants
#[rustfmt::skip]
const CASTLING_RIGHTS: [u8; 64] = [
    7, 15, 15, 15, 3, 15, 15, 11, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 13, 15, 15, 15, 12, 15, 15, 14,
];

// pieces
#[derive(Debug)]
pub enum Piece {
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
}
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(PartialEq)]
pub enum Variant {
    Standard,
    Suicide
}

#[derive(Clone, Copy, Debug)]
pub struct Side;
impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
    pub const BOTH: usize = 2;
}

#[derive(Debug, Clone)]
pub struct Position {
    pub bitboards: [Bitboard; 12],
    pub occupancies: [Bitboard; 2],
    pub side: usize,
    pub enpassant: Square,
    pub castle: u8,
    pub halfmove: u16,
    pub fullmove: u16,
    pub hash: u64,
    pub null_moves: u8,
    pub halfmove_clocks_stack: Vec<u16>,
    pub captured_pieces_stack: Vec<u8>,
    pub castling_rights_stack: Vec<u8>,
    pub en_passant_stack: Vec<Square>,
    pub hash_stack: Vec<u64>,
    pub material_scores: [[i16; 2]; 2],
    pub pst_scores: [[i16; 2]; 2],
    pub mobility: [i16; 12],
}

impl Position {
    pub fn new() -> Position {
        let mut pos = Position {
            // initialize bitboards to the default chess position at the start of a standard game
            bitboards: [
                Bitboard(71776119061217280),
                Bitboard(4755801206503243776),
                Bitboard(2594073385365405696),
                Bitboard(9295429630892703744),
                Bitboard(576460752303423488),
                Bitboard(1152921504606846976),
                Bitboard(65280),
                Bitboard(66),
                Bitboard(36),
                Bitboard(129),
                Bitboard(8),
                Bitboard(16),
            ],
            occupancies: [
                Bitboard(18446462598732840960),
                Bitboard(65535),
            ],
            side: Side::WHITE,
            enpassant: Square::NoSquare,
            castle: 15, // <--- all castles allowed
            halfmove: 0,
            fullmove: 0,
            hash: 0,
            null_moves: 0,
            halfmove_clocks_stack: Vec::with_capacity(32),
            captured_pieces_stack: Vec::with_capacity(32),
            castling_rights_stack: Vec::with_capacity(32),
            en_passant_stack: Vec::with_capacity(32),
            hash_stack: Vec::with_capacity(32),
            material_scores: [[0; 2]; 2],
            pst_scores: [[0; 2]; 2],
            mobility: [0; 12],
        };
        pos.hash = pos.generate_hash_key();
        init_calculation(&mut pos);
        return pos;
    }
    pub const fn empty() -> Position {
        Position {
            bitboards: [Bitboard(0); 12],
            occupancies: [Bitboard(0); 2],
            side: Side::WHITE,
            enpassant: Square::NoSquare,
            castle: 0,
            halfmove: 0,
            fullmove: 0,
            hash: 0,
            null_moves: 0,
            halfmove_clocks_stack: vec![],
            captured_pieces_stack: vec![],
            castling_rights_stack: vec![],
            en_passant_stack: vec![],
            hash_stack: vec![],
            material_scores: [[0; 2]; 2],
            pst_scores: [[0; 2]; 2],
            mobility: [0; 12],
        }
    }

    pub fn phase(&self) -> u32 {
        let mut phase: u32 = 0;
        phase += (self.bitboards[Piece::WhiteKnight as usize].0 | self.bitboards[Piece::WhiteBishop as usize].0).count_ones();
        phase += (self.bitboards[Piece::BlackKnight as usize].0 | self.bitboards[Piece::BlackBishop as usize].0).count_ones();
        phase += (self.bitboards[Piece::WhiteRook as usize].0 | self.bitboards[Piece::BlackRook as usize].0).count_ones() * 2;
        phase += (self.bitboards[Piece::WhiteQueen as usize].0 | self.bitboards[Piece::BlackQueen as usize].0).count_ones() * 4;
        return phase;
    }

    pub fn get_square_piece(&self, square: usize ) -> usize {
        let start_piece;
        let end_piece;
        if self.side == 0 {
            start_piece = Piece::BlackPawn as usize;
            end_piece = Piece::BlackKing as usize;
        } else {
            start_piece = Piece::WhitePawn as usize;
            end_piece = Piece::WhiteKing as usize;
        }
        // loop over bitboards opposite to the current side to move
        for piece in start_piece..end_piece {
            // if there's a piece on the target square
            if self.bitboards[piece].get(square) != 0 {
                return piece;
            }
        }
        return 0;
    }

    pub fn is_legal(&self) -> bool {
        // check if the position is legal
        if self.side == 1 {
            if self.is_attacked(self.bitboards[Piece::BlackKing as usize].ls1b() as usize, 0) {
                // move is illegal
                return false;
            }
        } else {
            // check if the move is illegal
            if self.is_attacked(self.bitboards[Piece::WhiteKing as usize].ls1b() as usize, 1) {
                // move is illegal
                return false;
            }
        }
        return true;
    }
    // Moves `piece` from the field specified by `from` to the field specified by `to` with the specified `color`, also updates occupancy and incremental values.
    pub fn move_piece(&mut self, color: u8, piece: u8, from: usize, to: usize) {
        //self.pieces[color as usize][piece as usize] ^= (1u64 << from) | (1u64 << to);
        self.occupancies[color as usize].0 ^= (1u64 << from) | (1u64 << to);

        // piece table
        self.bitboards[piece as usize].pop(to);
        self.bitboards[piece as usize].set(from);

        // -6 the piece index if its black
        let both = Bitboard(self.occupancies[0].0 | self.occupancies[1].0);
        if color == 1 {
            match piece as usize {
                // mobility
                8 => {
                    self.mobility[8] = get_bishop_attacks(to, both).count_ones() as i16 - 7;
                }
                9 => {
                    self.mobility[9] = get_rook_attacks(to, both).count_ones() as i16 - 7;
                }
                10 => {
                    self.mobility[10] = get_queen_attacks(to, both).count_ones() as i16 - 7;
                },
                _ => ()
            }


            let index = (piece - 6) as usize;
            self.pst_scores[color as usize][0] -= PSQT[index][to^56];
            self.pst_scores[color as usize][1] -= PSQT_EG[index][to^56];
            self.pst_scores[color as usize][0] += PSQT[index][from^56];
            self.pst_scores[color as usize][1] += PSQT_EG[index][from^56];
        } else {
            match piece as usize {
                // mobility
                2 => {
                    self.mobility[2] = get_bishop_attacks(to, both).count_ones() as i16 - 7;
                }
                3 => {
                    self.mobility[3] = get_rook_attacks(to, both).count_ones() as i16 - 7;
                }
                4 => {
                    self.mobility[4] = get_queen_attacks(to, both).count_ones() as i16 - 7;
                },
                _ => ()
            }

            self.pst_scores[color as usize][0] -= PSQT[piece as usize][to];
            self.pst_scores[color as usize][1] -= PSQT_EG[piece as usize][to];
            self.pst_scores[color as usize][0] += PSQT[piece as usize][from];
            self.pst_scores[color as usize][1] += PSQT_EG[piece as usize][from];
        }
    }

    // Adds `piece` on the `field` with the specified `color`, also updates occupancy and incremental values.
    pub fn add_piece(&mut self, color: u8, piece: u8, field: u8) {
        // self.pieces[color as usize][piece as usize] |= 1u64 << field;
        self.occupancies[color as usize].0 |= 1u64 << field;
        self.bitboards[piece as usize].set(field as usize);

        // -6 the piece index if its black
        let both = Bitboard(self.occupancies[0].0 | self.occupancies[1].0);
        if color == 1 {
            match piece as usize {
                // mobility
                8 => {
                    self.mobility[8] = get_bishop_attacks(field as usize, both).count_ones() as i16 - 7;
                }
                9 => {
                    self.mobility[9] = get_rook_attacks(field as usize, both).count_ones() as i16 - 7;
                }
                10 => {
                    self.mobility[10] = get_queen_attacks(field as usize, both).count_ones() as i16 - 7;
                },
                _ => ()
            }

            let index = (piece - 6) as usize;
            self.pst_scores[color as usize][0] += PSQT[index][(field^56) as usize];
            self.pst_scores[color as usize][1] += PSQT_EG[index][(field^56) as usize];
            self.material_scores[color as usize][0] += PIECE_VALUE[index];
            self.material_scores[color as usize][1] += PIECE_VALUE_EG[index];
        } else {
            match piece as usize {
                // mobility
                2 => {
                    self.mobility[2] = get_bishop_attacks(field as usize, both).count_ones() as i16 - 6;
                }
                3 => {
                    self.mobility[3] = get_rook_attacks(field as usize, both).count_ones() as i16 - 7;
                }
                4 => {
                    self.mobility[4] = get_queen_attacks(field as usize, both).count_ones() as i16 - 7;
                },
                _ => ()
            }

            self.material_scores[color as usize][0] += PIECE_VALUE[piece as usize];
            self.material_scores[color as usize][1] += PIECE_VALUE_EG[piece as usize];
            self.pst_scores[color as usize][0] += PSQT[piece as usize][field as usize];
            self.pst_scores[color as usize][1] += PSQT_EG[piece as usize][field as usize];
        }
    }

    // Removes `piece` on the `field` with the specified `color`, also updates occupancy and incremental values.
    pub fn remove_piece(&mut self, color: u8, piece: u8, field: u8) {
        //self.pieces[color as usize][piece as usize] &= !(1u64 << field);
        self.occupancies[color as usize].0 &= !(1u64 << field);
        self.bitboards[piece as usize].pop(field as usize);

        // -6 the piece index if its black
        if color == 1 {
            let index = (piece - 6) as usize;
            self.material_scores[color as usize][0] -= PIECE_VALUE[index];
            self.material_scores[color as usize][1] -= PIECE_VALUE_EG[index];
            self.pst_scores[color as usize][0] -= PSQT[index][(field^56) as usize];
            self.pst_scores[color as usize][1] -= PSQT_EG[index][(field^56) as usize];
        } else {          
            self.material_scores[color as usize][0] -= PIECE_VALUE[piece as usize];
            self.material_scores[color as usize][1] -= PIECE_VALUE_EG[piece as usize];
            self.pst_scores[color as usize][0] -= PSQT[piece as usize][field as usize];
            self.pst_scores[color as usize][1] -= PSQT_EG[piece as usize][field as usize];
        }
    }

    pub fn get_piece(&self, square: u8) -> usize {
        let start_piece;
        let end_piece;
        if self.side == 0 {
            // white to move
            start_piece = Piece::BlackPawn as usize;
            end_piece = Piece::BlackKing as usize;
        } else {
            // black to move
            start_piece = Piece::WhitePawn as usize;
            end_piece = Piece::WhiteKing as usize;
        }

        // loop over bitboard opposite to the current side to move
        for bb_piece in start_piece..end_piece + 1 {
            // if there is a piece on the target square
            if self.bitboards[bb_piece].get(square as usize) != 0 {
                return bb_piece % 6;
            }
        }
        return 15;
    }

    pub fn make(&mut self, move_: u32) -> bool {
        let opp_color = self.side ^ 1;
        // parse move
        let source_square = source(move_);
        let target_square = target(move_);
        let capture = capture(move_);
        let piece = get_piece(move_);
        let promoted = promoted(move_);
        let enpassant = enpassant(move_);
        let double = double(move_);
        let castling = castling(move_);

        self.halfmove_clocks_stack.push(self.halfmove);
        self.castling_rights_stack.push(self.castle);
        self.en_passant_stack.push(self.enpassant);
        self.hash_stack.push(self.hash);
        
        // if self.enpassant != Square::NoSquare {
        //     self.hash ^= unsafe { ZOBRIST_EP_KEYS[self.enpassant as usize & 7] };
        //     self.enpassant = Square::NoSquare;
        // }

        // move piece
        self.move_piece(
            self.side as u8,
            piece as u8,
            target_square as usize,
            source_square as usize,
        );

        // update scores
        // if piece == 0 || piece == Piece::BlackPawn as u8 {
        //     if capture != 0 {
        //         self.calculate_isolated(target_square, false);
        //     }
        //     self.calculate_passed(target_square, false);
        // } else if piece == Piece::WhiteRook as u8 || piece == Piece::BlackRook as u8 {
        //     self.calculate_rook(target_square, false);
        // }

        // hash piece
        unsafe {
            // remove piece from source square in hash key
            self.hash ^= ZOBRIST_KEYS[piece as usize][source_square as usize];
            // add piece to target square in hash key
            self.hash ^= ZOBRIST_KEYS[piece as usize][target_square as usize];
        }

        if capture != 0 {
            // pick up bitboard piece index ranges depending on side
            let start_piece;
            let end_piece;

            if self.side == 0 {
                // white to move
                start_piece = Piece::BlackPawn as usize;
                end_piece = Piece::BlackKing as usize;
            } else {
                // black to move
                start_piece = Piece::WhitePawn as usize;
                end_piece = Piece::WhiteKing as usize;
            }

            // loop over bitboard opposite to the current side to move
            for bb_piece in start_piece..end_piece + 1 {
                // if there is a piece on the target square
                if self.bitboards[bb_piece].get(target_square as usize) != 0 {
                    // remove it from corresponding bitboard
                    self.captured_pieces_stack.push(bb_piece as u8);
                    self.remove_piece(opp_color as u8, bb_piece as u8, target_square as u8);
                    unsafe {
                        // remove piece from hash key
                        self.hash ^= ZOBRIST_KEYS[bb_piece as usize][target_square as usize];
                    }
                    
                    break;
                }
            }
        }
        // handle pawn promotions
        if promoted != 0 {
            // erase the pawn from the target square and remove from hash key
            unsafe {
                if self.side == 0 {
                    self.remove_piece(0, Piece::WhitePawn as u8, target_square);
                    self.hash ^= ZOBRIST_KEYS[Piece::WhitePawn as usize][target_square as usize];
                } else {
                    self.remove_piece(1, Piece::BlackPawn as u8, target_square);
                    self.hash ^= ZOBRIST_KEYS[Piece::BlackPawn as usize][target_square as usize];
                }
                // set up promoted piece on chess board
                self.add_piece(self.side as u8, promoted, target_square);
                // add promoted piece to hash key
                self.hash ^= ZOBRIST_KEYS[promoted as usize][target_square as usize];
            }
        }
        // handle enpassant captures
        unsafe {
            if enpassant != 0 {
                // erase the pawn from the target square
                if self.side == 0 {
                    self.remove_piece(opp_color as u8, Piece::BlackPawn as u8, target_square + 8);
                    // hash enpassant
                    self.hash ^= ZOBRIST_KEYS[Piece::BlackPawn as usize][target_square as usize + 8];
                } else {
                    self.remove_piece(opp_color as u8, Piece::WhitePawn as u8, target_square - 8);
                    // hash enpassant
                    self.hash ^= ZOBRIST_KEYS[Piece::WhitePawn as usize][target_square as usize - 8];
                }
            }
            if self.enpassant != Square::NoSquare {
                self.hash ^= ZOBRIST_EP_KEYS[self.enpassant as usize];
            }
        }
        self.enpassant = Square::NoSquare;

        // handle double pawn push
        if double != 0 {
            // set enpassant square
            unsafe {
                if self.side == 0 {
                    // using transmute as its a few hudred nanoseconds faster than indexing a list with the squares lol
                    self.enpassant = std::mem::transmute(target_square + 8);
                    // hash enpassant
                    self.hash ^= ZOBRIST_EP_KEYS[target_square as usize + 8];
                } else {
                    self.enpassant = std::mem::transmute(target_square - 8);
                    // hash enpassant
                    self.hash ^= ZOBRIST_EP_KEYS[target_square as usize - 8];
                }
            }
        }

        // handle castling
        if castling != 0 {
            // move the rook
            match target_square {
                62 => {
                    // move H rook white
                    self.move_piece(
                        0,
                        Piece::WhiteRook as u8,
                        Square::F1 as usize,
                        Square::H1 as usize,
                    );

                    // hash rook
                    unsafe {
                        self.hash ^= ZOBRIST_KEYS[Piece::WhiteRook as usize][Square::H1 as usize];
                        self.hash ^= ZOBRIST_KEYS[Piece::WhiteRook as usize][Square::F1 as usize];
                    }
                }
                58 => {
                    // move A rook white
                    self.move_piece(
                        0,
                        Piece::WhiteRook as u8,
                        Square::D1 as usize,
                        Square::A1 as usize,
                    );

                    // hash rook
                    unsafe {
                        self.hash ^= ZOBRIST_KEYS[Piece::WhiteRook as usize][Square::A1 as usize];
                        self.hash ^= ZOBRIST_KEYS[Piece::WhiteRook as usize][Square::D1 as usize];
                    }
                }
                6 => {
                    // move H rook black
                    self.move_piece(
                        1,
                        Piece::BlackRook as u8,
                        Square::F8 as usize,
                        Square::H8 as usize,
                    );

                    // hash rook
                    unsafe {
                        self.hash ^= ZOBRIST_KEYS[Piece::BlackRook as usize][Square::H8 as usize];
                        self.hash ^= ZOBRIST_KEYS[Piece::BlackRook as usize][Square::F8 as usize];
                    }
                }
                2 => {
                    // move A rook black
                    self.move_piece(
                        1,
                        Piece::BlackRook as u8,
                        Square::D8 as usize,
                        Square::A8 as usize,
                    );

                    // hash rook
                    unsafe {
                        self.hash ^= ZOBRIST_KEYS[Piece::BlackRook as usize][Square::A8 as usize];
                        self.hash ^= ZOBRIST_KEYS[Piece::BlackRook as usize][Square::D8 as usize];
                    }
                }
                _ => panic!("Invalid castling move: {}", target_square),
            }
        }

        // hash castling
        unsafe {
            self.hash ^= ZOBRIST_CASTLING_KEYS[self.castle as usize];
        }

        // update castling rights
        self.castle &= CASTLING_RIGHTS[source_square as usize];
        self.castle &= CASTLING_RIGHTS[target_square as usize];

        // hash castling
        unsafe {
            self.hash ^= ZOBRIST_CASTLING_KEYS[self.castle as usize];
        }
        // change position variables
        self.side = opp_color;

        // hash side
        unsafe {
            self.hash ^= ZOBRIST_TURN;
        }
        // self.hash = self.generate_hash_key();
        // if self.hash != self.generate_hash_key() {
        //     print!("move: ");
        //     Move(move_).show();
        //     println!();
        // }

        // update half move clock
        if piece == 0 || capture != 0 {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }

        if self.side == 0 {
            self.fullmove += 1;
            if unsafe { OPTIONS.variant == Variant::Suicide} {
                return true;
            }
            // check if the move is illegal
            if self.is_attacked(self.bitboards[Piece::BlackKing as usize].ls1b() as usize, 0) {
                // move is illegal
                return false;
            }
        } else {
            if unsafe { OPTIONS.variant == Variant::Suicide} {
                return true;
            }
            // check if the move is illegal
            if self.is_attacked(self.bitboards[Piece::WhiteKing as usize].ls1b() as usize, 1) {
                // move is illegal
                return false;
            }
        }
        return true;
    }

    pub fn unmake(&mut self, move_: u32) {
        let opp_color = self.side;
        self.side ^= 1;

        // parse move
        let from = source(move_);
        let to = target(move_);
        let piece = get_piece(move_);
        let promoted = promoted(move_);
        let enpassant = enpassant(move_);
        let castling = castling(move_);
        let capture = capture(move_);

        self.halfmove = self.halfmove_clocks_stack.pop().unwrap();
        self.castle = self.castling_rights_stack.pop().unwrap();
        self.enpassant = self.en_passant_stack.pop().unwrap();
        self.hash = self.hash_stack.pop().unwrap();

        // update score
        // if piece == 0 || piece == Piece::BlackPawn as u8 {
        //     if capture != 0 {
        //         self.calculate_isolated(to, true);
        //     }
        //     self.calculate_passed(to, true);
        // } else if piece == Piece::WhiteRook as u8 || piece == Piece::BlackRook as u8 {
        //     self.calculate_rook(to, true);
        // }

        // check flags to determine how to proceed with undoing the move
        if castling != 0 {
            match to {
                62 => {
                    self.move_piece(0, Piece::WhiteKing as u8, 60, 62);
                    self.move_piece(0, Piece::WhiteRook as u8, 63, 61);
                }
                58 => {
                    self.move_piece(0, Piece::WhiteKing as u8, 60, 58);
                    self.move_piece(0, Piece::WhiteRook as u8, 56, 59);
                }
                6 => {
                    self.move_piece(1, Piece::BlackKing as u8, 4, 6);
                    self.move_piece(1, Piece::BlackRook as u8, 7, 5);
                }
                2 => {
                    self.move_piece(1, Piece::BlackKing as u8, 4, 2);
                    self.move_piece(1, Piece::BlackRook as u8, 0, 3);
                }
                _ => panic!("Invalid castling move: {}", to),
            }
        } else if enpassant != 0 {
            self.move_piece(self.side as u8, piece, from as usize, to as usize);
            if self.side == 0 {
                self.add_piece(1, Piece::BlackPawn as u8, to + 8);
            } else {
                self.add_piece(0, Piece::WhitePawn as u8, to - 8);
            }
        } else if capture != 0 && promoted == 0 {
            let captured_piece = self.captured_pieces_stack.pop().unwrap();
            self.move_piece(self.side as u8, piece, from as usize, to as usize);
            self.add_piece(opp_color as u8, captured_piece, to);
        } else if capture == 0 && promoted == 0 {
            self.move_piece(self.side as u8, piece, from as usize, to as usize);
        } else {
            if self.side == 0 {
                self.add_piece(0, Piece::WhitePawn as u8, from);
            } else {
                self.add_piece(1, Piece::BlackPawn as u8, from);
            }
            self.remove_piece(self.side as u8, promoted, to);

            if capture != 0 {
                let captured_piece = self.captured_pieces_stack.pop().unwrap();
                self.add_piece(opp_color as u8, captured_piece, to);
            }
        }

        if self.side == 1 {
            self.fullmove -= 1;
        }
    }

    pub fn show(&self, unicode: bool) {
        let pieces;
        if unicode {
            pieces = UNICODE_PIECES;
        } else {
            pieces = ASCII_PIECES;
        }
        // loop over board ranks
        for rank in 0..8 {
            for file in 0..8 {
                // init square
                let square = rank * 8 + file;

                if file == 0 {
                    print!("{}  ", 8 - rank);
                }

                // define piece
                let mut piece: i8 = -1;
                // loop over all the piece bitboards
                for bb_piece in 0..12 {
                    if self.bitboards[bb_piece].get(square) != 0 {
                        piece = bb_piece as i8;
                        break;
                    }
                }

                if piece == -1 {
                    print!(". ");
                } else {
                    print!("{} ", pieces[piece as usize]);
                }
            }
            // print new line every rank
            println!();
        }
        // print board files
        println!("   a b c d e f g h\n");
        // print side to move
        println!(
            "   Side to move: {}",
            if self.side == Side::WHITE {
                "White"
            } else {
                "Black"
            }
        );
        // print enpassant
        println!(
            "   Enpassant: {}",
            if self.enpassant == Square::NoSquare {
                "None"
            } else {
                SQUARE_COORDS[self.enpassant as usize]
            }
        );
        // print castling rights
        println!(
            "   Castling: {}",
            if self.castle == 0 {
                "None".to_string()
            } else {
                let mut castling = String::new();
                if self.castle & Castling::WK as u8 != 0 {
                    castling.push('K');
                } else {
                    castling.push('-');
                }
                if self.castle & Castling::WQ as u8 != 0 {
                    castling.push('Q');
                } else {
                    castling.push('-');
                }
                if self.castle & Castling::BK as u8 != 0 {
                    castling.push('k');
                } else {
                    castling.push('-');
                }
                if self.castle & Castling::BQ as u8 != 0 {
                    castling.push('q');
                } else {
                    castling.push('-');
                }
                castling
            }
        );
        // print hash key
        println!("   Hash: {}", self.hash);
        // print halfmove clock
        println!("   Halfmove clock: {}", self.halfmove);
        // print fullmove number
        println!("   Fullmove number: {}\n", self.fullmove);
    }

    pub fn from_fen(fen: &str) -> Position {
        let mut position = Position::empty();

        let mut square = 0;
        let mut rank = 0;
        let mut file = 0;
        let mut index = 0;
        for x in fen.split_whitespace() {
            if index == 0 {
                for i in x.chars() {
                    match i {
                        '1' => {
                            file += 1;
                            square = rank * 8 + file;
                        }
                        '2' => {
                            file += 2;
                            square = rank * 8 + file;
                        }
                        '3' => {
                            file += 3;
                            square = rank * 8 + file;
                        }
                        '4' => {
                            file += 4;
                            square = rank * 8 + file;
                        }
                        '5' => {
                            file += 5;
                            square = rank * 8 + file;
                        }
                        '6' => {
                            file += 6;
                            square = rank * 8 + file;
                        }
                        '7' => {
                            file += 7;
                            square = rank * 8 + file;
                        }
                        '8' => {
                            file += 8;
                            square = rank * 8 + file;
                        }
                        'P' => {
                            position.bitboards[Piece::WhitePawn as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'N' => {
                            position.bitboards[Piece::WhiteKnight as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'B' => {
                            position.bitboards[Piece::WhiteBishop as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'R' => {
                            position.bitboards[Piece::WhiteRook as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'Q' => {
                            position.bitboards[Piece::WhiteQueen as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'K' => {
                            position.bitboards[Piece::WhiteKing as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'p' => {
                            position.bitboards[Piece::BlackPawn as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'n' => {
                            position.bitboards[Piece::BlackKnight as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'b' => {
                            position.bitboards[Piece::BlackBishop as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'r' => {
                            position.bitboards[Piece::BlackRook as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'q' => {
                            position.bitboards[Piece::BlackQueen as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        'k' => {
                            position.bitboards[Piece::BlackKing as usize].set(square);
                            file += 1;
                            square = rank * 8 + file;
                        }
                        '/' => {
                            rank += 1;
                            file = 0;
                        }
                        _ => (),
                    }
                }
            } else if index == 1 {
                if x == "w" {
                    position.side = Side::WHITE;
                } else {
                    position.side = Side::BLACK;
                }
            } else if index == 2 {
                for i in x.chars() {
                    match i {
                        'K' => {
                            position.castle |= Castling::WK as u8;
                        }
                        'Q' => {
                            position.castle |= Castling::WQ as u8;
                        }
                        'k' => {
                            position.castle |= Castling::BK as u8;
                        }
                        'q' => {
                            position.castle |= Castling::BQ as u8;
                        }
                        _ => (),
                    }
                }
            } else if index == 3 {
                if x != "-" {
                    position.enpassant = *ASCII_TO_SQUARE.get(x).unwrap_or_else(|| {
                        return &Square::NoSquare;
                    });
                    if position.enpassant == Square::NoSquare {
                        println!("info string Invalid fen given");
                        return Position::empty();
                    }
                }
            } else if index == 4 {
                position.halfmove = x.parse::<u16>().unwrap();
            } else if index == 5 {
                position.fullmove = x.parse::<u16>().unwrap();
            }
            index += 1;
        }

        // loop over white pieces bitboards
        for piece in Piece::WhitePawn as usize..Piece::WhiteKing as usize + 1 {
            position.occupancies[Side::WHITE].0 |= position.bitboards[piece].0;
        }
        // loop over black pieces bitboards
        for piece in Piece::BlackPawn as usize..Piece::BlackKing as usize + 1 {
            position.occupancies[Side::BLACK].0 |= position.bitboards[piece].0;
        }
        // init all occupancies
        // position.occupancies[Side::BOTH].0 |= position.occupancies[Side::WHITE].0;
        // position.occupancies[Side::BOTH].0 |= position.occupancies[Side::BLACK].0;

        // initalize history vectors for unmake/make functions
        position.halfmove_clocks_stack = Vec::with_capacity(32);
        position.captured_pieces_stack = Vec::with_capacity(32);
        position.castling_rights_stack = Vec::with_capacity(32);
        position.en_passant_stack = Vec::with_capacity(32);
        position.hash_stack = Vec::with_capacity(32);

        position.hash = position.generate_hash_key();
        position.null_moves = 0;

        init_calculation(&mut position);

        return position;
    }

    pub fn is_attacked(&self, square: usize, side: usize) -> bool {
        unsafe {
            // attacked by white and black pawns
            if (side == Side::WHITE
                && (PAWN_ATTACKS[Side::BLACK as usize][square]
                    & self.bitboards[Piece::WhitePawn as usize].0)
                    != 0)
                || (side == Side::BLACK
                    && (PAWN_ATTACKS[Side::WHITE as usize][square]
                        & self.bitboards[Piece::BlackPawn as usize].0)
                        != 0)
            {
                return true;
            }
            // attacked by knights
            if KNIGHT_ATTACKS[square]
                & (if side == Side::WHITE {
                    self.bitboards[Piece::WhiteKnight as usize].0
                } else {
                    self.bitboards[Piece::BlackKnight as usize].0
                })
                != 0
            {
                return true;
            }
            let both = Bitboard(self.occupancies[Side::WHITE].0 | self.occupancies[Side::BLACK].0);
            // attacked by bishops
            if get_bishop_attacks(square, both)
                & (if side == Side::WHITE {
                    self.bitboards[Piece::WhiteBishop as usize].0
                } else {
                    self.bitboards[Piece::BlackBishop as usize].0
                })
                != 0
            {
                return true;
            }
            // attacked by rooks
            if get_rook_attacks(square, both)
                & (if side == Side::WHITE {
                    self.bitboards[Piece::WhiteRook as usize].0
                } else {
                    self.bitboards[Piece::BlackRook as usize].0
                })
                != 0
            {
                return true;
            }
            // attacked by queens
            if get_queen_attacks(square, both)
                & (if side == Side::WHITE {
                    self.bitboards[Piece::WhiteQueen as usize].0
                } else {
                    self.bitboards[Piece::BlackQueen as usize].0
                })
                != 0
            {
                return true;
            }
            // attacked by king
            if KING_ATTACKS[square]
                & (if side == Side::WHITE {
                    self.bitboards[Piece::WhiteKing as usize].0
                } else {
                    self.bitboards[Piece::BlackKing as usize].0
                })
                != 0
            {
                return true;
            }

            return false;
        }
    }

    pub fn get_attackers(&self, square: usize, side: usize) -> u8 {
        let mut attackers = 0;
        
        let bishops_rooks = if side == Side::WHITE {
            self.bitboards[Piece::BlackBishop as usize].0
                | self.bitboards[Piece::BlackRook as usize].0
        } else {
            self.bitboards[Piece::WhiteBishop as usize].0
                | self.bitboards[Piece::WhiteRook as usize].0
        };
        let rooks_queens = if side == Side::WHITE {
            self.bitboards[Piece::BlackQueen as usize].0
                | self.bitboards[Piece::BlackRook as usize].0
        } else {
            self.bitboards[Piece::WhiteQueen as usize].0
                | self.bitboards[Piece::WhiteRook as usize].0
        };
        let bishops_queens = if side == Side::WHITE {
            self.bitboards[Piece::BlackQueen as usize].0
                | self.bitboards[Piece::BlackBishop as usize].0
        } else {
            self.bitboards[Piece::WhiteQueen as usize].0
                | self.bitboards[Piece::WhiteBishop as usize].0
        };

        let occupancy_all = self.occupancies[Side::WHITE].0 | self.occupancies[Side::BLACK].0;

        let mut knight_bishop_count = 0;
        let king_attacks;
        unsafe {
            king_attacks = KING_ATTACKS[square];
            if side == 0 {
                if king_attacks & self.bitboards[Piece::BlackKing as usize].0 != 0 {
                    attackers |= 1 << 7;
                }
                if get_queen_attacks(square, Bitboard(occupancy_all & !bishops_rooks)) & self.bitboards[Piece::BlackQueen as usize].0 != 0 {
                    attackers |= 1 << 6;
                }
                let rook_attacks = get_rook_attacks(square, Bitboard(occupancy_all & !rooks_queens));
                if rook_attacks & self.bitboards[Piece::BlackRook as usize].0 != 0 {
                    if (rook_attacks & self.bitboards[Piece::BlackRook as usize].0).count_ones() == 1 {
                        attackers |= 1 << 4;
                    } else {
                        attackers |= 3 << 4;
                    }
                }
                let knight_attacks = KNIGHT_ATTACKS[square];
                if knight_attacks & self.bitboards[Piece::BlackKnight as usize].0 != 0 {
                    knight_bishop_count += (knight_attacks & self.bitboards[Piece::BlackKnight as usize].0).count_ones();
                }
                let bishop_attacks = get_bishop_attacks(square, Bitboard(occupancy_all & !bishops_queens));
                if bishop_attacks & self.bitboards[Piece::BlackBishop as usize].0 != 0 {
                    knight_bishop_count += (bishop_attacks & self.bitboards[Piece::BlackBishop as usize].0).count_ones();
                }
            } else {
                if king_attacks & self.bitboards[Piece::WhiteKing as usize].0 != 0 {
                    attackers |= 1 << 7;
                }
                if get_queen_attacks(square, Bitboard(occupancy_all & !bishops_rooks)) & self.bitboards[Piece::WhiteQueen as usize].0 != 0 {
                    attackers |= 1 << 6;
                }
                let rook_attacks = get_rook_attacks(square, Bitboard(occupancy_all & !rooks_queens));
                if rook_attacks & self.bitboards[Piece::WhiteRook as usize].0 != 0 {
                    if (rook_attacks & self.bitboards[Piece::WhiteRook as usize].0).count_ones() == 1 {
                        attackers |= 1 << 4;
                    } else {
                        attackers |= 3 << 4;
                    }
                }
                let knight_attacks = KNIGHT_ATTACKS[square];
                if knight_attacks & self.bitboards[Piece::WhiteKnight as usize].0 != 0 {
                    knight_bishop_count += (knight_attacks & self.bitboards[Piece::WhiteKnight as usize].0).count_ones();
                }
                let bishop_attacks = get_bishop_attacks(square, Bitboard(occupancy_all & !bishops_queens));
                if bishop_attacks & self.bitboards[Piece::WhiteBishop as usize].0 != 0 {
                    knight_bishop_count += (bishop_attacks & self.bitboards[Piece::WhiteBishop as usize].0).count_ones();
                }
            }
        }

        if knight_bishop_count != 0 {
            if knight_bishop_count == 1 {
                attackers |= 1 << 1;
            } else if knight_bishop_count == 2 {
                attackers |= 3 << 1;
            } else {
                attackers |= 7 << 1;
            }
        }

        let sq_bb = SHIFT_LOCATIONS[square];
        let enemy_pawns = if side == Side::WHITE {
            self.bitboards[Piece::BlackPawn as usize].0 & king_attacks
        } else {
            self.bitboards[Piece::WhitePawn as usize].0 & king_attacks
        };
        let attacking_pawns = if side == Side::WHITE {
            (enemy_pawns >> 7 | enemy_pawns >> 9) & sq_bb
        } else {
            (enemy_pawns << 7 | enemy_pawns << 9) & sq_bb
        };
        if attacking_pawns != 0 {
            attackers |= 1;
        }

        return attackers;
    }

    pub fn show_attacked(&self, side: usize) {
        for rank in 0..8 {
            for file in 0..8 {
                let square = rank * 8 + file;
                if file == 0 {
                    print!("{}  ", 8 - rank);
                }
                if self.is_attacked(square, side) {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
        println!("   a b c d e f g h\n");
    }

    pub fn make_null_move(&mut self) {
        let color = self.side;
        let enemy_color = self.side ^ 1;

        self.halfmove_clocks_stack.push(self.halfmove);
        self.castling_rights_stack.push(self.castle);
        self.en_passant_stack.push(self.enpassant);
        self.hash_stack.push(self.hash);

        if self.enpassant != Square::NoSquare {
            self.hash ^= unsafe { ZOBRIST_EP_KEYS[self.enpassant as usize] };
            self.enpassant = Square::NoSquare;
        }

        if color == 1 {
            self.fullmove += 1;
        }

        self.null_moves += 1;
        self.side = enemy_color;
        self.hash ^= unsafe { ZOBRIST_TURN };
    }

    pub fn unmake_null_move(&mut self) {
        let color = self.side ^ 1;

        self.halfmove_clocks_stack.pop().unwrap();
        self.hash = self.hash_stack.pop().unwrap();
        self.enpassant = self.en_passant_stack.pop().unwrap();
        self.castle = self.castling_rights_stack.pop().unwrap();

        if color == 1 {
            self.fullmove -= 1;
        }

        self.side = color;
        self.null_moves -= 1;
    }

    pub fn is_insufficent_material(&self) -> bool {
        if (self.material_scores[0][0] - self.material_scores[1][0]).abs() < 391 && self.bitboards[0].count() == 0 && self.bitboards[Piece::WhiteRook as usize].count() == 0
        && self.bitboards[Piece::BlackPawn as usize].count() == 0 && self.bitboards[Piece::BlackRook as usize].count() == 0 {
            return true;
        }
        return false;
    }

    pub fn is_threefold(&self) -> bool {
        if self.hash_stack.len() < 6 || self.null_moves > 0 {
            return false;
        }

        let mut repetitions_count = 1;
        let mut from = self.hash_stack.len().wrapping_sub(self.halfmove as usize);
        let to = self.hash_stack.len() - 1;

        if from > 1024 {
            from = 0;
        }

        for hash_index in (from..to).rev().step_by(2) {
            if self.hash_stack[hash_index] == self.hash {
                repetitions_count += 1;

                if repetitions_count >= 3 {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn is_fifty(&self) -> bool {
        if self.null_moves > 0 {
            return false;
        }

        return self.halfmove >= 100;
    }

}