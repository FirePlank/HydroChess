use crate::board::position::*;
use crate::r#move::encode::*;
use crate::r#move::movegen::*;
use crate::search::OPTIONS;
use crate::variants::antichess;
use crate::variants::chess960;

use std::time::Instant;

// leaf nodes (number of positions reached during the test of the move generator at a given depth)
pub static mut NODES: u64 = 0;

// perft driver
pub fn perft_driver(position: &mut Position, depth: u32) {
    // recursion espace condition
    if depth == 0 {
        // increment leaf nodes
        unsafe {
            NODES += 1;
        }
        return;
    }

    let mut move_list = MoveList::new();
    let variant = unsafe { &OPTIONS.variant };
    match variant {
        Variant::Chess960 => chess960::generate_moves(position, &mut move_list),
        Variant::Suicide => { move_list = antichess::generate_moves(position) },
        _ => position.generate_pseudo_moves(&mut move_list)
    }

    // loop over generated moves
    for move_count in 0..move_list.count {
        // init move
        let move_ = move_list.moves[move_count as usize];

        let legal = position.make(move_);
        if !legal {
            // unmake move
            position.unmake(move_);
            continue;
        }
        // recurse
        perft_driver(position, depth - 1);
        // unmake move
        position.unmake(move_);
    }
}

// perft test
pub fn perft_test(position: &mut Position, depth: u32) {
    println!("\n    Performance test\n");

    let mut move_list = MoveList::new();
    let variant = unsafe { &OPTIONS.variant };
    match variant {
        Variant::Chess960 => chess960::generate_moves(position, &mut move_list),
        Variant::Suicide => { move_list = antichess::generate_moves(position) },
        _ => position.generate_pseudo_moves(&mut move_list)
    }

    // start timer
    let now = Instant::now();

    // loop over generated moves
    for move_count in 0..move_list.count {
        // init move
        let move_ = move_list.moves[move_count as usize];

        let legal = position.make(move_);
        if !legal {
            // unmake move
            position.unmake(move_);
            continue;
        }

        // cummulative nodes
        let commulative_nodes = unsafe { NODES };

        // recurse
        perft_driver(position, depth - 1);

        // old nodes
        let old_nodes = unsafe { NODES - commulative_nodes };

        // unmake move
        position.unmake(move_);
        // print move
        let source = source(move_);
        let target = target(move_);
        let promoted = promoted(move_);
        println!(
            "    move: {}{}{}   nodes: {}",
            SQUARE_COORDS[source as usize],
            SQUARE_COORDS[target as usize],
            PROMOTED_PIECES[promoted as usize],
            old_nodes,
        );
    }

    // print results
    println!("\n   Depth: {}", depth);
    println!("   Nodes: {}", unsafe { NODES });
    println!("    Time: {} ms", now.elapsed().as_millis());
}
