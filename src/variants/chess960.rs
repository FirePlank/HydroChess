use crate::{board::{position::Position, Bitboard}, r#move::{movegen::MoveList, encode::capture}};
use crate::board::attacks::*;
use crate::board::position::*;
use crate::r#move::encode::*;
use crate::evaluation::*;

pub fn generate_moves(position: &mut Position, move_list: &mut MoveList) {
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
            // generate white pawns & white king castling moves
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
                
                // castling moves
                if piece == Piece::WhiteKing as usize {
                    // queen castling is available
                    if (position.castle & Castling::WQ as u8) != 0 {
                        // get position of rook
                        let left_rook_square = position.bitboards[Piece::WhiteRook as usize].ls1b();
                        // get king square
                        let king_square = position.bitboards[Piece::WhiteKing as usize].ls1b();

                        let check_square;
                        if left_rook_square > Square::C1 as isize {
                            check_square = Square::C1 as isize;
                        } else {
                            check_square = left_rook_square;
                        }

                        // check if we can castle, every square between the king and the rook must be empty
                        let mut can_castle = true;
                        if king_square == Square::C1 as isize {
                            // handle special case where king is on c1
                            if position.occupancies[Side::WHITE as usize].get(Square::D1 as usize) != 0 {
                                can_castle = false;
                            }
                        } 
                        if king_square == Square::B1 as isize {
                            if position.occupancies[Side::WHITE as usize].get(Square::D1 as usize) != 0
                                || position.occupancies[Side::WHITE as usize].get(Square::C1 as usize) != 0
                            {
                                can_castle = false;
                            }
                        } else {
                            for square in check_square..king_square {
                                if position.is_attacked(square as usize, Side::BLACK as usize) {
                                    can_castle = false;
                                    break;
                                }

                                if square == left_rook_square || square == king_square {
                                    continue;
                                }

                                if position.occupancies[Side::WHITE as usize].get(square as usize) != 0 {
                                    can_castle = false;
                                    break;
                                }
                            }
                        }

                        if can_castle {
                            // add move into a move list
                            move_list.add(encode_move(
                                king_square as u8,
                                Square::C1 as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                1,
                            ));
                        }
                    }

                    // king castling is available
                    if (position.castle & Castling::WK as u8) != 0 {
                        // get position of rook
                        let left_rook_square = position.bitboards[Piece::WhiteRook as usize].ls1b();
                        // pop left rook from bitboard
                        position.bitboards[Piece::WhiteRook as usize].pop(left_rook_square as usize);
                        let right_rook_square = position.bitboards[Piece::WhiteRook as usize].ls1b();
                        // push left rook back into bitboard
                        position.bitboards[Piece::WhiteRook as usize].set(left_rook_square as usize);
                        // get king square
                        let king_square = position.bitboards[Piece::WhiteKing as usize].ls1b();

                        // check if we can castle, every square between the king and the rook must be empty
                        let mut can_castle = true;
                        if king_square == Square::G1 as isize {
                            // handle edge case where king is farther than F1 meaning we need to check if F1 is empty
                            if position.occupancies[Side::WHITE as usize].get(Square::F1 as usize) != 0 {
                                can_castle = false;
                            }
                        } else {
                            for square in king_square..Square::H1 as isize {
                                if position.is_attacked(square as usize, Side::BLACK as usize) {
                                    can_castle = false;
                                    break;
                                }

                                if square as isize == right_rook_square || square as isize == king_square {
                                    continue;
                                }

                                if position.occupancies[Side::WHITE as usize].get(square as usize) != 0 {
                                    can_castle = false;
                                    break;
                                }
                            }
                        }
                        // make sure squares between king and rook are empty
                        if can_castle {
                            // add move into a move list
                            move_list.add(encode_move(
                                king_square as u8,
                                Square::G1 as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                1,
                            ));
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

                // generate castling moves
                if piece == Piece::BlackKing as usize {
                    // king position.side castling is available
                    if (position.castle & Castling::BK as u8) != 0 {
                        let left_rook_square = position.bitboards[Piece::BlackRook as usize].ls1b();
                        // pop left rook from bitboard
                        position.bitboards[Piece::BlackRook as usize].pop(left_rook_square as usize);
                        let right_rook_square = position.bitboards[Piece::BlackRook as usize].ls1b();
                        // push left rook back into bitboard
                        position.bitboards[Piece::BlackRook as usize].set(left_rook_square as usize);
                        // get king square
                        let king_square = position.bitboards[Piece::BlackKing as usize].ls1b();

                        // check if we can castle, every square between the king and the rook must be empty
                        let mut can_castle = true;
                        if king_square == Square::G8 as isize {
                            // handle edge case 
                            if position.occupancies[Side::BLACK as usize].get(Square::F8 as usize) != 0 {
                                can_castle = false;
                            }
                        } else {
                            for square in king_square..Square::H8 as isize {
                                if position.is_attacked(square as usize, Side::WHITE as usize) {
                                    can_castle = false;
                                    break;
                                }

                                if square == right_rook_square || square == king_square {
                                    continue;
                                }

                                if position.occupancies[Side::BLACK as usize].get(square as usize) != 0 {
                                    can_castle = false;
                                    break;
                                }
                            }
                        }

                        if can_castle {
                            // add move into a move list
                            move_list.add(encode_move(
                                king_square as u8,
                                Square::G8 as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                1,
                            ));
                        }
                    }

                    // queen position.side castling is available
                    if (position.castle & Castling::BQ as u8) != 0 {
                        let left_rook_square = position.bitboards[Piece::BlackRook as usize].ls1b();
                        // get king square
                        let king_square = position.bitboards[Piece::BlackKing as usize].ls1b();

                        let check_square;
                        if left_rook_square > Square::C1 as isize {
                            check_square = Square::C1 as isize;
                        } else {
                            check_square = left_rook_square;
                        }

                        // check if we can castle, every square between the king and the rook must be empty
                        let mut can_castle = true;
                        if king_square == Square::C8 as isize {
                            // handle special case where king is on c8
                            if position.occupancies[Side::BLACK as usize].get(Square::D8 as usize) != 0 {
                                can_castle = false;
                            }
                        } 
                        if king_square == Square::B8 as isize {
                            if position.occupancies[Side::BLACK as usize].get(Square::D8 as usize) != 0
                                || position.occupancies[Side::BLACK as usize].get(Square::C8 as usize) != 0
                            {
                                can_castle = false;
                            }
                        } else {
                            for square in check_square..king_square {
                                if position.is_attacked(square as usize, Side::WHITE as usize) {
                                    can_castle = false;
                                    break;
                                }

                                if square == left_rook_square || square == king_square {
                                    continue;
                                }

                                if position.occupancies[Side::BLACK as usize].get(square as usize) != 0 {
                                    can_castle = false;
                                    break;
                                }
                            }
                        }

                        if can_castle {
                            // add move into a move list
                            move_list.add(encode_move(
                                king_square as u8,
                                Square::C8 as u8,
                                piece as u8,
                                0,
                                0,
                                0,
                                0,
                                1,
                            ));
                        }
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