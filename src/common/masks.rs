use  crate::common::direction::{East, North, South, West};
use crate::common::{bitboard::{Bitboard}, square::Square::{self}};

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

#[inline]
pub fn knight_mask(sqr: Square) -> Bitboard {
    KNIGHT_MASKS[sqr as usize]
}

const fn gen_knight_mask(sqr: Square) -> Bitboard {
    // TODO: find const way to make this nicer
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
    assert_eq!( knight_mask(Square::D4).iter().collect::<Vec<_>>(), vec![
            Square::C2, 
            Square::E2, 
            Square::B3, 
            Square::F3, 
            Square::B5, 
            Square::F5, 
            Square::C6, 
            Square::E6
        ]
    );
}