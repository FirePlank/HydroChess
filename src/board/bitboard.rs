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
            return ((self.0 as i64 & (self.0 as i64).wrapping_neg()).wrapping_sub(1)).count_ones() as isize;
        } else {
            // illegal index
            return -1;
        }
    }
}

pub fn set_occupancy(index: i32, bits_in_mask: u32, mut attack_mask: Bitboard) -> Bitboard {
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