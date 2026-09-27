use std::{
    fmt,
    ops::Not,
};

use enum_map::Enum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum, Default)]
pub enum Colour {
    Black,
    #[default]
    White,
}

impl Colour {
    #[inline]
    pub const fn try_from_char(c: char) -> Option<Colour> {
        match c {
            'w' => Some(Colour::White),
            'b' => Some(Colour::Black),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_bool(self) -> bool {
        (self as usize) == 0b1
    }
}

impl From<bool> for Colour {
    fn from(value: bool) -> Self {
        if value { Colour::White } else { Colour::Black }
    }
}

impl Not for Colour {
    type Output = Colour;
    fn not(self) -> Self::Output {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black,
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colour::White => "White",
                Colour::Black => "Black",
            }
        )
    }
}

#[test]
fn colour() {
    assert_eq!(Colour::Black, Colour::from(false));
    assert_eq!(Colour::White, Colour::from(true));
}

#[test]
fn to_bool() {
    assert!(Colour::White.to_bool());
    assert!(!Colour::Black.to_bool());
}
