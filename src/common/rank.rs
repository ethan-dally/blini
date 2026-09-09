use std::fmt;

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
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[test]
fn try_index() {
    assert_eq!(Rank::try_index(0), Some(Rank::One));
    assert_eq!(Rank::try_index(8), None);
}