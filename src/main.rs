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
    
        // let mut position = Position::from_fen("r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1");
        // let mut searcher = Searcher::new();
        // searcher.search_position(&mut position, 100);


        
        // info score cp -32 depth 1 nodes 232 time 2 pv
        // info score cp -117 depth 2 nodes 657 time 6 pv c2b2 f7e6
        // info score cp 806 depth 3 nodes 3864 time 34 pv e4d6 e8d7 d6f7 d7e6 c2b2 
        // info score cp 806 depth 4 nodes 3905 time 35 pv e4d6 
        // info score cp 806 depth 5 nodes 8102 time 72 pv e4d6 e8d7 d6f7 d7e6 f7g5 e6f5 c2b2 b5a4 
        // info score cp 806 depth 6 nodes 11703 time 88 pv e4d6 
        // info score cp 999 depth 7 nodes 90782 time 855 pv e4d6 e8d7 d6f7 b6d5 c2b2 b5a4 d1e1 d7e6 f7h8 a8h8 
        // info score cp 999 depth 8 nodes 104731 time 906 pv e4d6 
        // info score cp 1530 depth 9 nodes 491658 time 3452 pv e4d6 e8d7 e6c5 d7d8 d6f7 d8c8 f7d6 c8b8 c2b2 b5c6 b2b6 b8c7 f3c6 e7c6 
        // info score cp 1530 depth 10 nodes 534065 time 3619 pv e4d6 
        // info score cp 1540 depth 11 nodes 2044324 time 10195 pv e4d6 e8d7 e6c5 d7d8 d6f7 d8c8 f7d6 c8b8 c2b2 b5c6 f3c6 e7c6 b2b6 b8c7 b6b7 c7d8 d2d5 
        // info score cp 1540 depth 12 nodes 2412327 time 11564 pv e4d6
        // bestmove g4g1


    } else {
        // start the main UCI loop to handle commands
        main_loop();
    }
}