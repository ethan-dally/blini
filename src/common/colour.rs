use enum_map::Enum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum Colour {
    Black,
    White,
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