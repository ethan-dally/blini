use std::fmt;

use enum_map::Enum;

use crate::common::{
    bitboard::Bitboard,
    colour::Colour,
    direction::Direction,
    file::File,
    rank::Rank,
};

#[repr(u8)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    #[inline]
    pub const fn new(rank: Rank, file: File) -> Square {
        // 8 * 8 + 8 < 255
        let index = rank as u8 * 8 + file as u8;
        Square::try_index(index).expect("unreachable")
    }

    #[inline]
    pub const fn to_bb(self) -> Bitboard {
        Bitboard(1u64 << self as u8)
    }

    #[inline]
    pub const fn rank(self) -> Rank {
        let index = (self as u8 & 0b0011_1000) >> 3;
        Rank::try_index(index).expect("unreachable")
    }

    #[inline]
    pub const fn file(self) -> File {
        let index = self as u8 & 0b0000_0111;
        File::try_index(index).expect("unreachable")
    }

    #[inline]
    pub const fn relative(self, colour: Colour) -> Square {
        let rank = self.rank().relative_to(colour);
        Square::new(rank, self.file())
    }

    #[inline]
    pub const fn try_index(index: u8) -> Option<Square> {
        if index > 0b0011_1111 {
            return None;
        }
        Some(Square::unchecked_index(index))
    }

    #[inline]
    pub const fn unchecked_index(index: u8) -> Square {
        unsafe { core::mem::transmute::<u8, Square>(index) }
    }

    #[inline]
    pub fn try_from_str(sqr: &str) -> Option<Square> {
        let mut chars = sqr.chars();
        let file = File::try_from_char(chars.next()?)?;
        let rank = Rank::try_from_char(chars.next()?)?;
        Some(Square::new(rank, file))
    }

    #[inline]
    pub fn relative_shift<D: Direction>(self, colour: Colour, amt: u8) -> Option<Square> {
        match colour {
            Colour::White => self.shift::<D>(amt),
            Colour::Black => self.shift::<D::Opposite>(amt),
        }
    }

    /*
    we *want* the wrap arounds/sign loss for the u8 guard here
    */
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn shift<D: Direction>(self, amt: u8) -> Option<Square> {
        let file = self.file() as i8 + D::DX * amt as i8;
        let rank = self.rank() as i8 + D::DY * amt as i8;
        if file as u8 & 0b1111_1000 != 0 {
            return None;
        }
        if rank as u8 & 0b1111_1000 != 0 {
            return None;
        }
        let index = rank << 3 | file;
        Square::try_index(index as u8)
    }

    pub fn parse(raw: &str) -> Option<Square> {
        let mut iter = raw.chars();
        let file = File::try_from_char(iter.next()?)?;
        let rank = Rank::try_from_char(iter.next()?)?;
        Some(Square::new(rank, file))
    }

    pub const ALL: [Square; 64] = {
        let mut sqrs = [Square::A1; 64];
        let mut i: u8 = 0;
        while i < 64 {
            sqrs[i as usize] = Square::unchecked_index(i);
            i += 1;
        }
        sqrs
    };
}

impl fmt::Display for Square {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[test]
fn rank_and_file() {
    for id in 0..64 {
        let square = Square::try_index(id).expect("id out of bounds");
        assert_eq!(square.rank() as u8, id / 8);
        assert_eq!(square.file() as u8, id % 8);
    }
}

#[test]
fn to_bb() {
    assert_eq!(Square::H8.to_bb().0, 0x8000000000000000);
    for id in 0..64 {
        let square = Square::try_index(id).unwrap();
        let bb = square.to_bb();
        assert_eq!(bb.0.trailing_zeros(), id as u32);
    }
}

#[test]
fn square_new() {
    let sqr1 = Square::new(Rank::Two, File::A);
    let sqr2 = Square::new(Rank::Eight, File::H);
    assert_eq!(sqr1.rank(), Rank::Two);
    assert_eq!(sqr1.file(), File::A);
    assert_eq!(sqr2.rank(), Rank::Eight);
    assert_eq!(sqr2.file(), File::H);
}

#[test]
fn shift() {
    use crate::common::direction::*;
    // North
    assert_eq!(Square::A1.shift::<North>(1), Some(Square::A2));
    assert_eq!(Square::A1.shift::<North>(7), Some(Square::A8));
    assert_eq!(Square::A8.shift::<North>(1), None);

    // South
    assert_eq!(Square::A8.shift::<South>(1), Some(Square::A7));
    assert_eq!(Square::A8.shift::<South>(7), Some(Square::A1));
    assert_eq!(Square::A1.shift::<South>(1), None);

    // East
    assert_eq!(Square::A1.shift::<East>(1), Some(Square::B1));
    assert_eq!(Square::A1.shift::<East>(7), Some(Square::H1));
    assert_eq!(Square::H1.shift::<East>(1), None);

    // West
    assert_eq!(Square::H1.shift::<West>(1), Some(Square::G1));
    assert_eq!(Square::H1.shift::<West>(7), Some(Square::A1));
    assert_eq!(Square::A1.shift::<West>(1), None);

    // NorthEast
    assert_eq!(Square::A1.shift::<NorthEast>(1), Some(Square::B2));
    assert_eq!(Square::A1.shift::<NorthEast>(7), Some(Square::H8));
    assert_eq!(Square::H1.shift::<NorthEast>(1), None);
    assert_eq!(Square::A8.shift::<NorthEast>(1), None);

    // NorthWest
    assert_eq!(Square::H1.shift::<NorthWest>(1), Some(Square::G2));
    assert_eq!(Square::H1.shift::<NorthWest>(7), Some(Square::A8));
    assert_eq!(Square::A1.shift::<NorthWest>(1), None);
    assert_eq!(Square::H8.shift::<NorthWest>(1), None);

    // SouthEast
    assert_eq!(Square::A8.shift::<SouthEast>(1), Some(Square::B7));
    assert_eq!(Square::A8.shift::<SouthEast>(7), Some(Square::H1));
    assert_eq!(Square::H8.shift::<SouthEast>(1), None);
    assert_eq!(Square::A1.shift::<SouthEast>(1), None);

    // SouthWest
    assert_eq!(Square::H8.shift::<SouthWest>(1), Some(Square::G7));
    assert_eq!(Square::H8.shift::<SouthWest>(7), Some(Square::A1));
    assert_eq!(Square::A8.shift::<SouthWest>(1), None);
    assert_eq!(Square::H1.shift::<SouthWest>(1), None);

    // Zero distance
    assert_eq!(Square::E4.shift::<North>(0), Some(Square::E4));
    assert_eq!(Square::E4.shift::<NorthEast>(0), Some(Square::E4));
    assert_eq!(Square::E4.shift::<SouthWest>(0), Some(Square::E4));
}
