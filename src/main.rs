#[macro_use]
extern crate lazy_static;

mod board;
use board::*;

mod r#move;
use r#move::*;

mod uci;
use uci::*;

mod search;
use search::*;

mod evaluation;
use evaluation::*;

mod cache;
use cache::*;
// use std::thread;

// FEN debug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
// fireplank_special "r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1"

// use std::io;
// use std::io::Write;

fn main() {
    init_all();

    // debug mode variable
    let debug = false;
    if debug {
        
        let mut position = Position::from_fen("3k4/p6p/p6p/7N/8/P7/P7/3K4 w - - 0 1");
        // let mut searcher = Searcher::new();
        // searcher.search_position(&mut position, 100);
        println!("{}", evaluate(&position));
        
        // info score cp 95 depth 1 nodes 2 time 0 pv
        // info score cp 0 depth 2 nodes 317 time 0 pv g1f3 g8f6
        // info score cp 50 depth 3 nodes 427 time 1 pv g1f3 b8c6 b1c3
        // info score cp 0 depth 4 nodes 1786 time 2 pv b1c3 g8f6 g1f3 b8c6
        // info score cp 29 depth 5 nodes 2947 time 3 pv b1c3 b8c6 g1f3 g8f6 d2d4
        // info score cp 0 depth 6 nodes 6939 time 6 pv b1c3 b8c6 g1f3 g8f6 e2e4 d7d5
        // info score cp 27 depth 7 nodes 14042 time 9 pv b1c3 b8c6 g1f3 d7d5 d2d4 g8f6 c1g5
        // info score cp 17 depth 8 nodes 57589 time 35 pv e2e4 b8c6 g1f3 g8f6 e4e5 f6d5 b1c3 e7e6
        // info score cp 22 depth 9 nodes 103781 time 61 pv e2e4 b8c6 g1f3 g8f6 e4e5 f6g4 d2d4 d7d6 f1b5
        // info score cp 12 depth 10 nodes 501930 time 262 pv b1c3 b8c6 d2d4 d7d5 g1f3 g8f6 c1g5 e7e6 e2e4 f8b4
        // bestmove b1c3


    } else {
        // start the main UCI loop to handle commands
        main_loop();
    }
}