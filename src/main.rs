#[macro_use]
extern crate lazy_static;

mod board;
use board::*;

mod r#move;
use r#move::*;

// use std::time::Instant;
// use std::io;
// use std::io::Write;
// use std::io::stdin;

// FEN debug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
// fireplank_special "r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1"

fn main() {
    init_all();
    
    let move_ = encode_move(Square::E2 as u8, Square::E4 as u8, Piece::WhitePawn as u8, Piece::WhiteQueen as u8, 1, 1, 1, 1);
    
    let source_square = source(move_);
    let target_square = target(move_);
    let piece = piece(move_);
    let promoted = promoted(move_);
    let capture = capture(move_);
    let double = double(move_);
    let enpassant = enpassant(move_);
    let castling = castling(move_);

    println!("source square: {}", SQUARE_COORDS[source_square as usize]);
    println!("target square: {}", SQUARE_COORDS[target_square as usize]);
    println!("piece: {}", ASCII_PIECES[piece as usize]);
    println!("promoted: {}", ASCII_PIECES[promoted as usize]);
    println!("capture: {}", capture);
    println!("double: {}", double);
    println!("enpassant: {}", enpassant);
    println!("castling: {}", castling);
}
