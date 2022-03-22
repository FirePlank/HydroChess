#[macro_use]
extern crate lazy_static;

mod board;
use board::*;

mod r#move;
use r#move::*;

mod uci;
use uci::*;

use std::time::Instant;
use std::io;
use std::io::Write;
use std::io::stdin;

// FEN debug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
// fireplank_special "r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1"

fn main() {
    init_all();

    let mut position = Position::new();

    perft_test(&mut position, 2);
    // println!("time taken: {} ms", now.elapsed().as_millis());
    // println!("nodes: {}", unsafe { NODES });
    // let mut move_list = MoveList::new();
    // position.generate_pseudo_moves(&mut move_list);
    // let mut h = String::new();

    // for move_count in 0..move_list.count {
    //     let move_ = move_list.moves[move_count as usize];
    //     let legal = position.make(move_);
    //     position.show(false);
    //     println!("Is position legal? :  {}", legal);
    //     position.unmake(move_);
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut h).unwrap();
    // }
}
