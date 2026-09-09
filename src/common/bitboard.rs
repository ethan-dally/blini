use crate::common::square::Square;

pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline]
    pub const fn try_next(self) -> Option<Square> {
        Square::try_index(self.0.trailing_zeros() as u8)
    }
}

