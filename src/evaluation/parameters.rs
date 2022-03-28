// evaluation parameters and constants
pub const PIECE_VALUE: [i16; 6] = [95, 370, 390, 590, 1100, 10000];
pub const PIECE_VALUE_EG: [i16; 6] = [115, 310, 270, 680, 1300, 10000];
pub const BISHOP_PAIR: i16 = 40;

pub const MOBILITY_OPENING: i16 = 5;
pub const MOBILITY_ENDING: i16 = 1;
pub const MOBILITY_CENTER_MULTIPLIER: i16 = 2;

pub const DOUBLED_PAWN_OPENING: i16 = -6;
pub const DOUBLED_PAWN_ENDING: i16 = -16;

pub const ISOLATED_PAWN_OPENING: i16 = -29;
pub const ISOLATED_PAWN_ENDING: i16 = -2;

pub const PASSED_PAWN_OPENING: i16 = 2;
pub const PASSED_PAWN_ENDING: i16 = 54;

pub const PAWN_SHIELD_OPENING: i16 = 12;
pub const PAWN_SHIELD_ENDING: i16 = 5;

pub const PAWN_SHIELD_OPEN_FILE_OPENING: i16 = -30;
pub const PAWN_SHIELD_OPEN_FILE_ENDING: i16 = 2;

pub const KING_ATTACKED_FIELDS_OPENING: i16 = -20;
pub const KING_ATTACKED_FIELDS_ENDING: i16 = 0;


// extract rank from square [square]
#[rustfmt::skip]
pub const GET_RANK: [u8;64] = [
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
];

// Piece square tables
#[rustfmt::skip]
pub const PSQT: [[i16;64];6] = [
    // pawn
    [
        000, 000, 000, 000, 000, 000, 000, 000,
        150, 120, 120, 130, 130, 120, 120, 150,
        090, 060, 060, 070, 070, 060, 050, 090,
        012, 010, 015, 035, 032, -05, 005, 012,
        004, 003, 011, 020, 020, 008, -08, 003,
        005, 015, -02, 008, 008, -06, 013, 005,
        004, 005, 007, -09, -09, 010, 006, 004,
        000, 000, 000, 000, 000, 000, 000, 000,
    ],
    // knight
    [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20, 000, 000, 000, 000, -20, -40,
        -30, 000, 010, 015, 015, 010, 000, -30,
        -30, 000, 015, 020, 020, 015, 000, -30,
        -30, 000, 015, 017, 017, 015, 000, -30,
        -30, -20, 006, 015, 015, 010, -20, -30,
        -40, -20, 000, 000, 000, 000, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ],
    // bishop
    [
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10, 000, 000, 000, 000, 000, 000, -10,
        -10, 000, 005, 010, 010, 005, 000, -10,
        -10, 017, 005, 012, 012, 005, 017, -10,
        -10, 000, 015, 012, 012, 015, 000, -10,
        -10, 010, 010, 010, 010, 010, 010, -10,
        -10, 016, 000, 000, 000, 000, 016, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // rook
    [
        000, 000, 000, 003, 003, 000, 000, 000,
        005, 010, 014, 014, 014, 014, 010, 005,
        -05, 000, 000, 000, 000, 000, 000, -05,
        -05, 000, 000, 000, 000, 000, 000, -05,
        -05, 000, 000, 000, 000, 000, 000, 002,
        001, 002, 000, 000, 000, 000, 004, 001,
        -05, 000, 000, 000, 000, 000, 000, -05,
        000, 000, 000, 007, 008, 003, 002, 000,
   ],
   // queen
   [
        -20, -10, -10, -03, -03, -10, -10, -20,
        -10, 000, 000, 000, 000, 000, 000, -10,
        -10, 000, 005, 010, 010, 005, 000, -10,
        -01, 005, 005, 010, 010, 005, 005, -02,
        002, 000, 010, 010, 010, 010, 000, 000,
        -10, 010, 012, 010, 010, 010, 010, -10,
        -10, 005, 000, 000, 000, 000, 005, -10,
        -20, -10, -10, -01, -03, -10, -10, -20,
    ],
    // king
    [
        -10, 000, -05, -10, -10, -05, 000, -10,
        -03, 000, -05, -10, -10, -05, 000, -03,
        -05, 000, -10, -20, -20, -10, 000, -05,
        -05, -05, -10, -20, -20, -10, -05, -05,
        -05, -05, -10, -20, -20, -10, -05, -05,
        -05, -05, -10, -20, -20, -10, -05, -05,
        -05, -05, -10, -10, -08, -04, -05, -05,
        001, 012, 010, 000, 000, 004, 011, 003,
    ],
];

// endgame piece square tables
#[rustfmt::skip]
pub const PSQT_EG: [[i16;64];6] = [
    // pawn
    [
        000, 000, 000, 000, 000, 000, 000, 000,
        250, 330, 350, 380, 380, 350, 330, 250,
        090, 110, 120, 135, 135, 120, 110, 090,
        050, 040, 045, 050, 050, 045, 040, 050,
        020, 020, 015, 020, 020, 015, 020, 020,
        005, 005, 005, 010, 010, 005, 005, 005,
        -20, -20, -10, -05, -05, -10, -20, -20,
        000, 000, 000, 000, 000, 000, 000, 000,
    ],
    // knight
    [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20, 000, 000, 000, 000, -20, -40,
        020, 050, 050, 055, 055, 050, 050, 020,
        000, 010, 015, 020, 020, 015, 010, 000,
        -30, 000, 015, 017, 017, 015, 000, -30,
        -30, -20, 000, 015, 015, 000, -20, -30,
        -40, -20, 000, 000, 000, 000, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ],
    // bishop
    [
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10, 000, 000, 000, 000, 000, 000, -10,
        -10, 000, 005, 010, 010, 005, 000, -10,
        -10, 005, 005, 010, 010, 005, 005, -10,
        -10, 000, 015, 010, 010, 015, 000, -10,
        -10, 010, 010, 010, 010, 010, 010, -10,
        -10, 015, 000, 000, 000, 000, 015, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // rook
    [
        020, 020, 020, 020, 020, 020, 020, 020,
        005, 010, 020, 030, 030, 020, 010, 005,
        -05, 000, 000, 010, 010, 000, 000, -05,
        -05, 000, 000, 000, 000, 000, 000, -05,
        -05, 000, 000, 000, 000, 000, 000, 002,
        001, 002, 000, 000, 000, 000, 004, 001,
        -05, 000, 000, 000, 000, 000, 000, -05,
        000, 000, 000, 000, 000, 000, 000, 000,
    ],
    // queen
    [
        -20, -10, -10, -03, -03, -10, -10, -20,
        -10, 000, 000, 000, 000, 000, 000, -10,
        -10, 000, 005, 010, 010, 005, 000, -10,
        -01, 005, 005, 010, 010, 005, 005, -02,
        002, 000, 010, 010, 010, 010, 000, 000,
        -10, 010, 012, 010, 010, 010, 010, -10,
        -10, 005, 000, 000, 000, 000, 005, -10,
        -20, -10, -10, -01, -03, -10, -10, -20,
    ],
    // king
    [
        000, 000, -05, -10, -10, -05, 000, 000,
        -10, 030, -05, 000, 000, -05, 030, -10,
        -20, 000, 020, 030, 030, 020, 000, -20,
        -30, -05, 030, 040, 040, 030, -05, -30,
        -30, -05, 030, 040, 040, 030, -05, -30,
        -30, -05, 020, 030, 030, 020, -05, -30,
        -30, -25, -20, -05, -05, -20, -25, -30,
        -50, -40, -40, -30, -30, -40, -40, -50,
    ],
];