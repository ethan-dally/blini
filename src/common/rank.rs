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
    Eight
} 

impl Rank {
    #[inline]
    pub const fn try_index(index: u8) -> Option<Rank> {
        if index > 0b0000_0111 {return None;}
        Some(unsafe { core::mem::transmute::<u8, Rank>(index) })
    }

    #[inline]
    pub const fn to_bb(self) -> Bitboard {
        Bitboard(0b11111111 << (8 * self as u8))
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
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