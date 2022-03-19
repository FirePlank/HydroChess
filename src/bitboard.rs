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
    pub fn pop(&mut self, square: Square) {
        if self.0 & (1 << square as usize) != 0 {
            self.0 ^= 1 << square as usize;
        }
    }

    // return 1 if square is not empty, 0 otherwise
    pub fn get(&self, square: usize) -> u64 {
        if self.0 & (1 << square) == 0 { return 0; } else { return 1; }
    }
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