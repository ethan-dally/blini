use std::{
    fmt::Display,
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Not,
    },
};

use crate::common::{
    colour::Colour,
    direction::{
        Direction,
        shift_mask,
    },
    file::File,
    rank::Rank,
    square::Square,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn try_next(self) -> Option<Square> {
        Square::try_index(self.0.trailing_zeros() as u8)
    }

    #[inline]
    pub const fn iter(self) -> BitboardIter {
        BitboardIter(self)
    }

    #[inline]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub const fn relative_shift<D: Direction>(self, colour: Colour, amt: u8) -> Bitboard {
        match colour {
            Colour::White => self.shift::<D>(amt),
            Colour::Black => self.shift::<D::Opposite>(amt),
        }
    }

    #[inline]
    pub const fn shift<D: Direction>(self, amt: u8) -> Bitboard {
        if amt > 7 {
            return Bitboard::EMPTY;
        }

        // SAFETY: empty output when outside of -7, 7
        #[allow(clippy::cast_possible_wrap)]
        let index = D::DX * amt as i8 + D::DY * 8 * amt as i8;
        Bitboard(match index.is_positive() {
            true => (self.0 & shift_mask::<D>(amt)) << index,
            false => (self.0 & shift_mask::<D>(amt)) >> index.abs(),
        })
    }

    #[inline]
    pub const fn has(&self, sqr: Square) -> bool {
        sqr.to_bb().0 & self.0 != 0
    }

    #[inline]
    pub const fn piece_count(self) -> u32 {
        self.0.count_ones()
    }

    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for rank in Rank::ALL.iter().rev() {
            for file in File::ALL {
                let sqr = Square::new(*rank, file);
                write!(
                    f,
                    "{}",
                    match self.has(sqr) {
                        false => " ·",
                        true => " X",
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f)
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
fn shift() {
    use crate::common::direction::{
        East,
        North,
        NorthEast,
        South,
        West,
    };
    //dirs
    assert_eq!(Square::A1.to_bb().shift::<East>(1), Square::B1.to_bb());
    assert_eq!(Square::C1.to_bb().shift::<West>(2), Square::A1.to_bb());
    assert_eq!(Square::A2.to_bb().shift::<South>(1), Square::A1.to_bb());
    assert_eq!(Square::A1.to_bb().shift::<North>(2), Square::A3.to_bb());

    //diagonals
    assert_eq!(Square::A1.to_bb().shift::<NorthEast>(1), Square::B2.to_bb());
}

#[test]
fn bitboard_iter() {
    use crate::common::file::File;
    let mut iter1 = Square::A1.to_bb().iter();
    assert_eq!(iter1.next(), Some(Square::A1));
    assert_eq!(iter1.next(), None);
    let file_list: Vec<_> = File::C.to_bb().iter().collect();
    assert_eq!(
        file_list,
        [
            Square::C1,
            Square::C2,
            Square::C3,
            Square::C4,
            Square::C5,
            Square::C6,
            Square::C7,
            Square::C8
        ]
    );
}

#[test]
fn bitboard_has() {
    let sqr = Square::F3;
    assert!(File::F.to_bb().has(sqr));
    assert!(!File::H.to_bb().has(sqr));
    assert!(Rank::Three.to_bb().has(sqr));
    assert!(!Rank::Seven.to_bb().has(sqr));
}
