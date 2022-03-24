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

use std::time::Instant;

// FEN debug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
// fireplank_special "r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1"

// fn position_max_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
//     slice.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx)
// }
fn main() {
    init_all();

    // debug mode variable
    let debug = true;
    if debug {
        // cmk = 2162211   tricky = 760453
        let mut position = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        //create move list instance
        let mut move_list = MoveList::new();
        // generate pseudo-legal moves
        position.generate_pseudo_moves(&mut move_list);

        // position.show(false);
        let mut searcher = Searcher::new();
        searcher.search_position(&mut position, 5);
        // searcher.killers[0][0] = move_list.moves[3];
        // let a = searcher.sort_moves(&position, move_list);
        // println!("{:?}", a);
        // searcher.search_position(&mut position, 6);
        // loop over generated moves
        // let now = Instant::now();
        // let a = sort_moves(&position, move_list);
        // Move(a[0].1).show();
        //println!("{:?}", now.elapsed());
    } else {
        // start the main UCI loop to handle commands
        main_loop();
    }
}