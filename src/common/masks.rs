use crate::common::{
    bitboard::Bitboard,
    direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West},
    square::Square::{self},
};

#[inline]
pub const fn knight_mask(sqr: Square) -> Bitboard {
    KNIGHT_MASKS[sqr as usize]
}

#[inline]
pub const fn king_mask(sqr: Square) -> Bitboard {
    KING_MASKS[sqr as usize]
}

#[inline]
///NOTE: includes sqr2 in the between calc but not sqr1
pub const fn between_mask(sqr1: Square, sqr2: Square) -> Bitboard {
    BETWEEN_MASKS[sqr1 as usize * 64 + sqr2 as usize]
}

static BETWEEN_MASKS: [Bitboard; 4096] = {
    macro_rules! fill_ray {
        ($masks:ident, $src:ident, $dir:ty) => {
            let mut step_mask: u64 = 0;
            let mut step: u8 = 1;
            while let Some(sqr2) = $src.shift::<$dir>(step) {
                step_mask |= sqr2.to_bb().0;
                $masks[$src as usize * 64 + sqr2 as usize] = Bitboard(step_mask);
                step += 1;
            }
        };
    }

    let mut masks = [Bitboard::EMPTY; 4096];
    let mut sqr1_idx = 0;
    while sqr1_idx < 64 {
        let sqr1 = Square::try_index(sqr1_idx).expect("unreachable");
        fill_ray!(masks, sqr1, North);
        fill_ray!(masks, sqr1, East);
        fill_ray!(masks, sqr1, South);
        fill_ray!(masks, sqr1, West);
        fill_ray!(masks, sqr1, NorthEast);
        fill_ray!(masks, sqr1, SouthEast);
        fill_ray!(masks, sqr1, SouthWest);
        fill_ray!(masks, sqr1, NorthWest);
        sqr1_idx += 1;
    }
    masks
};

static KNIGHT_MASKS: [Bitboard; 64] = {
    let mut masks = [Bitboard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        let sqr = Square::try_index(i).expect("unreachable");
        masks[i as usize] = gen_knight_mask(sqr);
        i += 1;
    }
    masks
};

static KING_MASKS: [Bitboard; 64] = {
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
fn knight_mask_test() {
    assert_eq!(
        knight_mask(Square::A1).iter().collect::<Vec<_>>(),
        vec![Square::C2, Square::B3]
    );
    assert_eq!(
        knight_mask(Square::H1).iter().collect::<Vec<_>>(),
        vec![Square::F2, Square::G3]
    );
    assert_eq!(
        knight_mask(Square::A8).iter().collect::<Vec<_>>(),
        vec![Square::B6, Square::C7]
    );
    assert_eq!(
        knight_mask(Square::H8).iter().collect::<Vec<_>>(),
        vec![Square::G6, Square::F7]
    );
}

#[test]
fn king_mask_test() {
    assert_eq!(king_mask(Square::A1), Bitboard(770));
    assert_eq!(king_mask(Square::C4), Bitboard(60298231808));
    assert_eq!(king_mask(Square::A8), Bitboard(144959613005987840));
    assert_eq!(king_mask(Square::G8), Bitboard(11592265440851656704));
    assert_eq!(king_mask(Square::H8), Bitboard(4665729213955833856));
}

#[test]
fn between_mask_test() {
    macro_rules! sqrs_to_bb {
        ($($sqr:expr),*) => {{
            let mut bb = Bitboard::EMPTY;

            $(
                bb |= $sqr.to_bb();
            )*

            bb
        }};
    }
    assert_eq!(
        between_mask(Square::A1, Square::A2),
        sqrs_to_bb!(Square::A2)
    );
    assert_eq!(
        between_mask(Square::A1, Square::B1),
        sqrs_to_bb!(Square::B1)
    );
    assert_eq!(
        between_mask(Square::A1, Square::B2),
        sqrs_to_bb!(Square::B2)
    );
    assert_eq!(
        between_mask(Square::A1, Square::D1),
        sqrs_to_bb!(Square::B1, Square::C1, Square::D1)
    );
    assert_eq!(
        between_mask(Square::H1, Square::E1),
        sqrs_to_bb!(Square::G1, Square::F1, Square::E1)
    );
    assert_eq!(
        between_mask(Square::A1, Square::A4),
        sqrs_to_bb!(Square::A2, Square::A3, Square::A4)
    );
    assert_eq!(
        between_mask(Square::H8, Square::H5),
        sqrs_to_bb!(Square::H7, Square::H6, Square::H5)
    );
    assert_eq!(
        between_mask(Square::A1, Square::D4),
        sqrs_to_bb!(Square::B2, Square::C3, Square::D4)
    );
    assert_eq!(
        between_mask(Square::H1, Square::E4),
        sqrs_to_bb!(Square::G2, Square::F3, Square::E4)
    );
    assert_eq!(
        between_mask(Square::H8, Square::E5),
        sqrs_to_bb!(Square::G7, Square::F6, Square::E5)
    );
    assert_eq!(
        between_mask(Square::A8, Square::D5),
        sqrs_to_bb!(Square::B7, Square::C6, Square::D5)
    );
    assert_eq!(
        between_mask(Square::A1, Square::H1),
        sqrs_to_bb!(
            Square::B1,
            Square::C1,
            Square::D1,
            Square::E1,
            Square::F1,
            Square::G1,
            Square::H1
        )
    );
    assert_eq!(
        between_mask(Square::A1, Square::A8),
        sqrs_to_bb!(
            Square::A2,
            Square::A3,
            Square::A4,
            Square::A5,
            Square::A6,
            Square::A7,
            Square::A8
        )
    );
    assert_eq!(between_mask(Square::A1, Square::C2), Bitboard::EMPTY);
    assert_eq!(between_mask(Square::B2, Square::E3), Bitboard::EMPTY);
    assert_eq!(between_mask(Square::A1, Square::H2), Bitboard::EMPTY);
    assert_eq!(between_mask(Square::D4, Square::D4), Bitboard::EMPTY);
}
