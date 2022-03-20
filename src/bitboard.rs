#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

// implementing all the bitboard functions
impl Bitboard {
    pub fn show(&self) {
        // loop over board ranks
        for rank in 0..8 {
            // loop over board files
            for file in 0..8 {
                // convert file and rank into square index
                let square = rank * 8 + file;
                // print ranks
                if file == 0 {
                    print!("{}  ", 8 - rank);
                }
                // print bit state (1 or 0)
                print!("{} ", self.get(square));
            }
            // print newline every rank
            println!();
        }
        // print board files
        println!("   a b c d e f g h");
        // print bitboard as unsigned decimal
        println!("\n   Bitboard: {}\n", self.0.to_string());
    }

    // set bit at given square
    pub fn set(&mut self, square: usize) {
        self.0 |= 1 << square;
    }

    // remove bit at given square, check if square is already set to 0
    pub fn pop(&mut self, square: usize) {
        if self.0 & (1u64.wrapping_shl(square as u32)) != 0 {
            self.0 ^= 1 << square;
        }
    }

    // return 1 if square is not empty, 0 otherwise
    pub fn get(&self, square: usize) -> u64 {
        if self.0 & (1 << square) == 0 { return 0; } else { return 1; }
    }

    // count one bits in bitboard
    pub fn count(&self) -> u32 {
        return self.0.count_ones(); // rust makes this really easy lol, thanks rust <3
    }

    // get least significant 1st bit index
    pub fn ls1b(&self) -> isize {
        if self.0 != 0 {
            return ((self.0 as i64 & -(self.0 as i64))-1).count_ones() as isize;
        } else {
            // illegal index
            return -1;
        }
    }
}

pub fn set_occupancy(index: i32, bits_in_mask: u32, attack_mask: &mut Bitboard) -> Bitboard {
    let mut occupancy = Bitboard(0); // occupancy map

    // loop over the range of bits within attack mask
    for count in 0..bits_in_mask {
        // get LS1B index of attack mask
        let square = attack_mask.ls1b();
        // pop LS1B in attack map
        attack_mask.pop(square as usize);
        // make sure occupancy is on board
        if index & (1 << count) != 0 {
            // populate occupancy map
            occupancy.0 |= 1u64.wrapping_shl(square as u32);
        }
    }

    // return occupancy map
    return occupancy;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}
// square string list
#[allow(dead_code)]
pub const SQUARE_COORDS: [&str;64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];