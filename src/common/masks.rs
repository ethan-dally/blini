use  crate::common::direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};
use crate::common::{bitboard::{Bitboard}, square::Square::{self}};

#[inline]
pub const fn knight_mask(sqr: Square) -> Bitboard {
    KNIGHT_MASKS[sqr as usize]
}

#[inline]
pub const fn king_mask(sqr: Square) -> Bitboard {
    KING_MASKS[sqr as usize]
}

const KNIGHT_MASKS: [Bitboard; 64] = {
    let mut masks = [Bitboard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        let sqr = Square::try_index(i).expect("unreachable");
        masks[i as usize] = gen_knight_mask(sqr);
        i += 1;
    }
    masks
};

const KING_MASKS: [Bitboard; 64] = {
    let mut masks = [Bitboard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        let sqr = Square::try_index(i).expect("unreachable");
        masks[i as usize] = gen_king_mask(sqr);
        i += 1;
    }
    masks
};

const fn gen_king_mask(sqr: Square) -> Bitboard {
    let mut mask = Bitboard::EMPTY.0;
    macro_rules! try_dir {
        ($dir:ty) => {
            if let Some(dst) = Square::shift::<$dir>(sqr, 1) {
                mask |= dst.to_bb().0;
            }
        };
    }

    try_dir!(NorthEast);
    try_dir!(SouthEast);
    try_dir!(SouthWest);
    try_dir!(NorthWest);
    try_dir!(North);
    try_dir!(East);
    try_dir!(South);
    try_dir!(West);
    Bitboard(mask)
}

const fn gen_knight_mask(sqr: Square) -> Bitboard {
    // TODO: use macro to make like the king masks
    let mut bb = Bitboard::EMPTY;
    if let Some(north) = sqr.shift::<North>(2) {
        if let Some(east) = north.shift::<East>(1) {
            bb.0 |= east.to_bb().0;
        }
        if let Some(west) = north.shift::<West>(1) {
            bb.0 |= west.to_bb().0;
        }
    }
    if let Some(south) = sqr.shift::<South>(2) {
        if let Some(east) = south.shift::<East>(1) {
            bb.0 |= east.to_bb().0;
        }
        if let Some(west) = south.shift::<West>(1) {
            bb.0 |= west.to_bb().0;
        }
    }
    if let Some(east) = sqr.shift::<East>(2) {
        if let Some(north) = east.shift::<North>(1) {
            bb.0 |= north.to_bb().0;
        }
        if let Some(south) = east.shift::<South>(1) {
            bb.0 |= south.to_bb().0;
        }
    }
    if let Some(west) = sqr.shift::<West>(2) {
        if let Some(north) = west.shift::<North>(1) {
            bb.0 |= north.to_bb().0;
        }
        if let Some(south) = west.shift::<South>(1) {
            bb.0 |= south.to_bb().0;
        }
    }
    bb
}

#[test]
pub fn knight_mask_test() {
    assert_eq!( knight_mask(Square::A1).iter().collect::<Vec<_>>(), vec![Square::C2, Square::B3]);
    assert_eq!( knight_mask(Square::H1).iter().collect::<Vec<_>>(), vec![Square::F2, Square::G3]);
    assert_eq!( knight_mask(Square::A8).iter().collect::<Vec<_>>(), vec![Square::B6, Square::C7]);
    assert_eq!( knight_mask(Square::H8).iter().collect::<Vec<_>>(), vec![Square::G6, Square::F7]);
}

#[test]
pub fn king_mask_test() {
    assert_eq!(king_mask(Square::A1), Bitboard(770));
    assert_eq!(king_mask(Square::C4), Bitboard(60298231808));
    assert_eq!(king_mask(Square::A8), Bitboard(144959613005987840));
    assert_eq!(king_mask(Square::G8), Bitboard(11592265440851656704));
    assert_eq!(king_mask(Square::H8), Bitboard(4665729213955833856));
}