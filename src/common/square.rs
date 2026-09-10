use std::fmt;
use enum_map::Enum;
use crate::common::bitboard::Bitboard;
use crate::common::file::File;
use crate::common::rank::Rank;

#[repr(u8)] 
#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {

    #[inline]
    pub const fn new(rank: Rank, file: File) -> Square {
        let index = file as u8 * 8 + rank as u8;
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
    pub const fn try_index(index: u8) -> Option<Square> {
        if index > 0b00111_111 {return None;}
        Some(unsafe { core::mem::transmute::<u8, Square>(index) })
    }
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
    for id in 0..64 {
        let square = Square::try_index(id).unwrap();
        let bb = square.to_bb();
        assert_eq!(bb.0.trailing_zeros(), id as u32);
    }
}