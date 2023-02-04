use crate::{board::{position::Position, Bitboard}, r#move::{movegen::MoveList, encode::capture}};
use crate::board::attacks::*;
use crate::board::position::*;
use crate::r#move::encode::*;
use crate::evaluation::*;

fn generate_all_moves(position: &mut Position, move_list: &mut MoveList) {
    // define source & target squares
    let mut source_square;
    let mut target_square;

    // combine both occupancy boards
    let both = Bitboard(position.occupancies[0].0 | position.occupancies[1].0);

    // define current piece's bitboard copy & it's attacks
    let mut bitboard: Bitboard;
    let mut attacks;

    // loop over all the bitboards
    unsafe {
        for piece in 0..12 {
            // init piece bitboard copy
            bitboard = position.bitboards[piece as usize];
            // generate white pawn moves
            if position.side == Side::WHITE {
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
                        if target_square >= Square::A8 as usize &&
                            (both.get(target_square) == 0)
                        {
                            // pawn promotion
                            if source_square >= Square::A7 as usize
                                && source_square <= Square::H7 as usize
                            {
                                // add move into a move list
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteQueen as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteKnight as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteBishop as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteRook as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));

                                // its possible to promote to a king in antichess
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteKing as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                            } else {
                                // one square ahead pawn move
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));

                                // two squares ahead pawn move
                                if (source_square >= Square::A2 as usize
                                    && source_square <= Square::H2 as usize)
                                    && (both
                                        .get(target_square - 8)
                                        == 0)
                                {
                                    target_square = source_square - 16;
                                    move_list.add(encode_move(
                                        source_square as u8,
                                        target_square as u8,
                                        piece as u8,
                                        0,
                                        0,
                                        1,
                                        0,
                                        0,
                                    ));
                                }
                            }
                        }

                        // init pawn attacks bitboard
                        attacks = Bitboard(
                            PAWN_ATTACKS[position.side as usize][source_square]
                                & position.occupancies[Side::BLACK as usize].0,
                        );
                        // generate pawn captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            if source_square >= Square::A7 as usize
                                && source_square <= Square::H7 as usize
                            {
                                // pawn promotion capture
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteQueen as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteKnight as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteBishop as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteRook as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));

                                // its possible to promote to a king in antichess
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::WhiteKing as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                            } else {
                                // pawn capture normal
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }

                        // generate enpassant captures
                        if position.enpassant != Square::NoSquare {
                            // lookup pawn attacks and bitwise AND with enpassant square (bit)
                            let enpassant_attacks = PAWN_ATTACKS[position.side as usize]
                                [source_square]
                                & (1 << position.enpassant as usize);
                            // make sure enpassant capture is available
                            if enpassant_attacks != 0 {
                                // get least significant 1st bit index
                                target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                // enpassant capture
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    1,
                                    0,
                                    1,
                                    0,
                                ));
                            }
                        }

                        // pop ls1b from piece bitboard copy
                        bitboard.pop(source_square);
                    }
                }
            // generate black pawn moves
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
                        if target_square <= Square::H1 as usize &&
                            (both.get(target_square) == 0)
                        {
                            // pawn promotion
                            if source_square >= Square::A2 as usize
                                && source_square <= Square::H2 as usize
                            {
                                // add move into a move list
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackQueen as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackKnight as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackBishop as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackRook as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));

                                // its possible to promote to a king in antichess
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackKing as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));
                            } else {
                                // one square ahead pawn move
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ));

                                // two squares ahead pawn move
                                if (source_square >= Square::A7 as usize
                                    && source_square <= Square::H7 as usize)
                                    && (both
                                        .get(target_square + 8)
                                        == 0)
                                {
                                    target_square = source_square + 16;
                                    move_list.add(encode_move(
                                        source_square as u8,
                                        target_square as u8,
                                        piece as u8,
                                        0,
                                        0,
                                        1,
                                        0,
                                        0,
                                    ));
                                }
                            }
                        }

                        // init pawn attacks bitboard
                        attacks = Bitboard(
                            PAWN_ATTACKS[position.side as usize][source_square]
                                & position.occupancies[Side::WHITE as usize].0,
                        );
                        // generate pawn captures
                        while attacks.0 != 0 {
                            // get least significant 1st bit index
                            target_square = attacks.ls1b() as usize;

                            // pawn promotion
                            if source_square >= Square::A2 as usize
                                && source_square <= Square::H2 as usize
                            {
                                // add move into a move list
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackQueen as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackKnight as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackBishop as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackRook as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));

                                // its possible to promote to a king in antichess
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    Piece::BlackKing as u8,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                            } else {
                                // pawn capture
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    1,
                                    0,
                                    0,
                                    0,
                                ));
                            }
                            // pop LS1B in bitboard
                            attacks.pop(target_square);
                        }

                        // generate enpassant captures
                        if position.enpassant != Square::NoSquare {
                            // lookup pawn attacks and bitwise AND with enpassant square (bit)
                            let enpassant_attacks = PAWN_ATTACKS[position.side as usize]
                                [source_square]
                                & (1 << position.enpassant as usize);
                            // make sure enpassant capture is available
                            if enpassant_attacks != 0 {
                                // get least significant 1st bit index
                                target_square = Bitboard(enpassant_attacks).ls1b() as usize;
                                // enpassant capture
                                move_list.add(encode_move(
                                    source_square as u8,
                                    target_square as u8,
                                    piece as u8,
                                    0,
                                    1,
                                    0,
                                    1,
                                    0,
                                ));
                            }
                        }

                        // pop ls1b from piece bitboard copy
                        bitboard.pop(source_square);
                    }
                }
            }

            // generate knight moves
            let piece_to_check;
            if position.side == Side::WHITE as usize {
                piece_to_check = Piece::WhiteKnight as usize;
            } else {
                piece_to_check = Piece::BlackKnight as usize;
            }

            if piece == piece_to_check {
                while bitboard.0 != 0 {
                    // get least significant 1st bit index
                    source_square = bitboard.ls1b() as usize;

                    // init knight attacks bitboard
                    attacks = Bitboard(
                        KNIGHT_ATTACKS[source_square]
                            & if position.side == Side::WHITE {
                                !position.occupancies[Side::WHITE as usize].0
                            } else {
                                !position.occupancies[Side::BLACK as usize].0
                            },
                    );
                    // generate knight captures
                    while attacks.0 != 0 {
                        // get least significant 1st bit index
                        target_square = attacks.ls1b() as usize;

                        if if position.side == Side::WHITE {
                            position.occupancies[Side::BLACK as usize].get(target_square) == 0
                        } else {
                            position.occupancies[Side::WHITE as usize].get(target_square) == 0
                        } {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ));
                        } else {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                1,
                                0,
                                0,
                                0,
                            ));
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
            if position.side == Side::WHITE as usize {
                piece_to_check = Piece::WhiteBishop as usize;
            } else {
                piece_to_check = Piece::BlackBishop as usize;
            }

            if piece == piece_to_check {
                while bitboard.0 != 0 {
                    // get least significant 1st bit index
                    source_square = bitboard.ls1b() as usize;

                    // init bishop attacks bitboard
                    attacks = Bitboard(
                        get_bishop_attacks(
                            source_square,
                            both,
                        ) & if position.side == Side::WHITE {
                            !position.occupancies[Side::WHITE as usize].0
                        } else {
                            !position.occupancies[Side::BLACK as usize].0
                        },
                    );
                    // generate bishop captures
                    while attacks.0 != 0 {
                        // get least significant 1st bit index
                        target_square = attacks.ls1b() as usize;

                        if if position.side == Side::WHITE {
                            position.occupancies[Side::BLACK as usize].get(target_square) == 0
                        } else {
                            position.occupancies[Side::WHITE as usize].get(target_square) == 0
                        } {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ));
                        } else {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                1,
                                0,
                                0,
                                0,
                            ));
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
            if position.side == Side::WHITE as usize {
                piece_to_check = Piece::WhiteRook as usize;
            } else {
                piece_to_check = Piece::BlackRook as usize;
            }
            if piece == piece_to_check {
                while bitboard.0 != 0 {
                    // get least significant 1st bit index
                    source_square = bitboard.ls1b() as usize;

                    // init rook attacks bitboard
                    let occ;
                    if position.side == Side::WHITE {
                        occ = position.occupancies[Side::WHITE as usize];
                    } else {
                        occ = position.occupancies[Side::BLACK as usize];
                    }
                    attacks = Bitboard(
                        get_rook_attacks(source_square, both)
                            & !occ.0,
                    );
                    // generate rook captures
                    while attacks.0 != 0 {
                        // get least significant 1st bit index
                        target_square = attacks.ls1b() as usize;

                        if if position.side == Side::WHITE {
                            position.occupancies[Side::BLACK as usize].get(target_square) == 0
                        } else {
                            position.occupancies[Side::WHITE as usize].get(target_square) == 0
                        } {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ));
                        } else {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                1,
                                0,
                                0,
                                0,
                            ));
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
            if position.side == Side::WHITE as usize {
                piece_to_check = Piece::WhiteQueen as usize;
            } else {
                piece_to_check = Piece::BlackQueen as usize;
            }
            if piece == piece_to_check {
                while bitboard.0 != 0 {
                    // get least significant 1st bit index
                    source_square = bitboard.ls1b() as usize;

                    // init queen attacks bitboard
                    attacks = Bitboard(
                        get_queen_attacks(source_square, both)
                            & if position.side == Side::WHITE {
                                !position.occupancies[Side::WHITE as usize].0
                            } else {
                                !position.occupancies[Side::BLACK as usize].0
                            },
                    );
                    // generate queen captures
                    while attacks.0 != 0 {
                        // get least significant 1st bit index
                        target_square = attacks.ls1b() as usize;

                        if if position.side == Side::WHITE {
                            position.occupancies[Side::BLACK as usize].get(target_square) == 0
                        } else {
                            position.occupancies[Side::WHITE as usize].get(target_square) == 0
                        } {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ));
                        } else {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                1,
                                0,
                                0,
                                0,
                            ));
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
            if position.side == Side::WHITE as usize {
                piece_to_check = Piece::WhiteKing as usize;
            } else {
                piece_to_check = Piece::BlackKing as usize;
            }
            if piece == piece_to_check {
                while bitboard.0 != 0 {
                    source_square = bitboard.ls1b() as usize;

                    // init piece attacks in order to get set of target squares
                    attacks = Bitboard(
                        KING_ATTACKS[source_square]
                            & if position.side == Side::WHITE {
                                !position.occupancies[Side::WHITE as usize].0
                            } else {
                                !position.occupancies[Side::BLACK as usize].0
                            },
                    );

                    // generate king captures
                    while attacks.0 != 0 {
                        target_square = attacks.ls1b() as usize;

                        if if position.side == Side::WHITE {
                            position.occupancies[Side::BLACK as usize].get(target_square) == 0
                        } else {
                            position.occupancies[Side::WHITE as usize].get(target_square) == 0
                        } {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ));
                        } else {
                            // add move into a move list
                            move_list.add(encode_move(
                                source_square as u8,
                                target_square as u8,
                                piece as u8,
                                0,
                                1,
                                0,
                                0,
                                0,
                            ));
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

// antichess move generation
pub fn generate_moves(position: &mut Position) -> MoveList {
    // generate pseudo-legal moves
    let mut move_list = MoveList::new();
    generate_all_moves(position, &mut move_list);
    let mut captures = MoveList::new();
    for move_ in move_list.moves.iter() {
        if capture(*move_) == 1 {
            captures.add(*move_);
        }
    }

    // if there are no captures, consider all moves
    return if captures.count == 0 {
        move_list
    } else {
        captures
    };
}

pub fn generate_captures(position: &mut Position) -> MoveList {
    // generate pseudo-legal moves
    let mut move_list = MoveList::new();
    generate_all_moves(position, &mut move_list);
    let mut captures = MoveList::new();
    for move_ in move_list.moves.iter() {
        if capture(*move_) == 1 {
            captures.add(*move_);
        }
    }

    captures
}

// evaluation function for antichess
pub fn evaluate(position: &Position) -> i16 {

    let mut score = 0;

    score += position.material_scores[1][0] - position.material_scores[0][0]; 
    // add piece square table score
    score += position.pst_scores[1][0] - position.pst_scores[0][0];
    
    // score += calculate_all(&position, phase);

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    }

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

