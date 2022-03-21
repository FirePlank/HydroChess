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

// FEN dedug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"


fn main() {
    init_all();
    
    let pos = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1");
    pos.show(true);
    generate_moves(&pos, Side::BLACK as usize);
    
    
    // pos.occupancies[Side::BOTH as usize].show();
    // println!("is square attacked A2? {}", pos.is_attacked(Square::A2 as usize, Side::WHITE));
    // pos.show_attacked(Side::WHITE as usize);
}
