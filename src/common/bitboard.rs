use std::{ops::BitXorAssign};
use crate::common::{file::File, square::Square};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline]
    pub const fn try_next(self) -> Option<Square> {
        Square::try_index(self.0.trailing_zeros() as u8)
    }
    #[inline]
    pub const fn iter(self) -> BitboardIter {
        BitboardIter(self)
    }
}

impl BitXorAssign for Bitboard{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

pub struct BitboardIter(Bitboard);

impl Iterator for BitboardIter {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        let square = self.0.try_next();
        if let Some(square) = square {
            self.0 ^= square.to_bb();
        }
        square
    }
}

#[test]
fn bitboard_iter() {
    let mut iter1 = Square::A1.to_bb().iter();
    assert_eq!(iter1.next(), Some(Square::A1));
    assert_eq!(iter1.next(), None);
    let file_list: Vec<_> = File::C.to_bb().iter().collect();
    assert_eq!(file_list, [
        Square::C1, Square::C2, Square::C3, Square::C4, 
        Square::C5, Square::C6, Square::C7, Square::C8
    ]);
}