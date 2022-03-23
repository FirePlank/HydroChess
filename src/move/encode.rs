// encode move
pub fn encode_move(
    source: u8,
    target: u8,
    piece: u8,
    promoted: u8,
    capture: u8,
    double: u8,
    enpassant: u8,
    castling: u8,
) -> u32 {
    return (source as u32)
        | ((target as u32) << 6)
        | ((piece as u32) << 12)
        | ((promoted as u32) << 16)
        | ((capture as u32) << 20)
        | ((double as u32) << 21)
        | ((enpassant as u32) << 22)
        | ((castling as u32) << 23);
}

// extract source square
pub fn source(move_: u32) -> u8 {
    return (move_ & 0x3f) as u8;
}

// extract target square
pub fn target(move_: u32) -> u8 {
    return ((move_ & 0xfc0) >> 6) as u8;
}

// extract piece
pub fn get_piece(move_: u32) -> u8 {
    return ((move_ & 0xf000) >> 12) as u8;
}

// extract promoted piece
pub fn promoted(move_: u32) -> u8 {
    return ((move_ & 0xf0000) >> 16) as u8;
}

// extract capture flag
pub fn capture(move_: u32) -> u8 {
    return ((move_ >> 20) & 0x1) as u8;
}

// extract double pawn push flag
pub fn double(move_: u32) -> u8 {
    return ((move_ >> 21) & 0x1) as u8;
}

// extract enpassant flag
pub fn enpassant(move_: u32) -> u8 {
    return ((move_ >> 22) & 0x1) as u8;
}

// extract castling flag
pub fn castling(move_: u32) -> u8 {
    return ((move_ >> 23) & 0x1) as u8;
}
