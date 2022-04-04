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

use std::mem::MaybeUninit;

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
        // let searcher = Searcher::new();
        // let now = std::time::Instant::now();
        // for _ in 0..10000 {
        //     let mut moves: [u32; 127] = unsafe { MaybeUninit::uninit().assume_init() };
        // }
        // println!("{:?}", now.elapsed().as_nanos());
        // let now = std::time::Instant::now();
        // for _ in 0..10000 {
        //     let mut moves: [u32; 127] = [0; 127];
        //     moves;
        // }
        // println!("{:?}", now.elapsed().as_nanos());
        
        // info string time: -1 start: 1648759755832 stop: 0 depth: 20 timeset: false factor: 0.75
        // info score cp 141 depth 1 nodes 2 time 0 pv
        // info score cp 24 depth 2 nodes 214 time 1 pv b1c3 g8f6
        // info score cp 21 depth 3 nodes 508 time 1 pv b1c3 b8c6 g1f3
        // info score cp 33 depth 4 nodes 1418 time 1 pv d2d4 g8f6 g1f3 b8c6
        // info score cp 35 depth 5 nodes 3226 time 2 pv d2d4 b8a6 g1f3 g8f6 b1c3
        // info score cp 41 depth 6 nodes 11251 time 6 pv d2d4 e7e6 b1c3 b8c6 g1f3 g8f6
        // info score cp 41 depth 7 nodes 11266 time 10 pv d2d4
        // info score cp 40 depth 8 nodes 11273 time 20 pv d2d4
        // info score cp 37 depth 9 nodes 12298 time 57 pv d2d4
        // info score cp 31 depth 10 nodes 54772 time 140 pv e2e4 b8c6 g1f3 g8f6 b1c3 d7d5 e4d5 f6d5 c3d5
        // info score cp 37 depth 11 nodes 111494 time 213 pv e2e4 b8c6 g1f3 g8f6 b1c3 e7e5 d2d4 f8b4 d4d5 b4c3
        // info score cp 37 depth 12 nodes 293653 time 647 pv e2e4 b8c6 d2d4 g8f6 d4d5 c6b4 b1c3 d7d6 g1f3 e7e5 f1b5
        // info score cp 35 depth 13 nodes 595064 time 1729 pv e2e4 e7e5 b1c3 b8c6 g1f3 g8f6 d2d4 e5d4 f3d4 f8b4 c1g5 c6d4
        // info score cp 25 depth 14 nodes 1050863 time 3521 pv e2e4 e7e5 b1c3 b8c6 g1f3 g8f6 f1b5 f8b4 d2d3 d7d6 b5c6 b7c6 e1g1 c8b7
        // info score cp 34 depth 15 nodes 3991784 time 8513 pv e2e4 e7e5 g1f3 g8f6 b1c3 b8c6 d2d4 e5d4 f3d4 f8c5 d4c6 d7c6 e4e5 d8d1

        // info string time: -1 start: 1648920631724 stop: 0 depth: 20 timeset: false factor: 0.75
        // info score cp 141 depth 1 nodes 2 time 0 pv
        // info score cp 24 depth 2 nodes 411 time 1 pv b1c3 g8f6
        // info score cp 21 depth 3 nodes 970 time 2 pv b1c3 b8c6 g1f3
        // info score cp 29 depth 4 nodes 2703 time 3 pv d2d4 g8f6 b1c3 b8c6
        // info score cp 33 depth 5 nodes 6478 time 5 pv d2d4 g8f6 b1c3 d7d5 g1f3
        // info score cp 40 depth 6 nodes 15444 time 8 pv d2d4 g8f6 g1f3 b8c6 b1c3 d7d5
        // info score cp 38 depth 7 nodes 20920 time 15 pv d2d4 g8f6 g1f3 b8c6 b1d2 d7d5
        // info score cp 30 depth 8 nodes 70594 time 54 pv e2e4 b8c6 g1f3 g8f6 b1c3 a7a6 d2d4
        // info score cp 26 depth 9 nodes 100240 time 120 pv e2e4 b8c6 g1f3 g8f6 e4e5 f6d5 d2d4 d7d6
        // info score cp 36 depth 10 nodes 194518 time 195 pv e2e4 b8c6 g1f3 g8f6 b1c3 e7e5 d2d4 c6d4 f3d4
        // info score cp 41 depth 11 nodes 437291 time 443 pv e2e4 b8c6 d2d4 d7d5 e4d5 d8d5 g1f3 g8f6 b1c3 d5e6
        // info score cp 44 depth 12 nodes 1014350 time 913 pv e2e4 b8c6
        // info score cp 43 depth 13 nodes 2073615 time 1849 pv e2e4 b8c6 d2d4 d7d5 e4d5 d8d5 g1f3 g8f6 b1c3 d5d6 f1b5 c8g4
        // info score cp 31 depth 14 nodes 3805883 time 11461 pv e2e4 b8c6 d2d4 d7d5 e4d5 d8d5
        // info score cp 28 depth 15 nodes 9250236 time 17623 pv e2e4 e7e5 b1c3 g8f6 g1f3 b8c6 d2d4 e5d4 f3d4 f8b4 d4c6 d7c6 d1f3 c6c5

        // info string time: -1 start: 1648925738216 stop: 0 depth: 127 timeset: false factor: 0.75
        // info score cp 141 depth 1 nodes 2 time 1 pv
        // info score cp 24 depth 2 nodes 411 time 1 pv b1c3 g8f6
        // info score cp 21 depth 3 nodes 970 time 2 pv b1c3 b8c6 g1f3
        // info score cp 29 depth 4 nodes 2631 time 2 pv d2d4 g8f6 b1c3 b8c6
        // info score cp 33 depth 5 nodes 6211 time 3 pv d2d4 g8f6 b1c3 d7d5 g1f3
        // info score cp 40 depth 6 nodes 14785 time 6 pv d2d4 g8f6 g1f3 b8c6 b1c3 d7d5
        // info score cp 36 depth 7 nodes 20193 time 13 pv d2d4 g8f6 g1f3 b8c6 b1d2 d7d5
        // info score cp 30 depth 8 nodes 57693 time 41 pv e2e4 g8f6 b1c3 b8c6 g1f3 a7a6 d2d4
        // info score cp 31 depth 9 nodes 103918 time 73 pv e2e4 b8c6 b1c3 g8f6 g1f3 a7a6 d2d4 d7d5
        // info score cp 36 depth 10 nodes 176685 time 154 pv e2e4 b8c6 b1c3 e7e5 g1f3 g8f6 f1c4 a7a6 d2d4
        // info score cp 36 depth 11 nodes 297939 time 315 pv e2e4 b8c6
        // info score cp 35 depth 12 nodes 796209 time 854 pv e2e4 e7e5 b1c3 b8c6 g1f3 f8b4 c3d5 g8f6 d5b4 c6b4 f3e5
        // info score cp 37 depth 13 nodes 1353252 time 1377 pv e2e4 e7e5 b1c3 b8c6 g1f3 g8f6 d2d4 e5d4 f3d4 c6d4 d1d4 d7d6
        // info score cp 23 depth 14 nodes 3693734 time 8746 pv e2e4 e7e5 b1c3 b8c6 g1f3 g8f6 f1b5 c6d4 b5c4 d4f3 d1f3 d7d6 d2d3 c8g4
        // info score cp 24 depth 15 nodes 5990519 time 21730 pv e2e4 e7e5 b1c3 g8f6 g1f3 b8c6 d2d4 e5d4 f3d4 f8c5 d4c6 d7c6 d1d8 e8d8 g2g3

    } else {
        // start the main UCI loop to handle commands
        main_loop();
    }
}