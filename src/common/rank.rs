use std::fmt;

use crate::common::bitboard::Bitboard;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    #[inline]
    pub const fn try_index(index: u8) -> Option<Rank> {
        if index > 0b0000_0111 {
            return None;
        }
        Some(unsafe { core::mem::transmute::<u8, Rank>(index) })
    }

    #[inline]
    pub const fn to_bb(self) -> Bitboard {
        Bitboard(0b11111111 << (8 * self as u8))
    }

    #[inline]
    pub const fn try_from_char(c: char) -> Option<Rank> {
        match c {
            '1' => Some(Rank::One),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            _ => None,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = *self as u8 + 1;
        write!(f, "{num}")
    }
}

#[test]
fn rank_try_index() {
    assert_eq!(Rank::try_index(0b0000_0000), Some(Rank::One));
    assert_eq!(Rank::try_index(0b0000_0001), Some(Rank::Two));
    assert_eq!(Rank::try_index(0b0000_1000), None);
}

#[test]
fn rank_to_bb() {
    assert_eq!(Rank::One.to_bb(), Bitboard(0x0000_0000_0000_00FF));
    assert_eq!(Rank::Two.to_bb(), Bitboard(0x0000_0000_0000_FF00));
    assert_eq!(Rank::Three.to_bb(), Bitboard(0x0000_0000_00FF_0000));
    assert_eq!(Rank::Eight.to_bb(), Bitboard(0xFF00_0000_0000_0000));
}
