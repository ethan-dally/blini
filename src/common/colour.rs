use enum_map::Enum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum Colour {
    Black,
    White,
}

impl Default for Colour {
    fn default() -> Self {
        Colour::White
    }
}

impl Colour {
    #[inline]
    pub const fn try_from_char(c: char) -> Option<Colour> {
        match c {
            'w' => Some(Colour::White),
            'b' => Some(Colour::Black),
            _ => None
        }
    }
}

impl From<bool> for Colour {
    fn from(value: bool) -> Self {
        match value {
            true => Colour::White,
            false => Colour::Black
        }
    }
}

#[test]
fn colour() {
    assert_eq!(Colour::Black, Colour::from(false));
    assert_eq!(Colour::White, Colour::from(true));
}